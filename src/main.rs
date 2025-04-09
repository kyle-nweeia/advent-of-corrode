use advent_of_code::{handler, session_cookie_handler};
use axum::routing;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    axum::serve(
        tokio::net::TcpListener::bind("localhost:3000").await?,
        axum::Router::new()
            .route("/{year}/{day}", routing::get(handler))
            .route("/session-cookie", routing::post(session_cookie_handler)),
    )
    .await
}
