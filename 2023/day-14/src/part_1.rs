/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::{fmt::Display, str::FromStr};

use anyhow::Error;

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
    // returns true if the system has changed
    pub fn tilt_north(&mut self) -> bool {
        use Tile::*;
        let mut new_tiles: Vec<Tile> = self.tiles.to_vec();

        new_tiles.iter_mut().enumerate().for_each(|(index, tile)| {
            let row_index = index / self.width;
            let col_index = index % self.width;

            match *tile {
                RoundedRock => {
                    if row_index != 0
                        && self.tiles[(row_index - 1) * self.width + col_index] == Empty
                    {
                        *tile = Empty
                    }
                }
                SquareRock => {}
                Empty => {
                    if row_index != self.height - 1
                        && self.tiles[(row_index + 1) * self.width + col_index] == RoundedRock
                    {
                        *tile = RoundedRock;
                    }
                }
            }
        });

        let has_changed = self.tiles != new_tiles;

        self.tiles = new_tiles;

        has_changed
    }
}

pub fn solve(input: &'static str) -> String {
    let mut platform = Platform::from_str(input).unwrap();

    while platform.tilt_north() {}

    let load: usize = platform
        .tiles
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            (*t == Tile::RoundedRock).then_some(platform.height - (i / platform.width))
        })
        .sum();

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
        assert_eq!(result, "136");
    }
}
