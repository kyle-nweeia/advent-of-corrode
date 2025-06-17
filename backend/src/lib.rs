use axum::Router;

mod endpoints;
pub mod models;
pub mod schema;
pub mod solutions;
pub mod utils;

#[derive(serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum Part {
    One = 1,
    Two,
}

pub fn router() -> Router {
    Router::new()
        .merge(endpoints::session::router())
        .merge(endpoints::solution::router())
        .layer(tower_http::cors::CorsLayer::very_permissive())
}
