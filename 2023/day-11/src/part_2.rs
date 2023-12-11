/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::{cmp, collections::HashSet, fmt::Display, str::FromStr};

use anyhow::Error;
use itertools::Itertools;

const EXPANSION_RATE: usize = 1_000_000;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Galaxy,
    Space,
}

#[derive(Debug)]
struct Universe {
    tiles: Vec<Tile>,
    empty_cols: HashSet<usize>,
    empty_rows: HashSet<usize>,
    width: usize,
    height: usize,
}

impl FromStr for Universe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap();

        let height = s.chars().filter(|ch| *ch == '\n').count() + 1;

        let tiles = s
            .replace('\n', "")
            .chars()
            .map(|ch| match ch {
                '#' => Tile::Galaxy,
                '.' => Tile::Space,
                ch => unreachable!("Unknown tile: {}", ch),
            })
            .collect::<Vec<_>>();

        let empty_rows = (0..height)
            .filter(|i| {
                tiles
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter(|(j, _)| (j / width) == *i)
                    .map(|(_, t)| t)
                    .all(|t| t == Tile::Space)
            })
            .collect();

        let empty_cols = (0..width)
            .filter(|i| {
                tiles
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter(|(j, _)| j % width == *i)
                    .map(|(_, t)| t)
                    .all(|t| t == Tile::Space)
            })
            .collect();

        Ok(Universe {
            tiles,
            width,
            height,
            empty_cols,
            empty_rows,
        })
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "size: cols: {} rows: {}", self.width, self.height)?;
        for (i, tile) in self.tiles.iter().enumerate() {
            match tile {
                Tile::Galaxy => write!(f, "#")?,
                Tile::Space => write!(f, ".")?,
            }
            if (i + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Universe {
    fn distance(&self, i: usize, j: usize) -> usize {
        let col_i = i % self.width;
        let col_j = j % self.width;

        let col_max = cmp::max(col_j, col_i);
        let col_min = cmp::min(col_j, col_i);

        let num_expanded_cols = self
            .empty_cols
            .iter()
            .filter(|c| col_min <= **c && **c <= col_max)
            .count();

        let row_i = i / self.width;
        let row_j = j / self.width;

        let row_max = cmp::max(row_j, row_i);
        let row_min = cmp::min(row_j, row_i);

        let num_expanded_rows = self
            .empty_rows
            .iter()
            .filter(|r| row_min <= **r && **r <= row_max)
            .count();

        col_max - col_min + num_expanded_cols * (EXPANSION_RATE - 1) + row_max - row_min
            + num_expanded_rows * (EXPANSION_RATE - 1)
    }

    fn galaxy_pairs(&self) -> Vec<(usize, usize)> {
        let galaxies = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == Tile::Galaxy)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        galaxies
            .iter()
            .cloned()
            .cartesian_product(galaxies.iter().cloned())
            .filter(|(g1, g2)| g1 < g2)
            .collect()
    }
}

pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
    let universe: Universe = input.parse().unwrap();

    let galaxy_pairs = universe.galaxy_pairs();

    let distances: usize = galaxy_pairs
        .iter()
        .map(|(g1, g2)| universe.distance(*g1, *g2))
        .sum();

    Ok(distances.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "82000210");
    }
}
