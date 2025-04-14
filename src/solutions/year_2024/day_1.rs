pub fn solve(input: String) -> f64 {
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

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = v[0].len();
    let mut rows: Vec<_> = v.into_iter().map(|row| row.into_iter()).collect();

    (0..n)
        .map(|_| rows.iter_mut().map(|row| row.next().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn example() {
        let answer = 11;
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};

        assert_eq!(solve(input.to_string()), f64::from(answer));
    }
}
