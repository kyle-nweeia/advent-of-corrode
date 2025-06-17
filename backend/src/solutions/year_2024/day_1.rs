use std::collections::HashMap;

use crate::{Part, utils::parse_columns};

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
    let mut cols = parse_columns(&input.into());

    cols.iter_mut().for_each(|col| col.sort_unstable());

    crate::utils::transpose(cols)
        .iter()
        .map(|pair| pair[0].abs_diff(pair[1]))
        .sum::<u32>()
        .into()
}

pub fn solve_part_2<T>(input: T) -> f64
where
    String: From<T>,
{
    let cols = parse_columns(&input.into());
    let cnts = cols[1].iter().fold(HashMap::new(), |mut cnts, num| {
        cnts.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);
        cnts
    });

    cols[0]
        .iter()
        .map(|num| num * cnts.get(num).unwrap_or(&0))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn part_1_example() {
        assert_eq!(solve_part_1(INPUT), 11.0);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(solve_part_2(INPUT), 31.0);
    }
}
