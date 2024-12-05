use anyhow::Result;
use nom::branch::alt;
use nom::character::complete::{anychar, u32};
use nom::combinator::map;
use nom::multi::many_till;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, sequence::delimited, IResult};

enum Op {
    Mult(u32),
    Do,
    Dont,
}

fn parse_mult(input: &str) -> IResult<&str, Op> {
    map(
        delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")")),
        |(a, b)| Op::Mult(a * b),
    )(input)
}

fn parse_do(input: &str) -> IResult<&str, Op> {
    map(tag("do()"), |_| Op::Do)(input)
}

fn parse_dont(input: &str) -> IResult<&str, Op> {
    map(tag("don't()"), |_| Op::Dont)(input)
}

fn parse_preceded_op(input: &str) -> IResult<&str, Op> {
    map(
        many_till(anychar, alt((parse_mult, parse_do, parse_dont))),
        |(_, m)| m,
    )(input)
}

pub fn solve(mut input: &'static str) -> Result<String> {
    let mut enable = true;
    let mut res = 0;

    while let Ok((rem, op)) = parse_preceded_op(input) {
        match op {
            Op::Mult(m) if enable => res += m,
            Op::Do => enable = true,
            Op::Dont => enable = false,
            _ => {}
        }

        input = rem
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "48");
    }
}
