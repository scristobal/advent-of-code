use std::collections::HashSet;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::{self, alphanumeric1};
use nom::multi::many1;
use nom::{IResult, Parser};

fn packet(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((tag("["), tag("]"), tag(","), alphanumeric1)))(input)
}

pub fn solve_part1(input: &str) -> String {
    let input = input
        .split("\n\n")
        .filter_map(|block| block.split_once('\n'))
        .collect::<Vec<_>>();
    let mut hits = HashSet::new();
    let p = input
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .filter_map(|(pair_ind, &(left, right))| {
            let (_, left) = packet(left).unwrap();

            let (_, right) = packet(right).unwrap();

            //dbg!(&left, &right);

            let (mut i, mut j) = (0_usize, 0_usize);

            while i < left.len() && j < right.len() {
                hits.insert((left[i], right[j]));
                match left[i] {
                    "[" => match right[j] {
                        "[" => {
                            j += 1;
                            i += 1;
                        }
                        "]" => {
                            return None;
                        }
                        "," => {
                            j += 1;
                        }
                        _ => {
                            i += 1;
                        }
                    },
                    "]" => match right[j] {
                        "[" => {
                            return Some(pair_ind);
                        }
                        "]" => {
                            j += 1;
                            i += 1;
                        }
                        "," => {
                            j += 1;
                        }
                        _ => {
                            return Some(pair_ind);
                        }
                    },
                    "," => {
                        i += 1;
                    }
                    n => match right[j] {
                        "[" => {
                            j += 1;
                        }
                        "]" => {
                            return None;
                        }
                        "," => {
                            j += 1;
                        }
                        m => {
                            let n = n.parse::<u32>().unwrap();
                            let m = m.parse::<u32>().unwrap();

                            // dbg!(&n, &m);

                            if n < m {
                                return Some(pair_ind);
                            }
                            if n > m {
                                return None;
                            }

                            j += 1;
                            i += 1;
                        }
                    },
                }
            }

            unreachable!();

            if i == left.len() {
                Some(pair_ind)
            } else {
                dbg!("panic");
                None
            }
        })
        .collect::<Vec<_>>();
    dbg!(&hits);
    //dbg!(&p);

    p.iter().sum::<usize>().to_string()
}

pub fn solve_part2(input: &str) -> String {
    dbg!(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "");
    }
}
