/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::combinator::eof;
use nom::multi::many0;
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{bytes::complete::tag, character::complete::u8, sequence::tuple, IResult};

#[derive(PartialEq, Debug, Clone)]
struct Card {
    id: usize,
    winners: Vec<u8>,
    played: Vec<u8>,
}

// eg, "  39 "
fn parse_num(input: &str) -> IResult<&str, u8> {
    delimited(many0(tag(" ")), u8, many0(tag(" ")))(input)
}

// eg. "Card  11:"
fn parse_card_id(input: &str) -> IResult<&str, u8> {
    delimited(tag("Card"), parse_num, tag(":"))(input)
}

// eg.  "  41 92  3 84 69 ""
fn parse_sequence(input: &str) -> IResult<&str, Vec<u8>> {
    many0(parse_num)(input)
}

// eg. "  41 48 83 86 17 | 83 86  6 31 17  9 48 53 "
fn parse_winners_and_played(input: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    separated_pair(parse_sequence, tag("|"), parse_sequence)(input)
}

// eg. "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (id, (winners, played))) = terminated(
        tuple((parse_card_id, parse_winners_and_played)),
        alt((line_ending, eof)),
    )(input)?;

    let card = Card {
        id: id as usize,
        winners,
        played,
    };

    Ok((input, card))
}

// eg. "Card  1: 41 92 73 84 69 | 7  8  1  2  3  4  5  6\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3: 13 32 20 16 61 | 61 30 68 82 17 32 24 19 "
fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    many0(parse_card)(input)
}

pub fn solve_part1(input: &'static str) -> Result<String, anyhow::Error> {
    let (input, cards) = parse_input(input)?;

    assert!(input.is_empty() || input == "\n");

    let score = cards
        .iter()
        .map(|card| {
            card.played
                .iter()
                .filter(|&played| card.winners.contains(played))
                .count()
        })
        .filter(|&matches| (matches > 0))
        .map(|matches| 1 << (matches - 1))
        .sum::<u32>();

    Ok(score.to_string())
}

pub fn solve_part2(input: &'static str) -> Result<String, anyhow::Error> {
    let (input, cards) = parse_input(input)?;

    assert!(input.is_empty() || input == "\n");

    let matches = cards
        .iter()
        .map(|card| {
            card.played
                .iter()
                .filter(|&played| card.winners.contains(played))
                .count()
        })
        .collect::<Vec<_>>();

    let mut num_cards = matches.iter().map(|_| 1).collect::<Vec<_>>();

    for (i, m) in matches.iter().enumerate() {
        for n in i + 1..=i + m {
            num_cards[n] += num_cards[i];
        }
    }

    Ok(num_cards.iter().map(|&n| n as u32).sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn parse_sequence_works() {
        let result = parse_sequence("41 92  3 84 69");
        assert_eq!(result, Ok(("", vec![41, 92, 3, 84, 69])));
    }

    #[test]
    fn parse_card_works() {
        let result = parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(
            result,
            Ok((
                "",
                Card {
                    id: 1,
                    winners: vec![41, 48, 83, 86, 17],
                    played: vec![83, 86, 6, 31, 17, 9, 48, 53],
                }
            ))
        );
    }

    #[test]
    fn parse_input_works() {
        let result = parse_input(
            "Card 1: 41 92 73 84 69 | 7  8  1  2  3  4  5  6
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        );
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Card {
                        id: 1,
                        winners: vec![41, 92, 73, 84, 69],
                        played: vec![7, 8, 1, 2, 3, 4, 5, 6],
                    },
                    Card {
                        id: 2,
                        winners: vec![13, 32, 20, 16, 61],
                        played: vec![61, 30, 68, 82, 17, 32, 24, 19],
                    },
                    Card {
                        id: 3,
                        winners: vec![13, 32, 20, 16, 61],
                        played: vec![61, 30, 68, 82, 17, 32, 24, 19],
                    }
                ]
            ))
        );
    }

    #[test]
    fn part1_works() {
        let result = solve_part1(SAMPLE).unwrap();
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(SAMPLE).unwrap();
        assert_eq!(result, "30");
    }
}
