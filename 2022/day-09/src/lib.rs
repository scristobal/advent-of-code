use std::collections::HashSet;
use std::fmt::{self, Debug};

use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::newline;
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, multi::separated_list0, IResult};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coords([i32; 2]);

#[derive(Debug)]
struct Rope {
    head: Coords,
    tail: Coords,
}

impl Debug for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Coord")
            .field("x", &self.0[0])
            .field("y", &self.0[1])
            .finish()
    }
}

impl Coords {
    fn mov(&mut self, dir: &Coords) {
        self.0[0] += dir.0[0];
        self.0[1] += dir.0[1];
    }

    fn diff(&self, coord: &Coords) -> Coords {
        Coords([self.0[0] - coord.0[0], self.0[1] - coord.0[1]])
    }

    fn normalize(&mut self) {
        if self.0[0].abs() <= 1 && self.0[1].abs() <= 1 {
            *self = Coords([0, 0]);
        }
        if self.0[0].abs() > 1 {
            self.0[0] /= self.0[0].abs();
        }
        if self.0[1].abs() > 1 {
            self.0[1] /= self.0[1].abs();
        }
    }
}

fn up(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, (_, times)) = separated_pair(tag("U"), tag(" "), complete::i32)(input)?;
    Ok((input, (0..times).map(|_| Coords([0, 1])).collect()))
}

fn down(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, (_, times)) = separated_pair(tag("D"), tag(" "), complete::i32)(input)?;
    Ok((input, (0..times).map(|_| Coords([0, -1])).collect()))
}

fn left(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, (_, times)) = separated_pair(tag("L"), tag(" "), complete::i32)(input)?;
    Ok((input, (0..times).map(|_| Coords([-1, 0])).collect()))
}

fn right(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, (_, times)) = separated_pair(tag("R"), tag(" "), complete::i32)(input)?;
    Ok((input, (0..times).map(|_| Coords([1, 0])).collect()))
}

fn moves(input: &str) -> IResult<&str, Vec<Coords>> {
    let (input, moves) = separated_list0(newline, alt((up, down, right, left)))(input)?;
    Ok((input, moves.into_iter().flatten().collect()))
}

pub fn solve_part1(input: &str) -> String {
    let (_, moves) = moves(input).unwrap();

    moves
        .into_iter()
        .scan(
            Rope {
                head: Coords([0, 0]),
                tail: Coords([0, 0]),
            },
            |state, mov| {
                state.head.mov(&mov);
                let mut diff = state.head.diff(&state.tail);
                diff.normalize();
                state.tail.mov(&diff);
                Some(state.tail)
            },
        )
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

struct LongRope([Coords; 10]);

pub fn solve_part2(input: &str) -> String {
    let (_, moves) = moves(input).unwrap();

    moves
        .into_iter()
        .scan(LongRope([Coords([0, 0]); 10]), |state, mov| {
            state.0[0].mov(&mov);

            for l in 0..(state.0.len() - 1) {
                let mut diff = state.0[l].diff(&state.0[l + 1]);
                diff.normalize();
                state.0[l + 1].mov(&diff);
            }

            Some(state.0[9])
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "13");
    }

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn part2_works_on_input1() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "1");
    }

    #[test]
    fn part2_works_on_input2() {
        let result = solve_part2(INPUT2);
        assert_eq!(result, "36");
    }
}
