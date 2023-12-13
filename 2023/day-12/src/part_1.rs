/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::VecDeque;

#[derive(PartialEq, Clone, Debug)]
enum State {
    Unknown, // '?'
    Failure, // '#'
    Correct, // '.'
}

fn parse(input: &str) -> (Vec<State>, VecDeque<usize>) {
    input
        .split_once(' ')
        .map(|(groups, sequence)| {
            (
                groups
                    .chars()
                    .map(|c| match c {
                        '#' => State::Failure,
                        '.' => State::Correct,
                        '?' => State::Unknown,
                        _ => panic!("Invalid character"),
                    })
                    .collect(),
                sequence.split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .unwrap()
}

fn check(sequence: &[State], missing: &VecDeque<usize>) -> bool {
    let Some((mut j, _)) = sequence
        .iter()
        .enumerate()
        .find(|(_, s)| **s == State::Failure)
    else {
        return false;
    };

    for &n in missing {
        let mut count = 0;
        while j < sequence.len() && sequence.get(j) == Some(&State::Failure) {
            j += 1;
            count += 1;
        }
        if count != n {
            return false;
        }

        while j < sequence.len() && sequence.get(j) == Some(&State::Correct) {
            j += 1;
        }
    }

    while j < sequence.len() {
        if sequence.get(j) == Some(&State::Failure) {
            return false;
        }

        j += 1;
    }

    true
}

fn guess(sequence: &[State]) -> Vec<Vec<State>> {
    let Some((j, _)) = sequence
        .iter()
        .enumerate()
        .find(|(_, s)| **s == State::Unknown)
    else {
        return vec![];
    };

    let mut left: Vec<_> = sequence.to_vec();
    left[j] = State::Correct;

    let mut right = sequence.to_vec();
    right[j] = State::Failure;

    vec![left, right]
}

fn solve_sequence(sequence: Vec<State>, missing: VecDeque<usize>) -> usize {
    // let sequence_formatted = format!(
    //     "{}",
    //     sequence
    //         .iter()
    //         .map(|s| match s {
    //             State::Correct => '.',
    //             State::Failure => '#',
    //             State::Unknown => '?',
    //         })
    //         .collect::<String>()
    // );

    // let missing_formatted = format!(
    //     "{}",
    //     missing
    //         .iter()
    //         .map(|n| n.to_string())
    //         .collect::<Vec<_>>()
    //         .join(",")
    // );

    let mut queue = VecDeque::new();

    queue.push_back(sequence);

    let mut count_solutions = 0;

    while let Some(sequence) = queue.pop_back() {
        let more = guess(&sequence);

        if more.is_empty() && check(&sequence, &missing) {
            count_solutions += 1;
        };

        queue.extend(more.into_iter());
    }

    // println!("assert_eq!(solve(\"{sequence_formatted} {missing_formatted}\").unwrap(), \"{count_solutions}\");");

    // println!("#[case(parse(\"{sequence_formatted}\"), {count_solutions})]");

    count_solutions
}

pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
    let records: Vec<_> = input.lines().map(parse).collect();

    let res: usize = records.into_iter().map(|(s, m)| solve_sequence(s, m)).sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_test() {
        assert_eq!(
            guess(&parse("???.### 3").0),
            vec![parse(".??.### 3").0, parse("#??.### 3").0]
        );

        assert_eq!(guess(&parse(".#.### 3").0), Vec::<Vec<State>>::new());

        assert_eq!(
            guess(&parse(".#?# 3").0),
            vec![parse(".#.# 3").0, parse(".### 3").0]
        );
    }

    #[test]
    fn check_test() {
        assert!(check(&parse("###. 3").0, &VecDeque::from(vec![3usize])));
        assert!(!check(&parse("#.## 3").0, &VecDeque::from(vec![3])));
        assert!(check(&parse("#.## 2").0, &VecDeque::from(vec![1, 2])));
    }

    #[test]
    fn single_lines() {
        assert_eq!(solve("???.### 1,1,3").unwrap(), "1");
        assert_eq!(solve(".??..??...?##. 1,1,3").unwrap(), "4");
        assert_eq!(solve("?#?#?#?#?#?#?#? 1,3,1,6").unwrap(), "1");
        assert_eq!(solve("????.#...#... 4,1,1").unwrap(), "1");
        assert_eq!(solve("????.######..#####. 1,6,5").unwrap(), "4");
        assert_eq!(solve("?###???????? 3,2,1").unwrap(), "10");
        assert_eq!(solve("??????##?? 1,4").unwrap(), "12");
    }

    #[test]
    fn solve_sample() {
        let result = solve(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        )
        .unwrap();
        assert_eq!(result, "21");
    }
}
