use anyhow::Result;
use nom::character::complete::{anychar, u32};
use nom::combinator::map;
use nom::multi::{many0, many_till};
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, sequence::delimited, IResult};

fn parse_mult(input: &str) -> IResult<&str, u32> {
    map(
        delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")")),
        |(a, b)| a * b,
    )(input)
}

fn parse_preceded_mult(input: &str) -> IResult<&str, u32> {
    map(many_till(anychar, parse_mult), |(_, m)| m)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    many0(parse_preceded_mult)(input)
}

pub fn solve(input: &str) -> Result<String> {
    let res: u32 = input
        .lines()
        .map(|line| parse_line(line).unwrap().1.iter().sum::<u32>())
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "161");
    }

    #[test]
    fn parse_single() {
        let r = parse_mult("mul(2,3)").unwrap();
        dbg!(r);
        assert_eq!(r.1, 6)
    }

    #[test]
    fn parse_skip_before() {
        let r = parse_preceded_mult("1)%&*234mul(2,3)").unwrap();
        dbg!("r", r);
        assert_eq!(r.1, 6)
    }
}
