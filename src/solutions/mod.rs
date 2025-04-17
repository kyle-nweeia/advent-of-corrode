use axum::http::StatusCode;

use crate::endpoints::submission::Part;

mod year_2024;

pub fn get_solution(year: u32, day: u32, part: Part) -> Result<fn(String) -> f64, StatusCode> {
    match year {
        2024 => Ok(year_2024::get_solution(day, part)?),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
