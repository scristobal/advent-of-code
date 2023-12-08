/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::HashMap;

use num::integer;

#[derive(PartialEq, Debug, Eq)]
enum Direction {
    Left,
    Right,
}

type Node = [char; 3];

type RightTurns = HashMap<Node, Node>;
type LeftTurns = HashMap<Node, Node>;

fn parse_input(input: &'static str) -> (Vec<Direction>, Vec<Node>, RightTurns, LeftTurns) {
    let (directions, turns) = input.split_once("\n\n").unwrap();

    let directions = directions
        .lines()
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                'L' => Direction::Left,
                'R' => Direction::Right,
                c => panic!("Invalid direction {:?}", c),
            })
        })
        .collect::<Vec<_>>();

    let (nodes, right, left) = turns
        .lines()
        .map(|line| {
            let (start, ends) = line.split_once(" = ").unwrap();

            let start_node = start.chars().take(3).fold([' '; 3], |mut acc, c| {
                acc[acc.iter().position(|&x| x == ' ').unwrap()] = c;
                acc
            });

            let (left, right) = ends.split_once(", ").unwrap();

            let left_node = left.chars().skip(1).take(3).fold([' '; 3], |mut acc, c| {
                acc[acc.iter().position(|&x| x == ' ').unwrap()] = c;
                acc
            });

            let right_node = right.chars().take(3).fold([' '; 3], |mut acc, c| {
                acc[acc.iter().position(|&x| x == ' ').unwrap()] = c;
                acc
            });

            (start_node, (left_node, right_node))
        })
        .fold(
            (Vec::new(), HashMap::new(), HashMap::new()),
            |(mut nodes, mut right, mut left), (start_node, (left_node, right_node))| {
                if start_node[2] == 'A' {
                    nodes.push(start_node);
                };

                right.insert(start_node, right_node);
                left.insert(start_node, left_node);
                (nodes, right, left)
            },
        );

    (directions, nodes, right, left)
}

pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
    let (directions, mut nodes, right, left) = parse_input(input);

    let steps = nodes
        .iter_mut()
        .filter_map(|node| {
            let mut visited = HashMap::new();

            directions.iter().enumerate().cycle().enumerate().find_map(
                |(steps, (index, direction))| {
                    *node = match direction {
                        Direction::Left => *left.get(node).unwrap(),
                        Direction::Right => *right.get(node).unwrap(),
                    };

                    match visited.get(&(*node, index)) {
                        Some(prev_steps) => Some(steps - *prev_steps),
                        None => {
                            visited.insert((*node, index), steps);
                            None
                        }
                    }
                },
            )
        })
        .fold(1, integer::lcm);

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn parse_sample() {
        let (directions, nodes, right, left) = parse_input(SAMPLE);

        assert_eq!(directions, vec![Direction::Left, Direction::Right]);

        assert_eq!(nodes, vec![['1', '1', 'A'], ['2', '2', 'A']]);

        assert_eq!(
            left,
            vec![
                (['1', '1', 'A'], ['1', '1', 'B']),
                (['2', '2', 'A'], ['2', '2', 'B']),
                (['1', '1', 'B'], ['X', 'X', 'X']),
                (['2', '2', 'B'], ['2', '2', 'C']),
                (['2', '2', 'C'], ['2', '2', 'Z']),
                (['2', '2', 'Z'], ['2', '2', 'B']),
                (['X', 'X', 'X'], ['X', 'X', 'X']),
                (['1', '1', 'Z'], ['1', '1', 'B'])
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(
            right,
            vec![
                (['1', '1', 'A'], ['X', 'X', 'X']),
                (['2', '2', 'A'], ['X', 'X', 'X']),
                (['1', '1', 'B'], ['1', '1', 'Z']),
                (['2', '2', 'B'], ['2', '2', 'C']),
                (['2', '2', 'C'], ['2', '2', 'Z']),
                (['2', '2', 'Z'], ['2', '2', 'B']),
                (['X', 'X', 'X'], ['X', 'X', 'X']),
                (['1', '1', 'Z'], ['X', 'X', 'X'])
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "6");
    }
}
