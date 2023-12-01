use advent_of_code::solve_part2;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("./input.txt")?;
    println!("{}", solve_part2(&file)?);
    Ok(())
}
