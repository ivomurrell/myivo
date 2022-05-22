use std::{io, net::SocketAddr};

use axum::{http::StatusCode, response::IntoResponse, routing::get_service};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = get_service(ServeDir::new("."))
        .handle_error(handle_serve_error)
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
