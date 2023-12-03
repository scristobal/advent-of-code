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

    const INPUT: &str = include_str!("../../sample-part-1.txt");
    const ANSWER: &str = "4361";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT).unwrap();
        assert!(!INPUT.is_empty());
        assert!(!ANSWER.is_empty());
        assert_eq!(result, ANSWER);
    }
}
