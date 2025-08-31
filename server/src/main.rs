#[cfg(debug_assertions)]
mod am_auth_flow;
mod index;
mod scrapers;

use std::{net::SocketAddr, sync::Arc};

#[cfg(debug_assertions)]
use crate::am_auth_flow::AuthFlowTemplate;
use crate::index::RootTemplate;
use crate::scrapers::apple_music::AppleMusicClient;

use askama::Template;
use axum::{
    Router,
    extract::State,
    http::{HeaderName, HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, get_service},
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, services::ServeDir, set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

#[derive(Clone)]
struct AppState {
    apple_music_client: Arc<AppleMusicClient>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let apple_music_client = Arc::new(AppleMusicClient::new()?);
    let state = AppState { apple_music_client };

    let app = Router::new()
        .route("/", get(render_index_handler))
        .route("/dev/am-auth-flow", get(render_apple_music_auth_flow))
        .fallback(get_service(ServeDir::new(".")))
        .with_state(state)
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

async fn render_index_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = RootTemplate::new(&state.apple_music_client).await;
    template.render().map(Html).map_err(|err| {
        tracing::error!("failed to render index: {err:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn render_apple_music_auth_flow(State(state): State<AppState>) -> impl IntoResponse {
    #[cfg(not(debug_assertions))]
    return StatusCode::NOT_FOUND;

    #[cfg(debug_assertions)]
    {
        let template = AuthFlowTemplate::new(&state.apple_music_client);
        template
            .and_then(|template| Ok(template.render()?))
            .map(Html)
            .map_err(|err| {
                tracing::error!("failed to render Apple Music auth flow: {err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })
    }
}
