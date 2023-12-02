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

#[derive(PartialEq, Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(PartialEq, Debug)]
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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("1 red", Cube::Red(1))]
    fn parse_red_cubes_success(#[case] input: &str, #[case] expected: Cube) {
        let result = red_cubes(input);
        assert!(result.is_ok());

        let (input, cube) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(cube, expected);
    }

    #[rstest]
    #[case("1 red, 2 green, 3 blue", Set {  red: 1, green: 2, blue: 3 })]
    fn parse_set_success(#[case] input: &str, #[case] expected: Set) {
        let result = game_set(input);
        assert!(result.is_ok());

        let (input, set) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(set, expected);
    }

    #[rstest]
    #[case( "Game 12: 1 red, 10 green; 4 red, 6 green, 1 blue",  Game { id: 12, sets: vec![Set {red: 1,green: 10,blue: 0},Set {red: 4,green: 6,blue: 1}]})]
    fn parse_game_success(#[case] input: &str, #[case] expected: Game) {
        let result = game(input);
        assert!(result.is_ok());

        let (input, game) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(game, expected)
    }
}
