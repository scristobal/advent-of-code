/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::str::FromStr;

use anyhow::Error;
use itertools::Itertools;

#[derive(PartialEq, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

#[derive(Copy, Clone, Debug)]
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
    position: usize,
    direction: Direction,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();

        let s = s.replace('\n', "");

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
                _ => panic!("Invalid character"),
            })
            .collect_vec();

        let position = s.find('S').unwrap();

        let east = &pipes[position + 1];
        let west = &pipes[position - 1];
        let north = &pipes[position - width];
        let south = &pipes[position + width];

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

        Ok(Grid {
            pipes,
            width,
            position,
            direction,
        })
    }
}

#[derive(Debug)]
struct State {
    position: usize,
    direction: Direction,
}

impl Iterator for Grid {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let next_position = match self.direction {
            Direction::North => self.position - self.width,
            Direction::South => self.position + self.width,
            Direction::East => self.position + 1,
            Direction::West => self.position - 1,
        };

        let next_pipe = &self.pipes[next_position];

        let next_direction = match (self.direction, next_pipe) {
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

            (_, _) => return None,
        };

        self.position = next_position;
        self.direction = next_direction;

        Some(State {
            position: self.position,
            direction: self.direction,
        })
    }
}

pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
    let grid = Grid::from_str(input)?;

    let len = (grid.collect_vec().len() + 1) / 2;

    Ok(len.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "-L|F7
 7S-7|
 L|7||
 -L-J|
 L|-JF";

    const SAMPLE_2: &str = "..F7.
 .FJ|.
 SJ.L7
 |F--J
 LJ...";

    #[test]
    #[ignore = "not"]
    fn solve_sample1() {
        let result = solve(SAMPLE_1).unwrap();
        assert_eq!(result, "4");
    }

    #[test]
    #[ignore = "reason"]
    fn solve_sample2() {
        let result = solve(SAMPLE_2).unwrap();
        assert_eq!(result, "8");
    }
}
