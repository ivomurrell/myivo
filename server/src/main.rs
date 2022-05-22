use std::io;

use axum::{http::StatusCode, response::IntoResponse, routing::get_service};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = get_service(ServeDir::new(".")).handle_error(handle_serve_error);

    axum::Server::bind(&"0.0.0.0:8010".parse()?)
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
