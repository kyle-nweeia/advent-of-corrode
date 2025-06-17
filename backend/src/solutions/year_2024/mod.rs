use axum::http::StatusCode;

use super::Solver;

mod day_1;
mod day_2;

pub fn get_solution(day: u32, part: crate::Part) -> Result<Solver, StatusCode> {
    match day {
        1 => Ok(day_1::solve(part)),
        2 => Ok(day_2::solve(part)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
