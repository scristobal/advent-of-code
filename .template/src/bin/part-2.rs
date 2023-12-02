/**
 * Advent of code solutions
 * by Samuel Cristobal github.com/scristobal/advent-of-code
 * Licensed under MIT
 */
use advent_of_code::solve_part2;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../input.txt");
    Ok(println!("{}", solve_part2(input)?))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = include_str!("../../sample-part-2.txt");
    const ANSWER: &str = "";

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT).unwrap();
        assert!(!INPUT.is_empty(), "is sample-part1.txt empty?");
        assert!(!ANSWER.is_empty(), "provide a sample answer");
        assert_eq!(result, ANSWER);
    }
}
