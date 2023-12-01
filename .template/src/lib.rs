/**
 * Advent of code solutions
 * by Samuel Cristobal github.com/scristobal/advent-of-code
 * Licensed under MIT
 */

pub fn solve_part1(input: &str) -> String {
    dbg!(input);
    todo!()
}

pub fn solve_part2(input: &str) -> String {
    dbg!(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = include_str!("../sample-part-1.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT_1);
        assert_eq!(result, "");
    }

    const INPUT_2: &str = include_str!("../sample-part-2.txt");

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT_2);
        assert_eq!(result, "");
    }
}
