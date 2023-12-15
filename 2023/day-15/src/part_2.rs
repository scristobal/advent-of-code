/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use core::num;
use std::collections::{HashMap, VecDeque};
use std::hash::{BuildHasherDefault, Hasher};

use indexmap::IndexMap;

enum Instruction {
    Removal(String),
    Insert(String, usize),
}

#[derive(Default)]
struct BoxNum(usize);

impl Hasher for BoxNum {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        if bytes.len() == 1 && bytes[0] == 255 {
            return;
        }

        self.0 = bytes[0] as usize;
    }
}

// eg. `rn=1` or `cm-`
fn parse_instruction(input: &str) -> Instruction {
    match input.split_once('-') {
        Some((label, _)) => Instruction::Removal(label.to_string()),
        None => match input.split_once('=') {
            Some((label, focus)) => Instruction::Insert(
                label.to_string(),
                focus
                    .parse()
                    .expect("unexpected instruction, `=` not followed by a number"),
            ),
            None => todo!(),
        },
    }
}

fn hash_algorithm(input: &str) -> usize {
    input
        .as_ascii()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|v| *v as usize)
        .fold(0, |acc, item| ((acc + item) * 17) % 256)
}

fn focusing_power(num_box: usize, e: IndexMap<String, usize>) -> usize {
    let values = e.values();

    values
        .enumerate()
        .map(|(i, v)| (num_box + 1) * (i + 1) * v)
        .sum()
}

pub fn solve(input: &'static str) -> String {
    let input = input.replace('\n', "");

    let mut instructions: VecDeque<_> = input.split(',').map(parse_instruction).collect();
    let mut boxes =
        HashMap::<usize, IndexMap<String, usize>, BuildHasherDefault<BoxNum>>::default();

    while let Some(instruction) = instructions.pop_front() {
        match instruction {
            Instruction::Removal(label) => {
                let hash = hash_algorithm(&label);

                boxes
                    .entry(hash)
                    .and_modify(|e: &mut IndexMap<String, usize>| {
                        e.shift_remove(&label);
                    });
            }
            Instruction::Insert(label, focus) => {
                let hash = hash_algorithm(&label);
                boxes
                    .entry(hash)
                    .and_modify(|e: &mut IndexMap<String, usize>| {
                        e.insert(label.clone(), focus);
                    })
                    .or_insert(IndexMap::from([(label, focus)]));
            }
        }
    }

    boxes
        .into_iter()
        .map(|(k, v)| focusing_power(k, v))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "145");
    }

    #[test]
    fn boxes_sample() {
        assert_eq!(hash_algorithm("rn"), 0);
        assert_eq!(hash_algorithm("pc"), 3);
        assert_eq!(hash_algorithm("cm"), 0);
        assert_eq!(hash_algorithm("qp"), 1);
    }
}
