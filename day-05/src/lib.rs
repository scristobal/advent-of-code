use itertools::Itertools;

use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    times: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut t = s
            .chars()
            .filter(|c| c.is_numeric())
            .filter_map(|c| c.to_digit(10))
            .map(|v| v as usize)
            .collect::<Vec<_>>();

        let to = t.pop().unwrap() - 1;
        let from = t.pop().unwrap() - 1;

        let times = t
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, e)| acc + 10_usize.pow(i as u32) * e);

        Ok(Move { times, to, from })
    }
}

#[derive(Debug)]
struct Stacks(HashMap<usize, VecDeque<char>>);

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Stacks(
            s.lines()
                .flat_map(|line| {
                    line.chars()
                        .skip(1)
                        .step_by(4)
                        .enumerate()
                        .filter(|(_, label)| !label.is_whitespace())
                        .filter(|(_, label)| label.is_ascii_alphabetic())
                })
                .into_grouping_map()
                .collect(),
        ))
    }
}

impl Stacks {
    fn move_crates_9000(&mut self, Move { times, from, to }: Move) {
        let source_crates = self.0.get_mut(&from).unwrap();

        let mut piles = source_crates.drain(0..times).rev().collect::<VecDeque<_>>();

        let destination_crate = self.0.get_mut(&to).unwrap();

        piles.append(destination_crate);

        self.0.insert(to, piles);
    }

    fn move_crates_9001(&mut self, Move { times, to, from }: Move) {
        let source_crates = self.0.get_mut(&from).unwrap();

        let mut piles = source_crates.drain(0..times).collect::<VecDeque<_>>();

        let destination_crate = self.0.get_mut(&to).unwrap();

        piles.append(destination_crate);

        self.0.insert(to, piles);
    }

    fn crates_on_top(&mut self) -> String {
        self.0
            .clone()
            .keys()
            .sorted()
            .into_iter()
            .map(|key| self.0.get_mut(key).unwrap().pop_front().unwrap())
            .collect()
    }
}

pub fn solve_part1(input: &str) -> String {
    let (diagram, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = diagram.parse::<Stacks>().unwrap();

    moves
        .lines()
        .for_each(|mov| stacks.move_crates_9000(mov.parse().unwrap()));

    stacks.crates_on_top()
}

pub fn solve_part2(input: &str) -> String {
    let (diagram, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = diagram.parse::<Stacks>().unwrap();

    moves
        .lines()
        .for_each(|mov| stacks.move_crates_9001(mov.parse().unwrap()));

    stacks.crates_on_top()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
