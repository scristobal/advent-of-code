/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use itertools::Itertools;

fn parse(input: &'static str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|str| str.parse().unwrap()).collect())
        .collect()
}

pub fn solve_part1(input: &'static str) -> Result<String, anyhow::Error> {
    let measures = parse(input);

    let result = measures
        .into_iter()
        .map(|measure| {
            let mut subsequences = vec![measure];

            while !subsequences.last().unwrap().iter().all_equal() {
                let subsequence = subsequences
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();

                subsequences.push(subsequence);
            }

            subsequences
                .iter()
                .rev()
                .fold(0, |acc, x| acc + (x.last().unwrap()))
        })
        .sum::<i32>();

    Ok(result.to_string())
}

pub fn solve_part2(input: &'static str) -> Result<String, anyhow::Error> {
    let measures = parse(input);

    let result = measures
        .into_iter()
        .map(|measure| {
            let mut subsequences = vec![measure];

            while !subsequences.last().unwrap().iter().all_equal() {
                let subsequence = subsequences
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();

                subsequences.push(subsequence);
            }

            subsequences
                .iter()
                .rev()
                .fold(0, |acc, x| (x.first().unwrap()) - acc)
        })
        .sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn parse_sample() {
        let result = parse(SAMPLE);

        assert_eq!(
            result,
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45]
            ]
        );
    }

    #[test]
    fn part1_works() {
        let result = solve_part1(SAMPLE).unwrap();
        assert_eq!(result, "114");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(SAMPLE).unwrap();
        assert_eq!(result, "2");
    }
}
