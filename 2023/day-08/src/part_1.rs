/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::HashMap;

#[derive(PartialEq, Debug, Eq)]
enum Direction {
    Left,
    Right,
}

type Node = [char; 3];

type RightTurns = HashMap<Node, Node>;
type LeftTurns = HashMap<Node, Node>;

fn parse_input(input: &'static str) -> (Vec<Direction>, RightTurns, LeftTurns) {
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

    let (right, left) = turns
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
            (HashMap::new(), HashMap::new()),
            |(mut right, mut left), (start_node, (left_node, right_node))| {
                right.insert(start_node, right_node);
                left.insert(start_node, left_node);
                (right, left)
            },
        );

    (directions, right, left)
}

pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
    let (directions, right, left) = parse_input(input);

    let mut current_node = ['A'; 3];

    let Some(steps) = directions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(index, direction)| {
            current_node = match direction {
                Direction::Left => *left.get(&current_node).unwrap(),
                Direction::Right => *right.get(&current_node).unwrap(),
            };
            (current_node == ['Z'; 3]).then_some(index + 1)
        })
    else {
        unreachable!()
    };

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn parse_sample() {
        let (directions, right, left) = parse_input(SAMPLE);

        assert_eq!(
            directions,
            vec![Direction::Left, Direction::Left, Direction::Right]
        );

        assert_eq!(right.len(), 3);
        assert_eq!(left.len(), 3);

        assert_eq!(
            left,
            vec![
                (['A', 'A', 'A'], ['B', 'B', 'B']),
                (['B', 'B', 'B'], ['A', 'A', 'A']),
                (['Z', 'Z', 'Z'], ['Z', 'Z', 'Z'])
            ]
            .into_iter()
            .collect()
        );

        assert_eq!(
            right,
            vec![
                (['A', 'A', 'A'], ['B', 'B', 'B']),
                (['B', 'B', 'B'], ['Z', 'Z', 'Z']),
                (['Z', 'Z', 'Z'], ['Z', 'Z', 'Z'])
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
