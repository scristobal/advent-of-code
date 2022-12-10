#![feature(iter_array_chunks)]

use std::iter::zip;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;

use nom::sequence::separated_pair;
use nom::{multi::separated_list1, IResult};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    tag("noop")(input).map(|(input, _)| (input, Instruction::Noop))
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    separated_pair(tag("addx"), tag(" "), complete::i32)(input)
        .map(|(input, (_, val))| (input, Instruction::Addx(val)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, alt((noop, addx)))(input)
}

fn register_values(instructions: Vec<Instruction>) -> Vec<(u32, i32)> {
    instructions
        .iter()
        .scan([1, 1], |[current, next], instruction| {
            *current = *next;
            match *instruction {
                Instruction::Noop => Some(vec![(*current, instruction)]),
                Instruction::Addx(val) => {
                    *next = *current + val;
                    Some(vec![(*current, instruction); 2])
                }
            }
        })
        .flatten()
        .enumerate()
        .map(|(cycle, (val, _))| ((cycle + 1) as u32, val))
        .collect()
}

pub fn solve_part1(input: &str) -> String {
    let (_, instructions) = parse(input).unwrap();

    register_values(instructions)
        .iter()
        .skip(19)
        .step_by(40)
        .fold(0, |power, (cycle, val)| power + *cycle as i32 * *val)
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (_, instructions) = parse(input).unwrap();

    const SCAN_LINE_LEN: usize = 40;
    const NUM_SCAN_LINES: usize = 6;

    zip(
        0..(SCAN_LINE_LEN * NUM_SCAN_LINES),
        register_values(instructions).iter().skip(0),
    )
    .map(|(pixel, reg)| (pixel % SCAN_LINE_LEN, reg))
    .map(|(pixel, (_, x_reg))| {
        if *x_reg - 1 <= pixel as i32 && pixel as i32 <= *x_reg + 1 {
            '#'
        } else {
            '.'
        }
    })
    .array_chunks::<SCAN_LINE_LEN>()
    .flat_map(|line| line.into_iter().chain(['\n']))
    .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "noop
addx 3
addx -5";

    const INPUT: &str = include_str!("../sample.txt");

    #[ignore = "incomplete input"]
    #[test]
    fn part1_sample_works() {
        let result = solve_part1(SAMPLE);
        assert_eq!(result, "");
    }

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "13140");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
