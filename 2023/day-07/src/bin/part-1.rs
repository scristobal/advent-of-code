/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use advent_of_code::part_1::solve_part1;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../input.txt");
    Ok(println!("{}", solve_part1(input)?))
}
