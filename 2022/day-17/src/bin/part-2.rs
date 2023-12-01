use advent_of_code_2022::solve_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solve_part2(&file, 1000000000000));
}
