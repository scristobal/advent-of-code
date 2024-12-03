use anyhow::Result;

pub fn solve(input: &str) -> Result<String> {
    let mut num_safe = 0;

    for line in input.lines() {
        let series: Vec<_> = line
            .split_whitespace()
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        if is_valid(&series) {
            num_safe += 1;
            continue;
        }

        for i in 0..series.len() {
            let mut series = series.clone();

            series.remove(i);

            if is_valid(&series) {
                num_safe += 1;
                break;
            }
        }
    }

    Ok(num_safe.to_string())
}

fn is_valid(series: &[i32]) -> bool {
    let deltas: Vec<_> = series.iter().map_windows(|[x, y]| (**x - **y)).collect();

    deltas.iter().all(|&val| 1 <= val && val <= 3)
        || deltas.iter().all(|&val| -1 >= val && val >= -3)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "4");
    }
}
