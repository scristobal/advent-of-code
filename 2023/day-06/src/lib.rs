/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, u64},
    multi::{fold_many0, many0},
    sequence::delimited,
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, u64> {
    delimited(space0, u64, space0)(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<u64>> {
    many0(parse_number)(input)
}

fn parse_times(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("Time:"), parse_sequence, many0(newline))(input)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("Distance:"), parse_sequence, many0(newline))(input)
}

fn parse_input(input: &str) -> IResult<&str, [Vec<u64>; 2]> {
    let (input, times) = parse_times(input)?;
    let (input, distances) = parse_distances(input)?;

    Ok((input, [times, distances]))
}

pub fn solve_part1(input: &'static str) -> Result<String, anyhow::Error> {
    let (input, [times, distances]) = parse_input(input)?;
    assert!(input.is_empty());

    let mut total_wins = 1;

    for (time, distance) in times.iter().zip(distances.iter()) {
        let discriminant = (time.pow(2) - 4 * distance) as f32;

        let min_time = ((*time as f32 - discriminant.sqrt()) / 2_f32) as u64;

        let dist_min = (time - min_time) * min_time;

        let mut num_wins = (time - 2 * (min_time)) + 1;

        if dist_min <= *distance {
            num_wins -= 2;
        }

        total_wins *= num_wins;
    }

    Ok(total_wins.to_string())
}

fn parse_sequence_as_one(input: &str) -> IResult<&str, u64> {
    fold_many0(
        delimited(space0, u64, space0),
        || 0,
        |acc, item| {
            format!("{}{}", acc, item)
                .parse::<u64>()
                .expect("Failed to parse")
        },
    )(input)
}

fn parse_times_as_one(input: &str) -> IResult<&str, u64> {
    delimited(tag("Time:"), parse_sequence_as_one, many0(newline))(input)
}

fn parse_distances_as_one(input: &str) -> IResult<&str, u64> {
    delimited(tag("Distance:"), parse_sequence_as_one, many0(newline))(input)
}

fn parse_input2(input: &str) -> IResult<&str, [u64; 2]> {
    let (input, times) = parse_times_as_one(input)?;
    let (input, distances) = parse_distances_as_one(input)?;

    Ok((input, [times, distances]))
}

pub fn solve_part2(input: &'static str) -> Result<String, anyhow::Error> {
    let (input, [total_time, best_distance]) = parse_input2(input)?;
    assert!(input.is_empty());

    let discriminant = ((total_time.pow(2) - 4 * best_distance) as f32).sqrt();

    let slowest_time = (total_time as f32 - discriminant) / 2_f32;

    let smallest_distance = (total_time as f32 - slowest_time) * slowest_time;

    let mut number_ways_to_win = (total_time as f32 - (2_f32 * slowest_time.ceil()) + 1.0) as u32;

    if smallest_distance.ceil() > best_distance as f32 {
        number_ways_to_win += 2;
    }

    Ok(number_ways_to_win.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn parse_input_works() {
        let result = parse_input(SAMPLE).unwrap();
        assert_eq!(result, ("", [vec![7, 15, 30], vec![9, 40, 200]]));
    }

    #[test]
    fn parse_input_as_one_works() {
        let result = parse_input2(SAMPLE).unwrap();
        assert_eq!(result, ("", [71530, 940200]));
    }

    #[test]
    fn part1_works() {
        let result = solve_part1(SAMPLE).unwrap();
        assert_eq!(result, "288");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(SAMPLE).unwrap();
        assert_eq!(result, "71503");
    }
}
