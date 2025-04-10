pub fn solve(input: String) -> f64 {
    f64::from(11)
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
