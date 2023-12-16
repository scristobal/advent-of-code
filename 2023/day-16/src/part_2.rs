/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

#[derive(Debug)]
enum Tile {
    RightMirror,     // '/'
    LeftMirror,      // '\'
    HorizontalSplit, // '-'
    VerticalSplit,   // '|'
    Empty,
}

use std::collections::HashSet;

use Tile::*;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Upward,
    Downward,
    Leftward,
    Rightward,
}
use Direction::*;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
}

impl Beam {
    fn step(&mut self, grid: &Grid) {
        match self.direction {
            Upward => {
                self.position.1 = if self.position.1 > 0 {
                    self.position.1 - 1
                } else {
                    0
                }
            }
            Downward => {
                self.position.1 = if self.position.1 < grid.height - 1 {
                    self.position.1 + 1
                } else {
                    grid.height - 1
                }
            }
            Leftward => {
                self.position.0 = if self.position.0 > 0 {
                    self.position.0 - 1
                } else {
                    0
                }
            }
            Rightward => {
                self.position.0 = if self.position.0 < grid.width - 1 {
                    self.position.0 + 1
                } else {
                    grid.width - 1
                }
            }
        }
    }
}

impl Grid {
    fn refract(&self, beam: &Beam) -> Vec<Beam> {
        let tile = self.get(beam.position.0, beam.position.1);

        let directions = match (tile, beam.direction) {
            (RightMirror, Upward) => vec![Rightward],
            (RightMirror, Downward) => vec![Leftward],
            (RightMirror, Leftward) => vec![Downward],
            (RightMirror, Rightward) => vec![Upward],
            (LeftMirror, Upward) => vec![Leftward],
            (LeftMirror, Downward) => vec![Rightward],
            (LeftMirror, Leftward) => vec![Upward],
            (LeftMirror, Rightward) => vec![Downward],
            (HorizontalSplit, Upward) => vec![Leftward, Rightward],
            (HorizontalSplit, Downward) => vec![Leftward, Rightward],
            (HorizontalSplit, Leftward) => vec![Leftward],
            (HorizontalSplit, Rightward) => vec![Rightward],
            (VerticalSplit, Upward) => vec![Upward],
            (VerticalSplit, Downward) => vec![Downward],
            (VerticalSplit, Leftward) => vec![Upward, Downward],
            (VerticalSplit, Rightward) => vec![Upward, Downward],
            (Empty, Upward) => vec![Upward],
            (Empty, Downward) => vec![Downward],
            (Empty, Leftward) => vec![Leftward],
            (Empty, Rightward) => vec![Rightward],
        };

        let position = beam.position;

        directions
            .into_iter()
            .filter_map(|direction| {
                let mut beam = Beam {
                    position,
                    direction,
                };

                beam.step(self);

                (beam.position != position).then_some(beam)
            })
            .collect()
    }
}

struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.width + x]
    }
}

fn parse(input: &str) -> Grid {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.chars().filter(|c| *c == '\n').count() + 1;

    let input = input.replace('\n', "");

    let tiles = input
        .chars()
        .map(|c| match c {
            '.' => Empty,
            '/' => RightMirror,
            '\\' => LeftMirror,
            '-' => HorizontalSplit,
            '|' => VerticalSplit,
            _ => panic!("Invalid character in input"),
        })
        .collect();

    Grid {
        tiles,
        width,
        height,
    }
}

fn count_energized(grid: &Grid, initial: Beam) -> usize {
    let mut queue = vec![initial];
    let mut visited = HashSet::new();

    while let Some(beam) = queue.pop() {
        if !visited.contains(&beam) {
            let new_beams = grid.refract(&beam);

            for beam in new_beams {
                queue.push(beam);
            }
            visited.insert(beam);
        }
    }

    let visited_positions: HashSet<_> = visited.iter().map(|b| b.position).collect();

    visited_positions.len()
}

pub fn solve(input: &'static str) -> String {
    let grid = parse(input);

    (0..grid.width)
        .map(|i| {
            count_energized(
                &grid,
                Beam {
                    position: (i, 0),
                    direction: Downward,
                },
            )
        })
        .chain((0..grid.width).map(|i| {
            count_energized(
                &grid,
                Beam {
                    position: (i, grid.height - 1),
                    direction: Upward,
                },
            )
        }))
        .chain((0..grid.height).map(|i| {
            count_energized(
                &grid,
                Beam {
                    position: (0, i),
                    direction: Rightward,
                },
            )
        }))
        .chain((0..grid.height).map(|i| {
            count_energized(
                &grid,
                Beam {
                    position: (grid.width - 1, i),
                    direction: Leftward,
                },
            )
        }))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "51");
    }
}
