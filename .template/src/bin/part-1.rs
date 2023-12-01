/**
 * Advent of code solutions
 * by Samuel Cristobal github.com/scristobal/advent-of-code
 * Licensed under MIT
 */
use advent_of_code::solve_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solve_part1(&file));
}
