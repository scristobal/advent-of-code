use anyhow::Result;
use std::collections::HashMap;

fn update_one(val: u64) -> (u64, Option<u64>) {
    if val == 0 {
        return (1, None);
    };

    let digits = val.ilog10() + 1;

    if digits % 2 == 0 {
        let m = 10_u64.pow(digits / 2);
        return (val % m, Some(val / m));
    }

    (val * 2024, None)
}

fn update(rocks: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut rocks_swap = HashMap::<u64, u64>::with_capacity(rocks.capacity());

    for (rock, amount) in rocks {
        let (a, b) = update_one(rock);

        rocks_swap
            .entry(a)
            .and_modify(|e| *e += amount)
            .or_insert(amount);

        if let Some(b) = b {
            rocks_swap
                .entry(b)
                .and_modify(|e| *e += amount)
                .or_insert(amount);
        }
    }

    rocks_swap
}

fn solve_interations(input: &'static str, iterations: usize) -> Result<String> {
    let mut rocks = input
        .replace("\n", "")
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .map(|v| (v, 1))
        .collect::<HashMap<u64, u64>>();

    for _ in 0..iterations {
        rocks = update(rocks)
    }

    Ok(rocks.values().sum::<u64>().to_string())
}

pub fn solve(input: &'static str) -> Result<String> {
    solve_interations(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "125 17";

    #[test]
    fn solve_sample() {
        let result = solve_interations(SAMPLE, 25).unwrap();
        assert_eq!(result, "55312");
    }

    #[test]
    fn solve_sort() {
        let result = solve_interations(SAMPLE, 6).unwrap();
        assert_eq!(result, "22");
    }

    #[test]
    fn solve_sort2() {
        let result = solve_interations("0 1 10 99 999", 1).unwrap();
        assert_eq!(result, "7");
    }
}
