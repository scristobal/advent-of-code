/*
 * Advent of code solutions
 * by Samuel Cristobal
 * https://github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023
 */

#[derive(PartialEq, Debug)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
}

type Numbers = Vec<Number>;
type Symbols = Vec<usize>;

fn parse_input(input: &str) -> (Numbers, Symbols) {
    let res = input.chars().enumerate().fold(
        (vec![], vec![], None::<Number>),
        |(mut numbers, mut symbols, mut partial_number), (i, c)| {
            if c.is_numeric() {
                let digit = c.to_digit(10).unwrap();
                partial_number = match partial_number {
                    Some(mut number) => {
                        number.value = digit + number.value * 10;
                        Some(number)
                    }
                    None => Some(Number {
                        value: digit,
                        start: i,
                        end: i,
                    }),
                }
            } else {
                if let Some(mut number) = partial_number {
                    number.end = i - 1;
                    numbers.push(number);
                    partial_number = None;
                }
                if c != '.' {
                    symbols.push(i);
                }
            }
            (numbers, symbols, partial_number)
        },
    );
    (res.0, res.1)
}

fn adjacent(width: usize) -> Box<dyn Fn(usize) -> Vec<usize>> {
    Box::new(move |i| {
        let mut res = vec![];

        // left
        if i % width != 0 {
            res.push(i - 1);
        }
        // right
        if i % width != width - 1 {
            res.push(i + 1);
        }
        // up
        if i >= width {
            res.push(i - width);
        }
        // down
        if i < width * (width - 1) {
            res.push(i + width);
        }

        // up, left
        if i % width != 0 && i >= width {
            res.push(i - width - 1);
        }
        // up, right
        if i % width != width - 1 && i >= width {
            res.push(i - width + 1);
        }
        // down, left
        if i % width != 0 && i < width * (width - 1) {
            res.push(i + width - 1);
        }
        // down, right
        if i % width != width - 1 && i < width * (width - 1) {
            res.push(i + width + 1);
        }
        res
    })
}

pub fn solve_part1(input: &str) -> Result<String, anyhow::Error> {
    let width = input.find('\n').unwrap();
    let input = input.replace('\n', "");

    let (numbers, symbols) = parse_input(input.as_str());

    let adjacent_fn = adjacent(width);

    Ok(numbers
        .iter()
        .filter_map(|number| {
            symbols
                .iter()
                .flat_map(|&symbol| adjacent_fn(symbol))
                .any(|adj| (number.start <= adj) && (adj <= number.end))
                .then_some(number.value)
        })
        .sum::<u32>()
        .to_string())
}

pub fn solve_part2(input: &str) -> Result<String, anyhow::Error> {
    let width = input.find('\n').unwrap();
    let input = input.replace('\n', "");

    let (numbers, _) = parse_input(input.as_str());

    let adjacent_fn = adjacent(width);

    Ok(input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            let adjacents = adjacent_fn(i);

            let adjacents_parts = numbers.iter().filter(|number| {
                adjacents
                    .iter()
                    .any(|&adj| (number.start <= adj) && (adj <= number.end))
            });

            (c == '*' && adjacents_parts.count() == 2).then_some(adjacents)
        })
        .map(|adjacents| {
            numbers
                .iter()
                .filter_map(|number| {
                    adjacents
                        .iter()
                        .any(|&adj| (number.start <= adj) && (adj <= number.end))
                        .then_some(number.value)
                })
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line() {
        let input = "467.$.114*.";
        let (numbers, symbols) = parse_input(input);

        assert_eq!(numbers.len(), 2);

        assert_eq!(
            numbers[0],
            Number {
                value: 467,
                start: 0,
                end: 2
            }
        );
        assert_eq!(
            numbers[1],
            Number {
                value: 114,
                start: 6,
                end: 8
            }
        );
        assert_eq!(symbols.len(), 2);

        assert_eq!(symbols[0], 4);
        assert_eq!(symbols[1], 9);
    }
}
