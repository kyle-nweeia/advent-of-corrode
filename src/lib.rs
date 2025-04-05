use axum::{extract::Path, http::StatusCode};

mod solutions;

#[derive(serde::Deserialize)]
pub struct Params {
    year: u32,
    day: u32,
}

async fn request_puzzle_input(year: u32, day: u32) -> Result<String, reqwest::Error> {
    let session = std::env::var("SESSION").unwrap_or_default();

    reqwest::Client::new()
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(reqwest::header::COOKIE, format!("session={session}"))
        .send()
        .await?
        .text()
        .await
}

pub async fn handler(Path(Params { year, day }): Path<Params>) -> Result<String, StatusCode> {
    let filename = format!("input_{year}_{day}.txt");

    Ok(solutions::get_solution(year, day)?(
        if let Ok(input) = std::fs::read_to_string(&filename) {
            input
        } else {
            let response = request_puzzle_input(year, day)
                .await
                .map_err(|_| StatusCode::BAD_GATEWAY)?;

            if std::fs::write(&filename, &response).is_err() {
                let _ = std::fs::remove_file(&filename);
            }

            response
        },
    ))
}
