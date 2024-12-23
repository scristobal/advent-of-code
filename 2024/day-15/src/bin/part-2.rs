use advent_of_code::part_2;
use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    println!("Part 2: {}", part_2::solve(input)?);
    Ok(())
}
