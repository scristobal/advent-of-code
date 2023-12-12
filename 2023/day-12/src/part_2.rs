/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::VecDeque;

fn parse(input: &str) -> (&str, VecDeque<usize>) {
    input
        .split_once(' ')
        .map(|(groups, sequence)| {
            (
                groups,
                sequence.split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .unwrap()
}

fn check(sequence: &String, missing: &VecDeque<usize>) -> bool {
    let Some(mut j) = sequence.find('#') else {
        return false;
    };

    for &n in missing {
        let mut count = 0;
        while j < sequence.len() && sequence.get(j..=j) == Some("#") {
            j += 1;
            count += 1;
        }
        if count != n {
            return false;
        }

        while j < sequence.len() && sequence.get(j..=j) == Some(".") {
            j += 1;
        }
    }

    while j < sequence.len() {
        if sequence.get(j..=j) == Some("#") {
            return false;
        }

        j += 1;
    }

    true
}

fn guess(sequence: &String) -> Option<(String, String)> {
    let Some(j) = sequence.find('?') else {
        return None;
    };

    let mut left = sequence.clone();
    left.replace_range(j..=j, ".");

    let mut right = sequence.clone();
    right.replace_range(j..=j, "#");

    Some((left, right))
}

fn solve_sequence(sequence: String, missing: VecDeque<usize>) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back(sequence);

    let mut count_solutions = 0;

    while let Some(sequence) = queue.pop_back() {
        match guess(&sequence) {
            None => {
                if check(&sequence, &missing) {
                    count_solutions += 1;
                }
            }
            Some((left, right)) => {
                queue.push_back(left);
                queue.push_back(right);
            }
        }
    }

    count_solutions
}

pub fn solve(input: &'static str) -> Result<String, anyhow::Error> {
    let records: Vec<_> = input.lines().map(parse).collect();

    let res: usize = records
        .into_iter()
        .map(|(s, m)| solve_sequence(s.chars().collect(), m))
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_test() {
        assert_eq!(
            guess(&"???.###".to_string()),
            Some((".??.###".to_string(), "#??.###".to_string()))
        );

        assert_eq!(guess(&".#.###".to_string()), None);

        assert_eq!(
            guess(&".#?#".to_string()),
            Some((".#.#".to_string(), ".###".to_string()))
        );
    }

    #[test]
    fn check_test() {
        assert!(check(&".###".to_string(), &VecDeque::from(vec![3])));
        assert!(!check(&"#.##".to_string(), &VecDeque::from(vec![3])));
        assert!(check(&"#.##".to_string(), &VecDeque::from(vec![1, 2])));
    }

    #[test]
    fn single_lines() {
        assert_eq!(solve("???.### 1,1,3").unwrap(), "1");
        assert_eq!(solve(".??..??...?##. 1,1,3").unwrap(), "16384");
        assert_eq!(solve("?#?#?#?#?#?#?#? 1,3,1,6").unwrap(), "1");
        assert_eq!(solve("????.#...#... 4,1,1").unwrap(), "16");
        assert_eq!(solve("????.######..#####. 1,6,5").unwrap(), "2500");
        assert_eq!(solve("?###???????? 3,2,1").unwrap(), "506250");
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
        assert_eq!(result, "525152");
    }
}
