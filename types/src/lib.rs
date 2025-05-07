#[derive(serde::Deserialize)]
pub struct Solution<T, U, V> {
    pub year: T,
    pub day: U,
    pub part: V,
}

impl<'a> Solution<&'a str, &'a str, &'a str> {
    pub fn from(
        vals: &'a std::collections::HashMap<String, impl std::ops::Deref<Target = [String]>>,
    ) -> Option<Solution<&'a str, &'a str, &'a str>> {
        Some(Solution {
            year: vals.get("year")?.first()?.as_str(),
            day: vals.get("day")?.first()?.as_str(),
            part: vals.get("part")?.first()?.as_str(),
        })
    }
}
