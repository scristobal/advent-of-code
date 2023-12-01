use advent_of_code_2022::solve_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solve_part1(&file, 2022));
}
