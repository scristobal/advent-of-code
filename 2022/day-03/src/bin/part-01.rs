use day_03::solve_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("solution to part 1 is {}", solve_part1(&file));
}
