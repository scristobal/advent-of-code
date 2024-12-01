use advent_of_code::part_1;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../../input.txt")?;

    println!("Part 1: {}", part_1::solve(&input)?);
    Ok(())
}
