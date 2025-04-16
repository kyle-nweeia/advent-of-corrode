use axum::http::StatusCode;

mod day_1;

pub fn get_solution(day: u32) -> Result<fn(String) -> f64, StatusCode> {
    match day {
        1 => Ok(day_1::solve_part_1),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
