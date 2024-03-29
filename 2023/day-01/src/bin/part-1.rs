use advent_of_code::solve_part1;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("./input.txt")?;
    Ok(println!("{}", solve_part1(&file)?))
}
