use axum::http::StatusCode;

use crate::endpoints::submission::Part;

mod day_1;

pub fn get_solution(day: u32, part: Part) -> Result<fn(String) -> f64, StatusCode> {
    match day {
        1 => Ok(day_1::solve(part)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
