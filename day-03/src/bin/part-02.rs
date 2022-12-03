use day_03::solve_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("solution to part 2 is {}", solve_part2(&file));
}
