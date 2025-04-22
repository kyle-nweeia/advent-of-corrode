use axum::routing::Router;

pub fn router() -> Router {
    Router::new().fallback_service(tower_http::services::ServeDir::new("assets"))
}
