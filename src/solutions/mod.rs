mod year_2024;

pub fn get_solution(year: u32, day: u32) -> fn() -> &'static str {
    match year {
        2024 => year_2024::get_solution(day),
        _ => || "Not Found",
    }
}
