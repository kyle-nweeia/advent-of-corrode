use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    axum::serve(
        TcpListener::bind("localhost:3000").await.unwrap(),
        axum::Router::new().route("/", axum::routing::get(async || "Hello, world!")),
    )
    .await
    .unwrap();
}
