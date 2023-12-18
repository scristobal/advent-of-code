/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::HashSet;

struct Instruction {
    dir: char,
    len: usize,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (dir, l) = l.split_once(' ').unwrap();
            let dir = dir.chars().next().unwrap();

            let (length, _) = l.split_once(' ').unwrap();
            let len = length.parse().unwrap();

            Instruction { dir, len }
        })
        .collect()
}

pub fn solve(input: &'static str) -> String {
    let instructions = parse(input);

    let mut cur = (0_i32, 0_i32);

    let border: Vec<_> = instructions
        .iter()
        .flat_map(|ins| -> Vec<_> {
            let seg = (0..ins.len as i32)
                .map(|d| match ins.dir {
                    'U' => (cur.0 - d, cur.1),
                    'D' => (cur.0 + d, cur.1),
                    'L' => (cur.0, cur.1 - d),
                    'R' => (cur.0, cur.1 + d),
                    _ => panic!(),
                })
                .collect();

            cur = match ins.dir {
                'U' => (cur.0 - ins.len as i32, cur.1),
                'D' => (cur.0 + ins.len as i32, cur.1),
                'L' => (cur.0, cur.1 - ins.len as i32),
                'R' => (cur.0, cur.1 + ins.len as i32),
                _ => panic!(),
            };

            seg
        })
        .collect();

    let mut queue = vec![(1, 1)];
    let mut visited = HashSet::new();

    while let Some(position) = queue.pop() {
        if !visited.contains(&position) {
            let positions = vec![
                (position.0 + 1, position.1),
                (position.0 - 1, position.1),
                (position.0, position.1 + 1),
                (position.0, position.1 - 1),
            ];

            for position in positions {
                if !border.contains(&position) && !visited.contains(&position) {
                    queue.push(position)
                }
            }

            visited.insert(position);
        }
    }

    (visited.len() + border.len()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "62");
    }
}
