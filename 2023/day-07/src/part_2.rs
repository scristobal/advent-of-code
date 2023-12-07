/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Set([Card; 5]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard(Set),
    OnePair(Set),
    TwoPair(Set),
    ThreeOfAKind(Set),
    FullHouse(Set),
    FourOfAKind(Set),
    FiveOfAKind(Set),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Play {
    hand: Hand,
    bid: u32,
}

impl Hand {
    fn from(cards: [char; 5]) -> Self {
        let cards = cards
            .iter()
            .map(|&ch| match ch {
                'J' => Card::Joker,
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                c => unreachable!("Unexpected card: {:?}", c),
            })
            .collect::<Vec<Card>>();

        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        let mut counts = [0; 12];
        let mut jokers = 0;

        for card in cards.iter().filter_map(|&ch| match ch {
            Card::Joker => {
                jokers += 1;
                None
            }
            _ => Some(ch),
        }) {
            let index = match card {
                Card::Two => 0,
                Card::Three => 1,
                Card::Four => 2,
                Card::Five => 3,
                Card::Six => 4,
                Card::Seven => 5,
                Card::Eight => 6,
                Card::Nine => 7,
                Card::Ten => 8,
                Card::Queen => 9,
                Card::King => 10,
                Card::Ace => 11,

                c => unreachable!("Unexpected card: {:?}", c),
            };

            counts[index] += 1;
        }

        let mut counts = counts.to_vec();
        counts.sort();

        counts[11] += jokers;

        match counts.as_slice() {
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1] => Hand::HighCard(Set(cards)),
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2] => Hand::OnePair(Set(cards)),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2] => Hand::TwoPair(Set(cards)),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 3] => Hand::ThreeOfAKind(Set(cards)),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4] => Hand::FourOfAKind(Set(cards)),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3] => Hand::FullHouse(Set(cards)),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5] => Hand::FiveOfAKind(Set(cards)),
            d => unreachable!("Unexpected hand: {:?}", d),
        }
    }
}

impl Play {
    fn new(cards: [char; 5], bid: u32) -> Self {
        Self {
            hand: Hand::from(cards),
            bid,
        }
    }
}

fn parse(input: &'static str) -> Vec<Play> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();

            let chars = line.next().unwrap().chars().take(5).collect::<Vec<char>>();
            let cards = [chars[0], chars[1], chars[2], chars[3], chars[4]];

            let bid = line.next().unwrap().parse::<u32>().unwrap();

            Play::new(cards, bid)
        })
        .collect()
}

pub fn solve_part2(input: &'static str) -> Result<String, anyhow::Error> {
    let mut game = parse(input);

    game.sort();

    let total = game
        .iter()
        .enumerate()
        .fold(0, |acc, (index, item)| acc + (index + 1) as u32 * item.bid);

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "32T3K 765
  T55J5 684
  KK677 28
  KTJJT 220
  QQQJA 483";

    #[test]
    fn parse_sample() {
        let result = parse(SAMPLE);

        let game = vec![
            Play {
                hand: Hand::OnePair(Set([
                    Card::Three,
                    Card::Two,
                    Card::Ten,
                    Card::Three,
                    Card::King,
                ])),
                bid: 765,
            },
            Play {
                hand: Hand::FourOfAKind(Set([
                    Card::Ten,
                    Card::Five,
                    Card::Five,
                    Card::Joker,
                    Card::Five,
                ])),
                bid: 684,
            },
            Play {
                hand: Hand::TwoPair(Set([
                    Card::King,
                    Card::King,
                    Card::Six,
                    Card::Seven,
                    Card::Seven,
                ])),
                bid: 28,
            },
            Play {
                hand: Hand::FourOfAKind(Set([
                    Card::King,
                    Card::Ten,
                    Card::Joker,
                    Card::Joker,
                    Card::Ten,
                ])),
                bid: 220,
            },
            Play {
                hand: Hand::FourOfAKind(Set([
                    Card::Queen,
                    Card::Queen,
                    Card::Queen,
                    Card::Joker,
                    Card::Ace,
                ])),
                bid: 483,
            },
        ];

        assert_eq!(result, game)
    }

    #[test]
    fn solve_sample() {
        let result = solve_part2(SAMPLE).unwrap();
        assert_eq!(result, "5905");
    }
}
