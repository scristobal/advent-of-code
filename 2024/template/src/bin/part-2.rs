use std::fs;

use advent_of_code::part_2;
use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../../input.txt")?;

    println!("Part 2: {}", part_2::solve(&input)?);
    Ok(())
}
