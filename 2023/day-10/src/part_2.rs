/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::{fmt::Display, str::FromStr};

use anyhow::Error;
use itertools::Itertools;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Inside,
    Outside,
    Start,
    Border,
    Main,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Grid {
    pipes: Vec<Pipe>,
    width: usize,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len() + 2;

        let mut s = s.replace('\n', "BB");

        s.push_str(&"B".repeat(width + 1));

        s.insert_str(0, &"B".repeat(width + 1));

        let pipes = s
            .chars()
            .map(|c| match c {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::NorthEast,
                'J' => Pipe::NorthWest,
                '7' => Pipe::SouthWest,
                'F' => Pipe::SouthEast,
                '.' => Pipe::Ground,
                'S' => Pipe::Start,
                'B' => Pipe::Border,

                _ => panic!("Invalid character"),
            })
            .collect_vec();

        Ok(Grid { pipes, width })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for (i, pipe) in self.pipes.iter().enumerate() {
            if i % self.width == 0 {
                s.push('\n');
            }

            let c = match pipe {
                Pipe::Vertical => '|',
                Pipe::Horizontal => '-',
                Pipe::NorthEast => 'L',
                Pipe::NorthWest => 'J',
                Pipe::SouthEast => 'F',
                Pipe::SouthWest => '7',
                Pipe::Ground => '.',
                Pipe::Inside => 'I',
                Pipe::Outside => 'O',
                Pipe::Start => 'S',
                Pipe::Border => ' ',
                Pipe::Main => '*',
            };

            s.push(c);
        }

        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    position: usize,
    direction: Direction,
}

impl Grid {
    fn initial_state(&self) -> State {
        let position = self
            .pipes
            .iter()
            .enumerate()
            .find(|(_, p)| **p == Pipe::Start)
            .unwrap()
            .0;

        let east = &self.pipes[position + 1];
        let west = &self.pipes[position - 1];
        let north = &self.pipes[position - self.width];
        let south = &self.pipes[position + self.width];

        let direction = if *east == Pipe::Horizontal
            || *east == Pipe::NorthWest
            || *east == Pipe::SouthWest
        {
            Direction::East
        } else if *west == Pipe::Horizontal || *west == Pipe::NorthEast || *west == Pipe::SouthEast
        {
            Direction::West
        } else if *north == Pipe::Vertical || *north == Pipe::SouthEast || *north == Pipe::SouthWest
        {
            Direction::North
        } else if *south == Pipe::Vertical || *south == Pipe::NorthEast || *south == Pipe::NorthWest
        {
            Direction::South
        } else {
            panic!("Invalid start position");
        };

        State {
            direction,
            position,
        }
    }

    fn next_state(&self, state: State) -> Option<State> {
        let next_position = match state.direction {
            Direction::North => state.position - self.width,
            Direction::South => state.position + self.width,
            Direction::East => state.position + 1,
            Direction::West => state.position - 1,
        };

        let next_pipe = &self.pipes[next_position];

        let next_direction = match (state.direction, next_pipe) {
            (Direction::North, Pipe::Vertical) => Direction::North,
            (Direction::North, Pipe::SouthEast) => Direction::East,
            (Direction::North, Pipe::SouthWest) => Direction::West,

            (Direction::South, Pipe::Vertical) => Direction::South,
            (Direction::South, Pipe::NorthEast) => Direction::East,
            (Direction::South, Pipe::NorthWest) => Direction::West,

            (Direction::East, Pipe::Horizontal) => Direction::East,
            (Direction::East, Pipe::NorthWest) => Direction::North,
            (Direction::East, Pipe::SouthWest) => Direction::South,

            (Direction::West, Pipe::Horizontal) => Direction::West,
            (Direction::West, Pipe::NorthEast) => Direction::North,
            (Direction::West, Pipe::SouthEast) => Direction::South,

            (_, Pipe::Start) => return None,
            (d, p) => panic!("Invalid state {:?} and pipe {:?} combination", d, p),
        };

        Some(State {
            position: next_position,
            direction: next_direction,
        })
    }

