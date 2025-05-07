use axum::http::StatusCode;

use crate::endpoints::solution::Part;

use super::Solver;

mod day_1;

pub fn get_solution(day: u32, part: Part) -> Result<Solver, StatusCode> {
    match day {
        1 => Ok(day_1::solve(part)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
