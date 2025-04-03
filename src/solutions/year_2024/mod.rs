mod day_1;

pub fn get_solution(day: u32) -> fn() -> &'static str {
    match day {
        1 => day_1::solve,
        _ => || "Not Found",
    }
}
