use std::collections::HashSet;

pub fn solve_part1(input: &str) -> String {
    let a = input.chars().collect::<Vec<_>>();

    let len = 4;

    let b = a
        .as_slice()
        .windows(len)
        .enumerate()
        .find(|(_, w)| validate_marker(w))
        .unwrap();

    (b.0 + len).to_string()
}

fn validate_marker(m: &[char]) -> bool {
    let mut chs = HashSet::new(); // slow and expensive, but safe and easy

    for c in m {
        if !chs.insert(c) {
            return false;
        }
    }

    true
}

pub fn solve_part2(input: &str) -> String {
    let len = 14;

    let b = input
        .as_bytes()
        .windows(len)
        .enumerate()
        .find(|(_, w)| w.into_iter().collect::<HashSet<_>>().len() == w.len())
        .unwrap();

    (b.0 + len).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "10");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "29");
    }
}
