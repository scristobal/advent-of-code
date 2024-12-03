use anyhow::Result;

pub fn solve(input: &str) -> Result<String> {
    let mut num_safe = 0;

    for line in input.lines() {
        let deltas: Vec<_> = line
            .split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .map_windows(|[x, y]| (x - y))
            .collect();

        if deltas.iter().all(|&val| 1 <= val && val <= 3)
            || deltas.iter().all(|&val| -1 >= val && val >= -3)
        {
            num_safe += 1
        }
    }

    Ok(num_safe.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "2");
    }
}
