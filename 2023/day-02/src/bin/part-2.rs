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

    const INPUT_2: &str = include_str!("../../sample-part-2.txt");
    const ANSWER_2: &str = "2286";

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT_2).unwrap();
        assert_eq!(result, ANSWER_2);
    }
}
