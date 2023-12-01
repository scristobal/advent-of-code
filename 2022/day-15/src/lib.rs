use std::collections::{HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
};

use nom::sequence::pair;

use nom::{sequence::preceded, IResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    fn abs(&self, other: Coords) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pair {
    sensor: Coords,
    beacon: Coords,
}

fn coords(s: &str) -> IResult<&str, Coords> {
    let (s, (x, y)) = separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(s)?;

    Ok((s, Coords { x, y }))
}

fn beacon(s: &str) -> IResult<&str, Coords> {
    preceded(tag(": closest beacon is at "), coords)(s)
}

fn sensor(s: &str) -> IResult<&str, Coords> {
    preceded(tag("Sensor at "), coords)(s)
}

fn line(s: &str) -> IResult<&str, Pair> {
    let (s, (sensor, beacon)) = pair(sensor, beacon)(s)?;
    Ok((s, Pair { sensor, beacon }))
}

fn file(s: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(newline, line)(s)
}

pub fn solve_part1(input: &str, row: i64) -> String {
    let (_, map) = file(input).unwrap();

    let b = map
        .iter()
        .filter_map(|Pair { beacon, .. }| if beacon.y == row { Some(beacon) } else { None })
        .collect::<HashSet<_>>()
        .len();

    let mut intervals = Vec::<(i64, i64)>::new();

    for Pair { sensor, beacon } in map.iter() {
        let distance_to_row = (sensor.y - row).abs();

        let distance_to_beacon = sensor.abs(*beacon);

        if distance_to_row > distance_to_beacon {
            continue;
        };

        let dy = distance_to_beacon - distance_to_row;

        let interval = (sensor.x - dy, sensor.x + dy);

        intervals.push(interval)
    }

    intervals.sort();

    let mut intervals = intervals.iter().copied().collect::<VecDeque<_>>();

    let mut stack = vec![intervals.pop_front().unwrap()];

    for interval in intervals {
        let mut top = stack.pop().unwrap();

        if (top.1 + 1) < interval.0 {
            stack.push(interval);
        } else {
            top.1 = top.1.max(interval.1);
            stack.push(top);
        }
    }

    let s = stack
        .iter()
        .fold(0, |acc, interval| acc + interval.1 - interval.0 + 1) as usize;

    (s - b).to_string()
}

pub fn solve_part2(input: &str, size: i64) -> String {
    let (_, map) = file(input).unwrap();

    let beacons_freqs = map
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .iter()
        .map(|Pair { beacon, .. }| beacon.x * size + beacon.y)
        .collect::<Vec<_>>();

    for row in 0..=size {
        let mut intervals = Vec::<(i64, i64)>::new();

        for Pair { sensor, beacon } in map.clone() {
            let distance_to_row = (sensor.y - row).abs();

            let distance_to_beacon = sensor.abs(beacon);

            if distance_to_row > distance_to_beacon {
                continue;
            };

            let dy = distance_to_beacon - distance_to_row;

            let interval = (
                (sensor.x - dy).max(0).min(size),
                (sensor.x + dy).max(0).min(size),
            );

            intervals.push(interval)
        }

        intervals.sort();

        let mut intervals = intervals.iter().copied().collect::<VecDeque<_>>();

        let mut stack = vec![intervals.pop_front().unwrap()];

        for interval in intervals {
            let mut top = stack.pop().unwrap();

            if (top.1 + 1) < interval.0 {
                let freq = (top.1 + 1) * 4_000_000 + row;
                if !beacons_freqs.contains(&freq) {
                    return freq.to_string();
                }
                stack.push(interval);
            } else {
                top.1 = top.1.max(interval.1);
                stack.push(top);
            }
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT, 10);
        assert_eq!(result, "26");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT, 20);
        assert_eq!(result, "56000011");
    }
}
