/**
 * Advent of code solutions
 * by Samuel Cristobal github.com/scristobal/advent-of-code
 * Licensed under MIT
 */
use advent_of_code::solve_part1;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../input.txt");
    Ok(println!("{}", solve_part1(input)?))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT_1: &str = include_str!("../../sample-part-1.txt");
    const ANSWER_1: &str = "8";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT_1).unwrap();
        assert_eq!(result, ANSWER_1)
    }
}
