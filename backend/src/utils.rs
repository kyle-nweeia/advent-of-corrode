use axum::http::StatusCode;
use diesel::{Connection, PgConnection};

pub fn establish_database_connection() -> Result<PgConnection, StatusCode> {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    PgConnection::establish(&database_url).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn parse_columns(input: &String) -> Vec<Vec<u32>> {
    transpose(parse_rows(input))
}

pub fn parse_rows(input: &String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = v[0].len();
    let mut rows: Vec<_> = v.into_iter().map(|row| row.into_iter()).collect();

    (0..n)
        .map(|_| rows.iter_mut().map(|row| row.next().unwrap()).collect())
        .collect()
}
