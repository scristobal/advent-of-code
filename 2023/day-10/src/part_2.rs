/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::{fmt::Display, str::FromStr};

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
        let width = s.lines().next().unwrap().len() + 2;

        let mut s = s.replace('\n', "  ");

        s.push_str(&" ".repeat(width + 1));

        s.insert_str(0, &" ".repeat(width + 1));

        let pipes = s.replace('\n', "").chars().collect_vec();

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

            let ch = match pipe {
                'F' => '┌',
                'L' => '└',
                'J' => '┘',
                '7' => '┐',
                'S' => 'S',
                'I' => 'I',
                '*' => '*',
                '.' => '·',
                _ => *pipe,
            };

            s.push(ch);
        }

        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    position: usize,
    direction: char,
}

impl Grid {
    fn initial_state(&self) -> State {
        let position = self
            .pipes
            .iter()
            .enumerate()
            .find(|(_, p)| **p == 'S')
            .unwrap()
            .0;

        let east = &self.pipes[position + 1];
        let west = &self.pipes[position - 1];
        let north = &self.pipes[position - self.width];
        let south = &self.pipes[position + self.width];

        let direction = if *east == '-' || *east == 'J' || *east == '7' {
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
            direction,
            position,
        }
    }

    fn next_state(&self, state: State) -> Option<State> {
        let next_position = match state.direction {
            'n' => state.position - self.width,
            's' => state.position + self.width,
            'e' => state.position + 1,
            'w' => state.position - 1,
            ch => unreachable!("Invalid direction {}", ch),
        };

        let next_pipe = &self.pipes[next_position];

        let next_direction = match (state.direction, next_pipe) {
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

            (_, 'S') => return None,
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
            if !main_loop.contains(&&i) && *pipe != ' ' {
                *pipe = '.';
            }
            // if main_loop.contains(&&i) {
            //     // *pipe = '*';
            // }
        }
    }

    fn mark_inside(&mut self, main_loop: &Vec<State>) -> usize {
        for state in main_loop {
            let right_adjacents = match state.direction {
                'n' => [
                    state.position - self.width + 1,
                    state.position + 1,
                    state.position + self.width + 1,
                ],
                's' => [
                    state.position - self.width - 1,
                    state.position - 1,
                    state.position + self.width - 1,
                ],
                'e' => [
                    state.position + self.width - 1,
                    state.position + self.width,
                    state.position + self.width + 1,
                ],
                'w' => [
                    state.position - self.width - 1,
                    state.position - self.width,
                    state.position - self.width + 1,
                ],
                _ => unreachable!(),
            };

            for right_adjacent in right_adjacents {
                if self.pipes[right_adjacent] == '.' {
                    self.flood(right_adjacent, 'A');
                }
            }

            let left_adjacent = match state.direction {
                'n' => [
                    state.position - self.width - 1,
                    state.position - 1,
                    state.position + self.width - 1,
                ],
                's' => [
                    state.position - self.width + 1,
                    state.position + 1,
                    state.position + self.width + 1,
                ],
                'e' => [
                    state.position - self.width - 1,
                    state.position - self.width,
                    state.position - self.width + 1,
                ],
                'w' => [
                    state.position + self.width - 1,
                    state.position + self.width,
                    state.position + self.width + 1,
                ],
                _ => unreachable!(),
            };

            for left_adjacent in left_adjacent {
                if self.pipes[left_adjacent] == '.' {
                    self.flood(left_adjacent, 'B');
                }
            }
        }

        let outside = self.pipes[self.width + 1];

        let inside = match outside {
            'A' => 'B',
            'B' => 'A',
            _ => panic!("Invalid outside pipe"),
        };

        self.pipes.iter_mut().for_each(|p| {
            if *p == outside {
                *p = 'O';
            } else if *p == inside {
                *p = 'I';
            }
        });

        self.pipes.iter().filter(|&p| *p == 'I').count()
    }

    fn flood(&mut self, position: usize, pipe: char) {
        self.pipes[position] = pipe;

        let mut queued = vec![position];

        while let Some(position) = queued.pop() {
            if self.pipes[position + 1] == '.' {
                queued.push(position + 1);
                self.pipes[position + 1] = pipe;
            }
            if self.pipes[position - 1] == '.' {
                queued.push(position - 1);
                self.pipes[position - 1] = pipe;
            }
            if self.pipes[position + self.width] == '.' {
                queued.push(position + self.width);
                self.pipes[position + self.width] = pipe;
            }
            if self.pipes[position - self.width] == '.' {
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
