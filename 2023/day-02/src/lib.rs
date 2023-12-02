use std::cmp;
use std::error::Error;

use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list0;
use nom::{bytes::complete::tag, sequence::terminated, IResult};
/**
 * Advent of code solutions
 * by Samuel Cristobal
 * https://github.com/scristobal/advent-of-code
 *
 * Licensed under MIT, 2023
 */

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    sets: Vec<Set>,
    id: u32,
}

#[derive(PartialEq, Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

// eg. 1 red
fn red_cubes(input: &str) -> IResult<&str, Cube> {
    terminated(complete::u32, tag(" red"))(input).map(|(input, value)| (input, Cube::Red(value)))
}

// eg. 1 green
fn green_cubes(input: &str) -> IResult<&str, Cube> {
    terminated(complete::u32, tag(" green"))(input)
        .map(|(input, value)| (input, Cube::Green(value)))
}

// eg. 1 blue
fn blue_cubes(input: &str) -> IResult<&str, Cube> {
    terminated(complete::u32, tag(" blue"))(input).map(|(input, value)| (input, Cube::Blue(value)))
}

// eg. 1 red, 2 green, 3 blue (any order)
fn game_set(input: &str) -> IResult<&str, Set> {
    let (input, cubes) =
        separated_list0(tag(", "), alt((blue_cubes, green_cubes, red_cubes)))(input)?;

    let mut set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };

    for cube in cubes {
        match cube {
            Cube::Red(v) => set.red += v,
            Cube::Green(v) => set.green += v,
            Cube::Blue(v) => set.blue += v,
        }
    }

    Ok((input, set))
}

// eg. Game 12: 1 red, 10 green; 4 red, 6 green, 1 blue; 9 green, 1 blue, 7 red; 1 blue, 13 green, 2 red; 2 blue, 5 red, 11 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, sets) = separated_list0(tag("; "), game_set)(input)?;

    Ok((input, Game { sets, id }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list0(line_ending, game)(input)
}

pub fn solve_part1(input: &'static str) -> Result<String, Box<dyn Error>> {
    let (input, games) = parse_input(input)?;
    assert!(input.is_empty() || input == "\n");

    const LIMITS: Set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let possible = games
        .into_iter()
        .filter(|game| {
            !game.sets.iter().any(|set| {
                set.red > LIMITS.red || set.green > LIMITS.green || set.blue > LIMITS.blue
            })
        })
        .map(|game| game.id)
        .sum::<u32>();

    Ok(possible.to_string())
}

pub fn solve_part2(input: &'static str) -> Result<String, Box<dyn Error>> {
    let (input, games) = parse_input(input)?;
    assert!(input.is_empty() || input == "\n");

    let sum_power = games
        .into_iter()
        .map(|game| {
            let empty = Set {
                red: 0,
                green: 0,
                blue: 0,
            };
            game.sets.iter().fold(empty, |acc, set| Set {
                red: cmp::max(acc.red, set.red),
                green: cmp::max(acc.green, set.green),
                blue: cmp::max(acc.blue, set.blue),
            })
        })
        .map(|totals| totals.red * totals.green * totals.blue)
        .sum::<u32>();

    Ok(sum_power.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_red_cubes() {
        let input = "1 red";
        let result = red_cubes(input);
        assert!(result.is_ok());
        let (input, cube) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(cube, Cube::Red(1));
    }

    #[test]
    fn parse_set() {
        let input = "1 red, 2 green, 3 blue";
        let result = game_set(input);
        assert!(result.is_ok());
        let (input, set) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(set.red, 1);
        assert_eq!(set.green, 2);
        assert_eq!(set.blue, 3);
    }

    #[test]
    fn parse_game() {
        let input = "Game 12: 1 red, 10 green; 4 red, 6 green, 1 blue; 9 green, 1 blue, 7 red; 1 blue, 13 green, 2 red; 2 blue, 5 red, 11 green";
        let result = game(input);
        assert!(result.is_ok());
        let (input, game) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(game.id, 12);
        assert_eq!(game.sets.len(), 5);
    }

    const INPUT_1: &str = include_str!("../sample-part-1.txt");
    const ANSWER_1: &str = "8";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT_1).unwrap();
        assert_eq!(result, ANSWER_1)
    }

    const INPUT_2: &str = include_str!("../sample-part-2.txt");
    const ANSWER_2: &str = "2286";

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT_2).unwrap();
        assert_eq!(result, ANSWER_2);
    }
}
