use std::collections::{HashMap, HashSet};

fn priority(c: &char) -> usize {
    let ascii_value = *c as usize;

    if c.is_lowercase() {
        ascii_value - 96 // so that "a" == 1
    } else {
        ascii_value - 38 // so that "A" == 27
    }
}

pub fn solve_part1(input: &str) -> String {
    let priorities = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(v, c)| (c, v + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            first
                .chars()
                .collect::<HashSet<char>>()
                .intersection(&second.chars().collect::<HashSet<_>>())
                .map(|char| priorities.get(char).unwrap())
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| {
            let [first, second, third] = lines else { todo!() };

            first
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&second.chars().collect::<HashSet<char>>())
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&third.chars().collect::<HashSet<_>>())
                .map(priority)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "70");
    }
}
