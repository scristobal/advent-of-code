use anyhow::Result;

fn update(val: &mut u64) -> Option<u64> {
    if *val == 0 {
        *val = 1;
        return None;
    };

    let digits = val.ilog10() + 1;

    if digits % 2 == 0 {
        let m = 10_u64.pow(digits / 2);

        let new_val = *val / m;

        *val %= m;
        return Some(new_val);
    }

    *val *= 2024;
    None
}

fn solve_interations(input: &'static str, iterations: usize) -> Result<String> {
    let mut rocks = input
        .replace("\n", "")
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u64>>();

    let mut new_rocks = Vec::with_capacity(rocks.len());

    for _ in 0..iterations {
        new_rocks.clear();

        rocks.iter_mut().for_each(|rock| {
            if let Some(new_rock) = update(rock) {
                new_rocks.push(new_rock);
            }
        });

        rocks.append(&mut new_rocks);
    }

    Ok(rocks.len().to_string())
}

pub fn solve(input: &'static str) -> Result<String> {
    solve_interations(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "125 17";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
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
