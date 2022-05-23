mod scrobble_monitor;

use std::{env, io, net::SocketAddr};

use crate::scrobble_monitor::ScrobbleMonitor;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Router,
};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let monitor = ScrobbleMonitor::new(env::var("LAST_FM_API_KEY")?);

    let app = Router::new()
        .route("/scrobbles.json", get(get_scrobble))
        .fallback(get_service(ServeDir::new(".")).handle_error(handle_serve_error))
        .layer(Extension(monitor))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8010));
    tracing::debug!("starting server on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handle_serve_error(error: io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {error}"),
    )
}

async fn get_scrobble(Extension(mut monitor): Extension<ScrobbleMonitor>) -> impl IntoResponse {
    monitor.get_scrobble().await.map_err(|err| {
        tracing::error!("failed to get data from last.fm: {err}");
        StatusCode::BAD_GATEWAY
    })
}
