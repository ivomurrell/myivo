mod index;
mod scrobble;
mod scrobble_monitor;

use std::{convert::Infallible, env, net::SocketAddr, time::Duration};

use crate::index::RootTemplate;
use crate::scrobble_monitor::ScrobbleMonitor;

use askama::Template;
use async_stream::stream;
use axum::{
    Router,
    extract::{Query, State},
    http::{HeaderName, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Sse, sse},
    routing::{get, get_service},
};
use serde::Deserialize;
use tokio::time::{self, MissedTickBehavior};
use tokio_stream::Stream;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let monitor = ScrobbleMonitor::new(env::var("LAST_FM_API_KEY")?);

    let app = Router::new()
        .route("/", get(render_index_handler))
        .route("/scrobbles", get(get_scrobble))
        .fallback(get_service(ServeDir::new(".")))
        .with_state(monitor)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(SetResponseHeaderLayer::overriding(
                    HeaderName::from_static("strict-transport-security"),
                    HeaderValue::from_static("max-age=2592000; includeSubDomains"),
                )),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("starting server on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn render_index_handler(State(monitor): State<ScrobbleMonitor>) -> impl IntoResponse {
    let template = RootTemplate::new(monitor);
    template.render().map(Html).map_err(|err| {
        tracing::error!("failed to render index: {err:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(Deserialize)]
struct ScrobbleQuery {
    #[serde(default)]
    immediate: bool,
}

async fn get_scrobble(
    State(monitor): State<ScrobbleMonitor>,
    Query(ScrobbleQuery { immediate }): Query<ScrobbleQuery>,
) -> Sse<impl Stream<Item = Result<sse::Event, Infallible>>> {
    let stream = stream! {
        let mut last_template = None;

        let mut interval = time::interval(Duration::from_secs(30));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        if !immediate {
            interval.tick().await;
            last_template = monitor.get_scrobble().await.ok();
        }

        loop {
            interval.tick().await;
            let new_template = match monitor.get_scrobble().await {
                Ok(template) => template,
                Err(error) => {
                    tracing::error!(?error, "failed to get data from last.fm");
                    continue;
                }
            };

            if last_template
                .as_ref()
                .is_some_and(|last_template| last_template == &new_template)
            {
                continue;
            }

            let data = match new_template.render() {
                Ok(data) => data,
                Err(error) => {
                    tracing::error!(?error, "failed to render scrobble");
                    break;
                }
            };
            yield Ok(sse::Event::default().event("scrobble").data(data));

            last_template.replace(new_template);
        }
    };

    Sse::new(stream)
}
