use advent_of_code::{session_cookie_handler, submission_handler};
use axum::routing;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    axum::serve(
        tokio::net::TcpListener::bind("localhost:3000").await?,
        axum::Router::new()
            .route("/submit/{year}/{day}", routing::get(submission_handler))
            .route("/session-cookie", routing::post(session_cookie_handler)),
    )
    .await
}
