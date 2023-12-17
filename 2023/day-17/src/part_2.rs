/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use ndarray::{Array, ArrayBase, Dim, OwnedRepr};
use pathfinding::prelude::dijkstra;

type Matrix = ArrayBase<OwnedRepr<usize>, Dim<[usize; 2]>>;

fn parse(input: &str) -> Matrix {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.chars().filter(|c| *c == '\n').count() + 1;

    let input = input.replace('\n', "");

    Array::from_shape_vec(
        (height, width),
        input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>(),
    )
    .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

const MAX_CONSECUTIVE_STEPS: usize = 10;
const MIN_STEPS_BEFORE_TURNING: usize = 4;

#[derive(Debug, Clone, Copy, Hash)]
struct State {
    position: (usize, usize),
    consecutive_steps: usize,
    direction: Direction,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for State {}

fn move_in_direction(
    position: &(usize, usize),
    direction: &Direction,
    dim: &(usize, usize),
) -> Option<(usize, usize)> {
    match direction {
        Up if position.0 > 0 => Some((position.0 - 1, position.1)),
        Down if position.0 < dim.0 - 1 => Some((position.0 + 1, position.1)),
        Left if position.1 > 0 => Some((position.0, position.1 - 1)),
        Right if position.1 < dim.1 - 1 => Some((position.0, position.1 + 1)),
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
                });
            };
        }

        // try move left

        if MIN_STEPS_BEFORE_TURNING <= self.consecutive_steps {
            let direction = match self.direction {
                Up => Left,
                Down => Right,
                Left => Down,
                Right => Up,
            };

            let position = move_in_direction(&self.position, &direction, &dim);

            if let Some(position) = position {
                new_states.push(State {
                    position,
                    consecutive_steps: 1,
                    direction,
                });
            };
        }

        // try move right
        if MIN_STEPS_BEFORE_TURNING <= self.consecutive_steps {
            let direction = match self.direction {
                Up => Right,
                Down => Left,
                Left => Up,
                Right => Down,
            };

            let position = move_in_direction(&self.position, &direction, &dim);

            if let Some(position) = position {
                new_states.push(State {
                    position,
                    consecutive_steps: 1,
                    direction,
                });
            };
        }

        new_states
    }
}

pub fn solve(input: &'static str) -> String {
    let heat = parse(input);

    let dist_y = dijkstra(
        &State {
            position: (0, 0),
            direction: Right,
            consecutive_steps: 0,
        },
        |state: &State| {
            state
                .step(&heat)
                .into_iter()
                .map(|s| (s, *heat.get(s.position).unwrap()))
        },
        |state: &State| {
            state.position
                == (
                    heat.dim().0 - 1 - MIN_STEPS_BEFORE_TURNING,
                    heat.dim().1 - 1,
                )
        },
    )
    .unwrap()
    .1 + (0..MIN_STEPS_BEFORE_TURNING)
        .map(|y| heat.get((heat.dim().0 - 1 - y, heat.dim().1 - 1)).unwrap())
        .sum::<usize>();

    let dist_x = dijkstra(
        &State {
            position: (0, 0),
            direction: Right,
            consecutive_steps: 0,
        },
        |state: &State| {
            state
                .step(&heat)
                .into_iter()
                .map(|s| (s, *heat.get(s.position).unwrap()))
        },
        |state: &State| {
            state.position
                == (
                    heat.dim().0 - 1,
                    heat.dim().1 - 1 - MIN_STEPS_BEFORE_TURNING,
                )
        },
    )
    .unwrap()
    .1 + (0..MIN_STEPS_BEFORE_TURNING)
        .map(|x| heat.get((heat.dim().0 - 1, heat.dim().1 - 1 - x)).unwrap())
        .sum::<usize>();

    dist_x.min(dist_y).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "94");
    }

    const SAMPLE2: &str = include_str!("../sample2.txt");

    #[test]
    fn solve_sample2() {
        let result = solve(SAMPLE2);
        assert_eq!(result, "71");
    }
}
