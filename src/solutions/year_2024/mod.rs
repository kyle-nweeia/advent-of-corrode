use axum::http::StatusCode;

mod day_1;

pub fn get_solution(day: u32) -> Result<fn(String) -> String, StatusCode> {
    match day {
        1 => Ok(day_1::solve),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
