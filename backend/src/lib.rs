use axum::Router;

mod endpoints;
pub mod models;
pub mod schema;
pub mod solutions;
pub mod utils;

pub fn router() -> Router {
    Router::new()
        .merge(endpoints::session::router())
        .merge(endpoints::submission::router())
        .layer(tower_http::cors::CorsLayer::very_permissive())
}
