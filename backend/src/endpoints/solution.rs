use axum::{Router, extract::Path, http::StatusCode};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::fs;
use types::Solution;

use crate::schema::session_cookies::dsl;

async fn request_puzzle_input(year: u32, day: u32) -> Result<String, StatusCode> {
    dotenvy::dotenv().ok();

    let session =
        dsl::session_cookies
            .filter(dsl::username.eq(
                &std::env::var("CURRENT_USER").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            ))
            .select(dsl::val)
            .first::<String>(&mut crate::utils::establish_database_connection()?)
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

async fn handler(
    Path(Solution { year, day, part }): Path<Solution<u32, u32, crate::Part>>,
) -> Result<String, StatusCode> {
    let filename = format!("input_{year}_{day}.txt");

    Ok(crate::solutions::get_solution(year, day, part)?(
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

pub fn router() -> Router {
    Router::new().route("/solution/{year}/{day}/{part}", axum::routing::get(handler))
}
