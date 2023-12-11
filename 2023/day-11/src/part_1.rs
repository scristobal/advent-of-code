/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::{cmp, fmt::Display, str::FromStr};

use anyhow::Error;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Galaxy,
    Space,
}

#[derive(Debug)]
struct Universe {
    tiles: Vec<Tile>,
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
            .collect();

        Ok(Universe {
            tiles,
            width,
            height,
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
    fn row(&self, i: usize) -> Vec<Tile> {
        self.tiles
            .iter()
            .cloned()
            .enumerate()
            .filter(|(j, _)| (j / self.width) == i)
            .map(|(_, t)| t)
            .collect()
    }
    fn col(&self, i: usize) -> Vec<Tile> {
        self.tiles
            .iter()
            .cloned()
            .enumerate()
            .filter(|(j, _)| j % self.width == i)
            .map(|(_, t)| t)
            .collect()
    }

    fn is_empty_row(&self, i: usize) -> bool {
        self.row(i).iter().all(|t| *t == Tile::Space)
    }

    fn is_empty_col(&self, i: usize) -> bool {
        self.col(i).iter().all(|t| *t == Tile::Space)
    }

    fn expand_row(&mut self, i: usize) {
        self.tiles.splice(
            i * self.width..i * self.width,
            vec![Tile::Space; self.width],
        );
        self.height += 1;
    }

    fn expand_col(&mut self, i: usize) {
        for j in (0..self.height).rev() {
            let k = i + j * self.width;
            self.tiles.insert(k, Tile::Space);
        }
        self.width += 1;
    }

    fn distance(&self, i: usize, j: usize) -> usize {
        let col_i = i % self.width;
        let col_j = j % self.width;

        let col_max = cmp::max(col_j, col_i);
        let col_min = cmp::min(col_j, col_i);

        let row_i = i / self.width;
        let row_j = j / self.width;

        let row_max = cmp::max(row_j, row_i);
        let row_min = cmp::min(row_j, row_i);

        col_max - col_min + (row_max - row_min)
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
    let mut universe: Universe = input.parse().unwrap();

    let mut num_col = universe.width;

    while 1 <= num_col {
        if universe.is_empty_col(num_col) {
            universe.expand_col(num_col);
        }
        num_col -= 1;
    }

    let mut num_row = universe.height;

    while 1 <= num_row {
        if universe.is_empty_row(num_row) {
            universe.expand_row(num_row);
        }
        num_row -= 1;
    }

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
    fn universe_impl() {
        let mut universe: Universe = SAMPLE.parse().unwrap();

        let row = universe.row(4);

        let expected_row = vec![
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Galaxy,
            Tile::Space,
            Tile::Space,
            Tile::Space,
        ];

        assert_eq!(expected_row, row);

        let col = universe.col(4);

        let expected_col = vec![
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Space,
            Tile::Galaxy,
        ];

        assert_eq!(expected_col, col);

        universe.expand_col(2);
        let col = universe.col(2);

        assert_eq!(col, vec![Tile::Space; 10]);

        let col = universe.col(3);

        assert_eq!(col, vec![Tile::Space; 10]);

        universe.expand_row(3);

        let row = universe.row(3);

        assert_eq!(row, vec![Tile::Space; 11]);

        let row = universe.row(4);

        assert_eq!(row, vec![Tile::Space; 11]);

        universe.expand_col(6);

        universe.expand_col(10);

        universe.expand_row(8);

        let i = 13 * 6 + 1;
        let j = 13 * 11 + 5;
        let d = 9;

        assert_eq!(universe.tiles[i], Tile::Galaxy);
        assert_eq!(universe.tiles[j], Tile::Galaxy);

        let distance = universe.distance(i, j);

        assert_eq!(distance, d);

        let i = 4;
        let j = 13 * 10 + 9;
        let d = 15;

        assert_eq!(universe.tiles[i], Tile::Galaxy);
        assert_eq!(universe.tiles[j], Tile::Galaxy);

        let distance = universe.distance(i, j);

        assert_eq!(distance, d);

        let i = 13 * 2;
        let j = 13 * 7 + 12;

        let d = 17;
        assert_eq!(universe.tiles[i], Tile::Galaxy);
        assert_eq!(universe.tiles[j], Tile::Galaxy);

        let distance = universe.distance(i, j);

        assert_eq!(distance, d);

        let i = 13 * 11;
        let j = 13 * 11 + 5;

        let d = 5;

        assert_eq!(universe.tiles[i], Tile::Galaxy);
        assert_eq!(universe.tiles[j], Tile::Galaxy);

        let distance = universe.distance(i, j);

        assert_eq!(distance, d);
    }

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "374");
    }
}
