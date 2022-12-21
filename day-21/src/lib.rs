use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, anychar, newline},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Con,
}

impl From<char> for Op {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            _ => unreachable!("Unknown operation: {}", value),
        }
    }
}

use Op::*;

#[derive(Debug)]
struct Calculation<'a> {
    operation: Op,
    a: Option<i64>,
    b: Option<i64>,
    result: Option<i64>,
    ref_a: Option<&'a str>,
    ref_b: Option<&'a str>,
}

impl<'a> Calculation<'a> {
    fn from_value(val: i64) -> Self {
        Self {
            operation: Con,
            a: None,
            b: None,
            result: Some(val),
            ref_a: None,
            ref_b: None,
        }
    }
    fn from_refs(op: Op, ref_a: &'a str, ref_b: &'a str) -> Self {
        Self {
            operation: op,
            a: None,
            b: None,
            result: None,
            ref_a: Some(ref_a),
            ref_b: Some(ref_b),
        }
    }

    fn compute(&mut self) -> Option<i64> {
        if self.result.is_some() {
            return self.result;
        }
        if self.a.is_none() || self.b.is_none() {
            return None;
        }
        self.result = Some(match self.operation {
            Add => self.a.unwrap() + self.b.unwrap(),
            Sub => self.a.unwrap() - self.b.unwrap(),
            Mul => self.a.unwrap() * self.b.unwrap(),
            Div => self.a.unwrap() / self.b.unwrap(),
            Con => self.result.unwrap(),
        });
        self.result
    }
}

#[derive(Debug)]
struct Call<'a> {
    caller: &'a str,
    receiver: &'a str,
}

// eg. vtww: 3
fn value(s: &str) -> IResult<&str, (&str, Calculation)> {
    let (s, (id, val)) = separated_pair(alpha1, tag(": "), complete::i64)(s)?;
    Ok((s, (id, Calculation::from_value(val))))
}

// eg. jzvz: hhgs + dpzm
fn operation(s: &str) -> IResult<&str, (&str, Calculation)> {
    let (s, id) = terminated(alpha1, tag(": "))(s)?;
    let (s, (ref_a, op, ref_b)) =
        tuple((alpha1, delimited(tag(" "), anychar, tag(" ")), alpha1))(s)?;
    Ok((s, (id, Calculation::from_refs(Op::from(op), ref_a, ref_b))))
}

fn monkey(s: &str) -> IResult<&str, (&str, Calculation)> {
    alt((operation, value))(s)
}

fn parse(s: &str) -> IResult<&str, HashMap<&str, Calculation>> {
    let (_, monkeys) = separated_list0(newline, monkey)(s)?;
    Ok((s, monkeys.into_iter().collect()))
}

pub fn solve_part1(input: &str) -> String {
    let (_, mut monkeys) = parse(input).unwrap();

    let root = monkeys.get("root").unwrap();

    let mut stack = vec![
        Call {
            caller: "root",
            receiver: root.ref_a.unwrap(),
        },
        Call {
            caller: "root",
            receiver: root.ref_b.unwrap(),
        },
    ];

    while let Some(call) = stack.pop() {
        let Call { caller, receiver } = call;

        let receiver_calc = monkeys.get_mut(receiver).unwrap();

        if let Some(result) = receiver_calc.compute() {
            let caller = monkeys.get_mut(caller).unwrap();

            if caller.ref_a.unwrap() == receiver {
                caller.a = Some(result);
            } else {
                caller.b = Some(result);
            };
        } else {
            stack.push(Call { caller, receiver });

            if receiver_calc.a.is_none() {
                stack.push(Call {
                    caller: receiver,
                    receiver: receiver_calc.ref_a.unwrap(),
                });
            }
            if receiver_calc.b.is_none() {
                stack.push(Call {
                    caller: receiver,
                    receiver: receiver_calc.ref_b.unwrap(),
                })
            }
        }
    }

    let root = monkeys.get_mut("root").unwrap();

    root.compute().unwrap().to_string()
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
        assert_eq!(result, "152");
    }

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "");
    }
}
