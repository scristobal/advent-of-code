/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, line_ending, newline, space0, u32},
    combinator::eof,
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

pub mod part_1 {

    use super::*;

    // eg. " 79 "
    pub fn parse_number(input: &str) -> IResult<&str, u32> {
        delimited(space0, u32, space0)(input)
    }

    // eg. "79 14 55 13"
    pub fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
        many0(parse_number)(input)
    }

    // eg. "seeds: 79 14 55 13\n\n"
    pub fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
        delimited(tag("seeds:"), parse_list, tuple((newline, newline)))(input)
    }

    // eg. "50 98 2"
    pub fn parse_triple(input: &str) -> IResult<&str, (u32, u32, u32)> {
        tuple((parse_number, parse_number, parse_number))(input)
    }

    //eg. "0 98 2\n52 50 48\n\n"
    pub fn parse_map(input: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
        terminated(
            many0(terminated(parse_triple, alt((line_ending, eof)))),
            alt((line_ending, eof)),
        )(input)
    }

    // eg. "seed-to-soil map:\n"
    pub fn parse_map_header(input: &str) -> IResult<&str, (&str, &str)> {
        terminated(
            separated_pair(alpha0, tag("-to-"), alpha0),
            tuple((tag(" map:"), line_ending)),
        )(input)
    }

    pub type Seeds = Vec<u32>;
    pub type MapDef = Vec<(u32, u32, u32)>;

    pub fn parse_input(input: &str) -> IResult<&str, (Seeds, Vec<MapDef>)> {
        tuple((parse_seeds, many0(preceded(parse_map_header, parse_map))))(input)
    }

    pub fn map_factory(def: Vec<(u32, u32, u32)>) -> impl Fn(u32) -> u32 {
        move |n| {
            def.iter()
                .filter(|(_, x, y)| x <= &n && n < x + y)
                .map(|(z, x, _)| z + (n - x))
                .min()
                .unwrap_or(n)
        }
    }

    pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
        let (input, result) = parse_input(input)?;
        assert!(input.is_empty());

        let seeds = result.0;

        let maps = result.1.into_iter().map(map_factory).collect::<Vec<_>>();

        let res = seeds
            .into_iter()
            .map(|s| maps.iter().fold(s, |acc, f| f(acc)))
            .min()
            .unwrap();

        Ok(res.to_string())
    }
}

pub mod part_2 {

    use super::*;

    pub type SeedsPairs = Vec<(u32, u32)>;

    pub fn parse_seeds_as_pairs(input: &str) -> IResult<&str, SeedsPairs> {
        (delimited(
            tag("seeds:"),
            many0(tuple((part_1::parse_number, part_1::parse_number))),
            tuple((newline, newline)),
        ))(input)
    }

    type RangeMap = Vec<(u32, u32, u32)>;

    pub fn parse_input(input: &str) -> IResult<&str, (SeedsPairs, Vec<RangeMap>)> {
        tuple((
            parse_seeds_as_pairs,
            many0(preceded(part_1::parse_map_header, part_1::parse_map)),
        ))(input)
    }

    pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
        let (input, result) = parse_input(input)?;
        assert!(input.is_empty());

        let maps = result
            .1
            .into_iter()
            .map(part_1::map_factory)
            .collect::<Vec<_>>();

        let mut min = u32::MAX;

        for seed_range in &result.0 {
            for seed in seed_range.0..(seed_range.0 + seed_range.1) {
                let res = maps.iter().fold(seed, |acc, f| f(acc));
                if res < min {
                    min = res;
                    println!("{} ", min)
                }
            }
        }

        Ok(min.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_works() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";

        let (input, result) = part_1::parse_input(input).unwrap();

        assert!(input.is_empty());

        assert_eq!(result.0, vec![79, 14, 55, 13]);

        let expected = vec![
            vec![(50, 98, 2), (52, 50, 48)],
            vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
        ];
        assert_eq!(result.1, expected)
    }

    #[test]
    fn parser_part_2_works() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";

        let (input, result) = part_2::parse_input(input).unwrap();

        dbg!(input);

        assert!(input.is_empty());

        assert_eq!(result.0, vec![(79, 92), (55, 67)]);

        let expected = vec![
            vec![(50, 98, 2), (52, 50, 48)],
            vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
        ];

        assert_eq!(result.1, expected)
    }

    const SAMPLE: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_works() {
        let result = part_1::solve(SAMPLE).unwrap();
        assert_eq!(result, "35");
    }

    #[test]
    fn part2_works() {
        let result = part_2::solve(SAMPLE).unwrap();
        assert_eq!(result, "46");
    }
}
