use advent_of_code::part_1;
use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    println!("Part 1: {}", part_1::solve(input)?);
    Ok(())
}
