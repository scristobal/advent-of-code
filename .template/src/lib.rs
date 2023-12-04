/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

pub fn solve_part1(input: &'static str) -> Result<String, anyhow::Error> {
    dbg!(input);
    todo!()
}

pub fn solve_part2(input: &'static str) -> Result<String, anyhow::Error> {
    dbg!(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[ignore = "not implemented"]
    #[test]
    fn part1_works() {
        let result = solve_part1(SAMPLE).unwrap();
        assert_eq!(result, "");
    }

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(SAMPLE).unwrap();
        assert_eq!(result, "");
    }
}
