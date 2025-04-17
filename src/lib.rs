use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::Deserialize;
use std::{env::var, fs};

mod models;
mod schema;
mod solutions;

#[derive(Deserialize)]
pub struct Params {
    year: u32,
    day: u32,
    part: Part,
}

#[derive(serde_repr::Deserialize_repr)]
#[repr(u8)]
enum Part {
    One = 1,
    Two,
}

#[derive(Deserialize)]
pub struct SessionCookieBody {
    username: String,
    val: String,
}

fn establish_connection() -> Result<PgConnection, StatusCode> {
    dotenv().ok();

    let database_url = var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    PgConnection::establish(&database_url).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn request_puzzle_input(year: u32, day: u32) -> Result<String, StatusCode> {
    use schema::session_cookies::dsl;

    dotenv().ok();

    let session = dsl::session_cookies
        .filter(
            dsl::username.eq(&var("CURRENT_USER").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        )
        .select(dsl::val)
        .first::<String>(&mut establish_connection()?)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    reqwest::ClientBuilder::new()
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(reqwest::header::COOKIE, format!("session={session}"))
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .text()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)
}

pub async fn submission_handler(
    Path(Params { year, day, part }): Path<Params>,
) -> Result<String, StatusCode> {
    let filename = format!("input_{year}_{day}.txt");

    Ok(solutions::get_solution(year, day, part)?(
        if let Ok(local) = fs::read_to_string(&filename) {
            local
        } else {
            let remote = request_puzzle_input(year, day).await?;

            fs::write(&filename, &remote)
                .or_else(|_| fs::remove_file(&filename))
                .ok();

            remote
        },
    )
    .to_string())
}

pub async fn session_cookie_handler(
    Json(SessionCookieBody { username, val }): Json<SessionCookieBody>,
) -> Result<StatusCode, StatusCode> {
    diesel::insert_into(schema::session_cookies::table)
        .values(&models::SessionCookie {
            username: &username,
            val: &val,
        })
        .execute(&mut establish_connection()?)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
