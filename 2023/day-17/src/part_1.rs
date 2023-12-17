/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};

use ndarray::{Array, ArrayBase, Dim, OwnedRepr};

struct HeatLossMap(ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>>);

fn parse(input: &str) -> HeatLossMap {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.chars().filter(|c| *c == '\n').count() + 1;

    let input = input.replace('\n', "");

    HeatLossMap(
        Array::from_shape_vec(
            (width, height),
            input
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
        )
        .unwrap(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    heat_loss: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    // ordered by self.heat_loss in reverse
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss).reverse()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

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
    fn step(&self, heat_loss_map: &HeatLossMap) -> Vec<State> {
        let mut new_states = vec![];

        let dim = heat_loss_map.0.dim();

        // try move forward
        if self.consecutive_steps < MAX_CONSECUTIVE_STEPS {
            let position = move_in_direction(&self.position, &self.direction, &dim);

            if let Some(position) = position {
                new_states.push(State {
                    position,
                    consecutive_steps: self.consecutive_steps + 1,
                    direction: self.direction,
                    heat_loss: self.heat_loss + *heat_loss_map.0.get(position).unwrap() as usize,
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
                heat_loss: self.heat_loss + *heat_loss_map.0.get(position).unwrap() as usize,
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
                heat_loss: self.heat_loss + *heat_loss_map.0.get(position).unwrap() as usize,
            });
        };

        new_states
    }
}

pub fn solve(input: &'static str) -> String {
    let heat_loss_map = parse(input);

    let end_position = (heat_loss_map.0.dim().0 - 1, heat_loss_map.0.dim().1 - 1);

    let initial_state = State {
        position: (0, 0),
        direction: Direction::Right,
        consecutive_steps: 0,
        heat_loss: 0,
    };

    let mut visited = HashSet::new();

    let mut neighbors: BinaryHeap<State> = BinaryHeap::new();

    neighbors.push(initial_state);

    let min_heat_loss = loop {
        if let Some(heat_loss) = neighbors
            .iter()
            .filter_map(|path_finder| {
                (path_finder.position == end_position).then_some(path_finder.heat_loss)
            })
            .min()
        {
            break heat_loss;
        }

        let Some(state) = neighbors.pop() else {
            panic!("No more neighbors to explore");
        };

        let states = state.step(&heat_loss_map);

        for state in states {
            if !visited.contains(&state) {
                neighbors.push(state)
            }
        }

        visited.insert(state);
    };

    min_heat_loss.to_string()
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
