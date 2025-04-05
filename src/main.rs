#[tokio::main]
async fn main() -> std::io::Result<()> {
    axum::serve(
        tokio::net::TcpListener::bind("localhost:3000").await?,
        axum::Router::new().route("/{year}/{day}", axum::routing::get(advent_of_code::handler)),
    )
    .await
}
