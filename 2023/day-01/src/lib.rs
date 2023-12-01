/**
 * Advent of code
 */
use std::cmp;

pub fn solve_part1(input: &str) -> Result<String, anyhow::Error> {
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .fold((None, None), |(first, last), l| match l.to_digit(10) {
                    Some(l) => match (first, last) {
                        (None, None) => (Some(l), Some(l)),
                        (_, _) => (first, Some(l)),
                    },
                    None => (first, last),
                })
        })
        .map(|(a, b)| a.unwrap() * 10 + b.unwrap())
        .sum::<u32>()
        .to_string())
}

fn literal_strings(n: u32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => unimplemented!(),
    }
}

fn find_first(input: &str, number: u32) -> Option<usize> {
    match (
        input.find(literal_strings(number)),
        input.find(&number.to_string()),
    ) {
        (Some(n), Some(m)) => Some(cmp::min(n, m)),
        (None, n) => n,
        (n, None) => n,
    }
}

fn find_last(input: &str, number: u32) -> Option<usize> {
    match (
        input.rfind(literal_strings(number)),
        input.rfind(&number.to_string()),
    ) {
        (Some(n), Some(m)) => Some(cmp::max(n, m)),
        (None, n) => n,
        (n, None) => n,
    }
}

pub fn solve_part2(input: &str) -> Result<String, anyhow::Error> {
    Ok(input
        .lines()
        .map(|line| {
            let mut fist_index = input.len();
            let mut last_index = 0;

            let mut res = (None, None);

            for num in 1..10 {
                let num_ind = find_first(line, num);

                if let Some(u) = num_ind {
                    if u <= fist_index {
                        fist_index = u;
                        res = (Some(num), res.1);
                    }
                }

                let num_ind = find_last(line, num);

                if let Some(u) = num_ind {
                    if u >= last_index {
                        last_index = u;
                        res = (res.0, Some(num));
                    }
                }
            }

            res
        })
        .map(|(a, b)| a.unwrap() * 10 + b.unwrap())
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    const INPUT1: &str = include_str!("../sample.txt");
    const RESULT_1: &str = "142";

    #[test]
    fn part1_works() -> Result<(), Box<dyn Error>> {
        let result = solve_part1(INPUT1)?;
        Ok(assert_eq!(result, RESULT_1))
    }

    const INPUT2: &str = include_str!("../sample2.txt");
    const RESULT_2: &str = "281";

    #[test]
    fn part2_works() -> Result<(), Box<dyn Error>> {
        let result = solve_part2(INPUT2)?;
        Ok(assert_eq!(result, RESULT_2))
    }
}
