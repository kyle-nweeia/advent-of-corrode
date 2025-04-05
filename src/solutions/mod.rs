use axum::http::StatusCode;

mod year_2024;

pub fn get_solution(year: u32, day: u32) -> Result<fn(String) -> String, StatusCode> {
    match year {
        2024 => Ok(year_2024::get_solution(day)?),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
