use axum::http::StatusCode;
use diesel::{Connection, PgConnection};

pub fn establish_database_connection() -> Result<PgConnection, StatusCode> {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    PgConnection::establish(&database_url).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
