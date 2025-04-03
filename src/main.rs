use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    axum::serve(
        TcpListener::bind("localhost:3000").await.unwrap(),
        axum::Router::new().route("/{year}/{day}", axum::routing::get(advent_of_code::handler)),
    )
    .await
    .unwrap();
}
