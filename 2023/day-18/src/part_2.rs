/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use itertools::Itertools;

#[derive(Debug)]
struct Instruction {
    dir: char,
    len: i128,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(' ').unwrap();

            let (_, bugged) = l.split_once(' ').unwrap();

            let dir = match bugged.chars().nth(7).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!(),
            };

            let len = i128::from_str_radix(&bugged.chars().skip(2).take(5).collect::<String>(), 16)
                .unwrap();

            Instruction { dir, len }
        })
        .collect()
}

fn solve_shoelace(instructions: &[Instruction]) -> i128 {
    let mut cur = (0_i128, 0_i128);

    let pivots: Vec<_> = instructions
        .iter()
        .map(|ins| {
            cur = match ins.dir {
                'U' => (cur.0 - ins.len, cur.1),
                'D' => (cur.0 + ins.len, cur.1),
                'L' => (cur.0, cur.1 - ins.len),
                'R' => (cur.0, cur.1 + ins.len),
                _ => panic!(),
            };

            cur
        })
        .collect();

    let pivots: Vec<_> = pivots.iter().chain(pivots.first()).collect();

    // Shoelace (Trapezoid) formula
    let area = pivots
        .iter()
        .tuple_windows()
        .fold(0_i128, |acc, (p, q)| acc + ((p.0 + q.0) * (p.1 - q.1)))
        / 2;

    let num_border: i128 = instructions
        .iter()
        .take(instructions.len())
        .fold(0, |acc, item| acc + item.len);

    // inverse of Pick's Theorem
    let num_inside = area - (num_border / 2) + 1;

    num_inside + num_border
}

pub fn solve(input: &str) -> String {
    let instructions = parse(input);

    let a = solve_shoelace(&instructions);

    a.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "952408144115");
    }
}
