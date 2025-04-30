use axum::{Json, Router, http::StatusCode};
use diesel::RunQueryDsl;

#[derive(serde::Deserialize)]
struct Body {
    username: String,
    value: String,
}

async fn handler(Json(Body { username, value }): Json<Body>) -> Result<StatusCode, StatusCode> {
    diesel::insert_into(crate::schema::session_cookies::table)
        .values(&crate::models::SessionCookie {
            username: &username,
            val: &value,
        })
        .execute(&mut crate::utils::establish_database_connection()?)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub fn router() -> Router {
    Router::new().route("/session", axum::routing::post(handler))
}
