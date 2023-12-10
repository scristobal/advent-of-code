/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::str::FromStr;

use anyhow::Error;

use itertools::Itertools;

#[derive(Debug)]
struct Grid {
    pipes: Vec<char>,
    width: usize,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();

        let pipes = s.replace('\n', "").chars().collect_vec();

        Ok(Grid { pipes, width })
    }
}

#[derive(Debug)]
struct State {
    position: usize,
    direction: char,
}

impl Grid {
    fn get_start(&self) -> State {
        let position = self
            .pipes
            .iter()
            .enumerate()
            .find(|(_, &s)| s == 'S')
            .unwrap()
            .0;

        let east = &self.pipes[position + 1];
        let west = &self.pipes[position - 1];
        let north = &self.pipes[position - self.width];
        let south = &self.pipes[position + self.width];

        let direction = if *east == '-' || *east == 'L' || *east == '7' {
            'e'
        } else if *west == '-' || *west == 'L' || *west == 'F' {
            'w'
        } else if *north == '|' || *north == 'F' || *north == '7' {
            'n'
        } else if *south == '|' || *south == 'L' || *south == 'J' {
            's'
        } else {
            panic!("Invalid start position");
        };

        State {
            position,
            direction,
        }
    }

    fn next_state(&self, state: &State) -> Option<State> {
        let position = match state.direction {
            'n' => state.position - self.width,
            's' => state.position + self.width,
            'e' => state.position + 1,
            'w' => state.position - 1,
            ch => unreachable!("Invalid direction {}", ch),
        };

        let next_pipe = &self.pipes[position];

        let direction = match (state.direction, next_pipe) {
            ('n', '|') => 'n',
            ('n', 'F') => 'e',
            ('n', '7') => 'w',

            ('s', '|') => 's',
            ('s', 'L') => 'e',
            ('s', 'J') => 'w',

            ('e', '-') => 'e',
            ('e', 'J') => 'n',
            ('e', '7') => 's',

            ('w', '-') => 'w',
            ('w', 'L') => 'n',
            ('w', 'F') => 's',

            (_, _) => return None,
        };

        Some(State {
            position,
            direction,
        })
    }
}

pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
    let grid = Grid::from_str(input)?;

    let mut state = grid.get_start();
    let mut len = 1;

    while let Some(next_state) = grid.next_state(&state) {
        len += 1;
        state = next_state;
    }

    Ok((len / 2).to_string())
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
    fn solve_sample1() {
        let result = solve(SAMPLE_1).unwrap();
        assert_eq!(result, "4");
    }

    #[test]
    fn solve_sample2() {
        let result = solve(SAMPLE_2).unwrap();
        assert_eq!(result, "8");
    }
}
