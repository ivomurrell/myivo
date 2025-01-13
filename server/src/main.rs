mod index;
mod scrobble;
mod scrobble_monitor;

use std::{env, net::SocketAddr};

use crate::index::get_index;
use crate::scrobble_monitor::ScrobbleMonitor;

use askama::Template;
use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Extension, Router,
};
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
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(Extension(monitor))
                .layer(SetResponseHeaderLayer::overriding(
                    HeaderName::from_static("strict-transport-security"),
                    HeaderValue::from_static("max-age=300; includeSubDomains"),
                )),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("starting server on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn render_index_handler(
    Extension(mut monitor): Extension<ScrobbleMonitor>,
) -> impl IntoResponse {
    let template = get_index(&mut monitor).await.map_err(|err| {
        tracing::error!("failed to get data from last.fm: {err:?}");
        StatusCode::BAD_GATEWAY
    })?;
    template.render().map(Html).map_err(|err| {
        tracing::error!("failed to render index: {err:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn get_scrobble(Extension(mut monitor): Extension<ScrobbleMonitor>) -> impl IntoResponse {
    let template = monitor.get_scrobble().await.map_err(|err| {
        tracing::error!("failed to get data from last.fm: {err:?}");
        StatusCode::BAD_GATEWAY
    })?;
    template.render().map(Html).map_err(|err| {
        tracing::error!("failed to render scrobble: {err:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
