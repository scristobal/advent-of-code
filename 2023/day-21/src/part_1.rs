/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::HashSet;

struct Garden {
    rocks: HashSet<(isize, isize)>,
}

fn parse(input: &str) -> Garden {
    let width = input.lines().next().unwrap().len();

    let input = input.replace('\n', "");

    let start = input
        .chars()
        .enumerate()
        .position(|(_, c)| c == 'S')
        .unwrap();

    let start: (isize, isize) = ((start % width) as isize, (start / width) as isize);

    let rocks: HashSet<_> = input
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i)
        .collect();

    let rocks: HashSet<(isize, isize)> = rocks
        .iter()
        .map(|i| ((i % width) as isize, (i / width) as isize))
        .map(|p| (p.0 - start.0, p.1 - start.1))
        .collect();

    Garden { rocks }
}

fn generate_neighbors(position: (isize, isize)) -> Vec<(isize, isize)> {
    vec![
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ]
}

fn solve_steps(garden: Garden, num_steps: isize) -> usize {
    let mut queue = vec![((0, 0), 0)]; // position, number of steps so far
    let mut visited = HashSet::new();

    while let Some(state) = queue.pop() {
        if !visited.contains(&state) {
            if state.1 < num_steps {
                let neighbors = generate_neighbors(state.0);

                for neighbor in neighbors {
                    if !garden.rocks.contains(&neighbor) {
                        queue.push((neighbor, state.1 + 1));
                    }
                }
            }

            visited.insert(state);
        }
    }

    visited
        .iter()
        .filter_map(|(p, c)| (*c == num_steps).then_some(p))
        .collect::<HashSet<_>>()
        .len()
}

pub fn solve(input: &'static str) -> String {
    let garden = parse(input);

    solve_steps(garden, 64).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let garden = parse(SAMPLE);

        let result = solve_steps(garden, 6);

        assert_eq!(result, 16);
    }
}