    fn erase(&mut self, main_loop: &[State]) {
        let main_loop = main_loop
            .iter()
            .map(|State { position, .. }| position)
            .collect_vec();

        for (i, pipe) in self.pipes.iter_mut().enumerate() {
            if !main_loop.contains(&&i) && *pipe != Pipe::Border {
                *pipe = Pipe::Ground;
            }
            if main_loop.contains(&&i) {
                *pipe = Pipe::Main;
            }
        }
    }

    fn mark_inside(&mut self, main_loop: &Vec<State>) -> usize {
        for state in main_loop {
            let right_adjacents = match state.direction {
                Direction::North => [
                    state.position - self.width + 1,
                    state.position + 1,
                    state.position + self.width + 1,
                ],
                Direction::South => [
                    state.position - self.width - 1,
                    state.position - 1,
                    state.position + self.width - 1,
                ],
                Direction::East => [
                    state.position + self.width - 1,
                    state.position + self.width,
                    state.position + self.width + 1,
                ],
                Direction::West => [
                    state.position - self.width - 1,
                    state.position - self.width,
                    state.position - self.width + 1,
                ],
            };

            for right_adjacent in right_adjacents {
                if self.pipes[right_adjacent] == Pipe::Ground {
                    self.flood(right_adjacent, Pipe::Inside);
                }
            }

            let left_adjacent = match state.direction {
                Direction::North => [
                    state.position - self.width - 1,
                    state.position - 1,
                    state.position + self.width - 1,
                ],
                Direction::South => [
                    state.position - self.width + 1,
                    state.position + 1,
                    state.position + self.width + 1,
                ],
                Direction::East => [
                    state.position - self.width - 1,
                    state.position - self.width,
                    state.position - self.width + 1,
                ],
                Direction::West => [
                    state.position + self.width - 1,
                    state.position + self.width,
                    state.position + self.width + 1,
                ],
            };

            for left_adjacent in left_adjacent {
                if self.pipes[left_adjacent] == Pipe::Ground {
                    self.flood(left_adjacent, Pipe::Outside);
                }
            }
        }

        let real_outside = self.pipes[self.width + 1];

        let real_inside = match real_outside {
            Pipe::Inside => Pipe::Outside,
            Pipe::Outside => Pipe::Inside,
            _ => panic!("Invalid outside pipe"),
        };

        if real_inside == Pipe::Outside {
            self.pipes.iter_mut().for_each(|p| {
                if *p == Pipe::Inside {
                    *p = Pipe::Outside;
                } else if *p == Pipe::Outside {
                    *p = Pipe::Inside;
                }
            });
        }

        self.pipes.iter().filter(|&p| *p == real_outside).count()
    }

    fn flood(&mut self, position: usize, pipe: Pipe) {
        self.pipes[position] = pipe;

        let mut queued = vec![position];

        while let Some(position) = queued.pop() {
            if self.pipes[position + 1] == Pipe::Ground {
                queued.push(position + 1);
                self.pipes[position + 1] = pipe;
            }
            if self.pipes[position - 1] == Pipe::Ground {
                queued.push(position - 1);
                self.pipes[position - 1] = pipe;
            }
            if self.pipes[position + self.width] == Pipe::Ground {
                queued.push(position + self.width);
                self.pipes[position + self.width] = pipe;
            }
            if self.pipes[position - self.width] == Pipe::Ground {
                queued.push(position - self.width);
                self.pipes[position - self.width] = pipe;
            }
        }
    }
}

pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
    let mut grid = Grid::from_str(input)?;

    let mut state = grid.initial_state();

    let mut main_loop = vec![state.clone()];

    while let Some(next_state) = grid.next_state(state.clone()) {
        state = next_state.clone();
        main_loop.push(next_state);
    }

    grid.erase(&main_loop);

    let insiders = grid.mark_inside(&main_loop);

    println!("{}", &grid);

    Ok(insiders.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn solve_sample1() {
        let result = solve(SAMPLE_1).unwrap();
        assert_eq!(result, "4");
    }

    const SAMPLE_2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn solve_sample2() {
        let result = solve(SAMPLE_2).unwrap();
        assert_eq!(result, "8");
    }

    const SAMPLE_3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn solve_sample3() {
        let result = solve(SAMPLE_3).unwrap();
        assert_eq!(result, "10");
    }
}
