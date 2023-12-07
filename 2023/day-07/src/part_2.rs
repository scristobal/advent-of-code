/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

#[derive(Debug, Eq)]
struct Set([char; 5]);

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
        let mut counts = [0; 12];
        let mut jokers = 0;

        for card in cards.iter().filter_map(|&ch| match ch {
            'J' => {
                jokers += 1;
                None
            }
            _ => Some(ch),
        }) {
            let index = match card {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'Q' => 9,
                'K' => 10,
                'A' => 11,
                c => unreachable!("Unexpected card: {}", c),
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

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Set {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let order = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];

        for (self_card, other_card) in self.0.iter().zip(other.0.iter()) {
            let self_index = order.iter().position(|&c| c == *self_card).unwrap();
            let other_index = order.iter().position(|&c| c == *other_card).unwrap();

            match self_index.cmp(&other_index) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
                hand: Hand::OnePair(Set(['3', '2', 'T', '3', 'K'])),
                bid: 765,
            },
            Play {
                hand: Hand::FourOfAKind(Set(['T', '5', '5', 'J', '5'])),
                bid: 684,
            },
            Play {
                hand: Hand::TwoPair(Set(['K', 'K', '6', '7', '7'])),
                bid: 28,
            },
            Play {
                hand: Hand::FourOfAKind(Set(['K', 'T', 'J', 'J', 'T'])),
                bid: 220,
            },
            Play {
                hand: Hand::FourOfAKind(Set(['Q', 'Q', 'Q', 'J', 'A'])),
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
