use crate::Part;

pub fn solve(part: Part) -> super::Solver {
    match part {
        Part::One => solve_part_1::<String>,
        Part::Two => solve_part_2::<String>,
    }
}

pub fn solve_part_1<T>(input: T) -> f64
where
    String: From<T>,
{
    todo!();
}

pub fn solve_part_2<T>(input: T) -> f64
where
    String: From<T>,
{
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn part_1_example() {}

    #[test]
    fn part_2_example() {}
}
