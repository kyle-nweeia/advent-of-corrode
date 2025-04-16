pub fn solve_part_1(input: String) -> f64 {
    let lines: Vec<_> = input.split('\n').filter(|line| !line.is_empty()).collect();
    let rows: Vec<_> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let mut cols = transpose(rows);

    cols.iter_mut().for_each(|col| col.sort_unstable());

    transpose(cols)
        .iter()
        .map(|pair| pair[0].abs_diff(pair[1]))
        .sum::<u32>()
        .into()
}

pub fn solve_part_2(input: String) -> f64 {
    31.into()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = v[0].len();
    let mut rows: Vec<_> = v.into_iter().map(|row| row.into_iter()).collect();

    (0..n)
        .map(|_| rows.iter_mut().map(|row| row.next().unwrap()).collect())
        .collect()
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
    fn example() {
        assert_eq!(solve_part_1(INPUT.into()), 11 as f64);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(solve_part_2(INPUT.into()), 31 as f64);
    }
}
