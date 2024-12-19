use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &'static str) -> Result<String> {
    let width = input.lines().next().unwrap().chars().count() as i32;

    let height = input
        .lines()
        .filter(|line| line.chars().count() == width as usize)
        .count() as i32;

    // a single lowercase letter, uppercase letter, or digit.
    let mut data = HashMap::with_capacity(2 * 28 + 10);

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;

            if char != '.' {
                data.entry(char)
                    .and_modify(|e: &mut Vec<(i32, i32)>| e.push((x, y)))
                    .or_insert(vec![(x, y)]);
            }
        }
    }

    let mut antinodes = HashSet::with_capacity((width * height) as usize);

    for antena_type_locations in data.values() {
        for locs in antena_type_locations.iter().combinations(2) {
            let mut v = (locs[1].0 - locs[0].0, locs[1].1 - locs[0].1);
            let gcd = num::integer::gcd(v.0, v.1);
            v = (v.0 / gcd, v.1 / gcd);

            let mut p = (locs[0].0, locs[0].1);

            while p.0 >= 0 && p.1 >= 0 && p.0 < width && p.1 < height {
                antinodes.insert(p);
                p = (p.0 + v.0, p.1 + v.1);
            }

            let mut p = (locs[0].0, locs[0].1);

            while p.0 >= 0 && p.1 >= 0 && p.0 < width && p.1 < height {
                antinodes.insert(p);
                p = (p.0 - v.0, p.1 - v.1);
            }
        }
    }

    Ok(antinodes.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "34");
    }
}
