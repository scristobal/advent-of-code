/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::Error;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    RoundedRock, // 'O'
    SquareRock,  // '#'
    Empty,       // '.'
}

struct Platform {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in s.lines() {
            height += 1;
            width = line.len();
            for c in line.chars() {
                match c {
                    'O' => tiles.push(Tile::RoundedRock),
                    '#' => tiles.push(Tile::SquareRock),
                    '.' => tiles.push(Tile::Empty),
                    _ => return Err(anyhow::anyhow!("Invalid character")),
                }
            }
        }

        Ok(Platform {
            tiles,
            width,
            height,
        })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.chunks(self.width) {
            for tile in row {
                match tile {
                    Tile::RoundedRock => write!(f, "O")?,
                    Tile::SquareRock => write!(f, "#")?,
                    Tile::Empty => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Platform {
    fn clone_col(&self, i: usize) -> Vec<Tile> {
        self.tiles
            .iter()
            .cloned()
            .skip(i)
            .step_by(self.width)
            .collect()
    }

    fn clone_row(&self, i: usize) -> Vec<Tile> {
        self.tiles
            .iter()
            .cloned()
            .skip(i * self.width)
            .take(self.width)
            .collect()
    }

    fn move_to_end(c: &mut [Tile]) {
        use Tile::*;

        c.split_mut(|t| *t == SquareRock).for_each(|s| {
            let num_empty = s.iter().filter(|r| **r == Empty).count();

            s.iter_mut().enumerate().for_each(|(i, t)| {
                if i < num_empty {
                    *t = Empty
                } else {
                    *t = RoundedRock
                }
            });
        });
    }

    fn move_to_beginning(c: &mut [Tile]) {
        use Tile::*;

        c.split_mut(|t| *t == SquareRock).for_each(|s| {
            let num_rounded = s.iter().filter(|r| **r == RoundedRock).count();
            s.iter_mut().enumerate().for_each(|(i, t)| {
                if i < num_rounded {
                    *t = RoundedRock
                } else {
                    *t = Empty
                }
            });
        });
    }

    pub fn tilt_north(&mut self) {
        for i in 0..self.width {
            let mut col: Vec<_> = self.clone_col(i);

            Self::move_to_beginning(&mut col);

            self.tiles
                .iter_mut()
                .skip(i)
                .step_by(self.width)
                .zip(col)
                .for_each(|(t, c)| *t = c);
        }
    }

    pub fn tilt_south(&mut self) {
        for i in 0..self.width {
            let mut col: Vec<_> = self.clone_col(i);

            Self::move_to_end(&mut col);

            self.tiles
                .iter_mut()
                .skip(i)
                .step_by(self.width)
                .zip(col)
                .for_each(|(t, c)| *t = c);
        }
    }

    pub fn tilt_east(&mut self) {
        for i in 0..self.height {
            let mut row: Vec<_> = self.clone_row(i);

            Self::move_to_end(&mut row);

            self.tiles
                .iter_mut()
                .skip(i * self.width)
                .take(self.width)
                .zip(row)
                .for_each(|(t, c)| *t = c);
        }
    }

    pub fn tilt_west(&mut self) {
        for i in 0..self.height {
            let mut row: Vec<_> = self.clone_row(i);

            Self::move_to_beginning(&mut row);

            self.tiles
                .iter_mut()
                .skip(i * self.width)
                .take(self.width)
                .zip(row)
                .for_each(|(t, c)| *t = c);
        }
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}

pub fn solve(input: &'static str) -> String {
    let mut platform = Platform::from_str(input).unwrap();

    let num_cycles = 1_000_000_000;

    let mut cache: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut i = 1;
    let n = 10; // cycle detection after n repetitions

    let hits = loop {
        platform.cycle();

        let load = platform
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                (*t == Tile::RoundedRock).then_some(platform.height - (i / platform.width))
            })
            .sum();

        cache
            .entry(load)
            .and_modify(|indexes| indexes.push(i))
            .or_insert(vec![i]);

        i += 1;
        let Some(hits) = cache.get(&load) else {
            continue;
        };

        if hits.len() > n
            && hits
                .iter()
                .take(n)
                .inspect(|v| {
                    dbg!(v);
                })
                .tuple_windows()
                .map(|(a, b)| b - a)
                .all_equal()
        {
            break hits;
        }
    };

    let last = hits.last().unwrap();
    let prev_last = hits.iter().rev().nth(1).unwrap();

    let period = last - prev_last;
    let offset = prev_last;

    let num_cycles = offset + (num_cycles - offset) % period;

    let load = cache
        .into_iter()
        .find(|(_, value)| value.contains(&num_cycles))
        .unwrap()
        .0;

    load.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "64");
    }
}
