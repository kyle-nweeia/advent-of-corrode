#[tokio::main]
async fn main() -> std::io::Result<()> {
    axum::serve(
        tokio::net::TcpListener::bind("localhost:3000").await?,
        advent_of_corrode::router(),
    )
    .await
}
