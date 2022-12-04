#![feature(iter_array_chunks)]

use std::ops::RangeInclusive;

fn contained(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a_contains_b(a, b) || a_contains_b(b, a)
}

fn a_contains_b(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.contains(&b.start()) && a.contains(&b.end())
}

fn a_overlaps_b(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.end() >= b.start() && a.start() <= b.end()
}

fn overlap(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a_overlaps_b(a, b) || a_overlaps_b(b, a)
}

pub fn solve_part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let (first, second) = line.split_once(",").unwrap();

            let (start_first, end_first) = first.split_once("-").unwrap();

            let (start_second, end_second) = second.split_once("-").unwrap();

            let first = start_first.parse::<usize>().unwrap()..=end_first.parse::<usize>().unwrap();
            let second =
                start_second.parse::<usize>().unwrap()..=end_second.parse::<usize>().unwrap();

            contained(&first, &second)
        })
        .count()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let (first, second) = line.split_once(",").unwrap();

            let (start_first, end_first) = first.split_once("-").unwrap();

            let (start_second, end_second) = second.split_once("-").unwrap();

            let first = start_first.parse::<usize>().unwrap()..=end_first.parse::<usize>().unwrap();
            let second =
                start_second.parse::<usize>().unwrap()..=end_second.parse::<usize>().unwrap();

            overlap(&first, &second)
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "4");
    }
}
