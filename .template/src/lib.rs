/**
 * Advent of code solutions
 * by Samuel Cristobal
 * https://github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023
 */

pub fn solve_part1(input: &str) -> Result<String, anyhow::Error> {
    dbg!(input);
    todo!()
}

pub fn solve_part2(input: &str) -> Result<String, anyhow::Error> {
    dbg!(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = include_str!("../sample-part-1.txt");
    const ANSWER_1: &str = "";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT_1).unwrap();
        assert_eq!(result, ANSWER_1)
    }

    const INPUT_2: &str = include_str!("../sample-part-2.txt");
    const ANSWER_2: &str = "";

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT_2).unwrap();
        assert_eq!(result, ANSWER_2);
    }
}
