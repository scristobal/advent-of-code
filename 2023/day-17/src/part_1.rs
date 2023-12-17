/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::BinaryHeap;

use ndarray::{Array, ArrayBase, Dim, OwnedRepr};

type Matrix = ArrayBase<OwnedRepr<usize>, Dim<[usize; 2]>>;

fn parse(input: &str) -> Matrix {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.chars().filter(|c| *c == '\n').count() + 1;

    let input = input.replace('\n', "");

    Array::from_shape_vec(
        (width, height),
        input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>(),
    )
    .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const MAX_CONSECUTIVE_STEPS: usize = 3;

#[derive(Debug, Clone, Copy)]
struct State {
    position: (usize, usize),
    consecutive_steps: usize,
    direction: Direction,
    g_score: usize,
    f_score: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    // ordered by self.f_score in reverse
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.direction == other.direction
            && self.consecutive_steps == other.consecutive_steps
    }
}

impl Eq for State {}

fn move_in_direction(
    position: &(usize, usize),
    direction: &Direction,
    dim: &(usize, usize),
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up if position.0 > 0 => Some((position.0 - 1, position.1)),
        Direction::Down if position.0 < dim.0 - 1 => Some((position.0 + 1, position.1)),
        Direction::Left if position.1 > 0 => Some((position.0, position.1 - 1)),
        Direction::Right if position.1 < dim.1 - 1 => Some((position.0, position.1 + 1)),
        _ => None,
    }
}

impl State {
    fn step(&self, heat: &Matrix) -> Vec<State> {
        let mut new_states = vec![];

        let dim = heat.dim();

        // try move forward
        if self.consecutive_steps < MAX_CONSECUTIVE_STEPS {
            let position = move_in_direction(&self.position, &self.direction, &dim);

            if let Some(position) = position {
                new_states.push(State {
                    position,
                    consecutive_steps: self.consecutive_steps + 1,
                    direction: self.direction,
                    f_score: usize::MAX,
                    g_score: usize::MAX,
                });
            };
        }

        // try move left
        let direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };

        let position = move_in_direction(&self.position, &direction, &dim);

        if let Some(position) = position {
            new_states.push(State {
                position,
                consecutive_steps: 1,
                direction,
                f_score: usize::MAX,
                g_score: usize::MAX,
            });
        };

        // try move right
        let direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };

        let position = move_in_direction(&self.position, &direction, &dim);

        if let Some(position) = position {
            new_states.push(State {
                position,
                consecutive_steps: 1,
                direction,
                f_score: usize::MAX,
                g_score: usize::MAX,
            });
        };

        new_states
    }
}

pub fn solve(input: &'static str) -> String {
    let heat = parse(input);
    let end_position = (heat.dim().0 - 1, heat.dim().1 - 1);

    let h = |state: &State| -> usize {
        let dx = (state.position.0 as isize - end_position.0 as isize).abs();
        let dy = (state.position.1 as isize - end_position.1 as isize).abs();

        (dx + dy) as usize
    };

    let mut initial_state = State {
        position: (0, 0),
        direction: Direction::Right,
        consecutive_steps: 0,
        g_score: 0,
        f_score: 0,
    };

    initial_state.f_score = h(&initial_state);

    let mut open_set: BinaryHeap<State> = BinaryHeap::new();
    open_set.push(initial_state);

    let d = |_current: &State, neighbor: &State| -> usize { *heat.get(neighbor.position).unwrap() };

    while let Some(state) = open_set.pop() {
        if state.position == end_position {
            return state.g_score.to_string();
        }

        let neighbor_states = state.step(&heat);

        for mut neighbor in neighbor_states {
            let tentative_g_score = state.g_score + d(&state, &neighbor);

            if tentative_g_score < neighbor.g_score {
                neighbor.g_score = tentative_g_score;
                neighbor.f_score = tentative_g_score + h(&neighbor);

                if !open_set.as_slice().contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "102");
    }
}
