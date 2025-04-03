use axum::extract::Path;

mod solutions;

#[derive(serde::Deserialize)]
pub struct Params {
    year: u32,
    day: u32,
}

pub async fn handler(Path(Params { year, day }): Path<Params>) -> &'static str {
    solutions::get_solution(year, day)()
}
