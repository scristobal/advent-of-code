use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

fn mix(value: i64, secret: i64) -> i64 {
    value ^ secret
}

fn prune(secret: i64) -> i64 {
    secret.rem_euclid(16777216)
}

fn generate_price_seq(mut secret: i64, iterations: usize) -> Vec<i64> {
    let mut prices = vec![secret % 10];

    for _ in 0..iterations {
        secret = prune(mix(secret * 64, secret));
        secret = prune(mix(secret / 32, secret));
        secret = prune(mix(secret * 2048, secret));
        prices.push(secret % 10);
    }

    prices
}

pub fn solve(input: &'static str) -> Result<String> {
    let iterations = 2000;

    let mut winnings = HashMap::<(i64, i64, i64, i64), HashMap<usize, i64>>::new();

    for (id, seed) in input.lines().map(|l| l.parse::<i64>().unwrap()).enumerate() {
        let price_diff = generate_price_seq(seed, iterations);

        let seqs: Vec<(_, _, _, _, _)> = price_diff.into_iter().tuple_windows().collect();

        for seq in seqs {
            winnings
                .entry((seq.1 - seq.0, seq.2 - seq.1, seq.3 - seq.2, seq.4 - seq.3))
                .and_modify(|h| {
                    h.try_insert(id, seq.4).ok();
                })
                .or_insert(HashMap::from([(id, seq.4)]));
        }
    }

    let res: i64 = winnings
        .values()
        .map(|h| h.values().into_iter().sum())
        .max()
        .unwrap();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        #[rustfmt::skip]
        let result = solve(
"1
2
3
2024
").unwrap();

        assert_eq!(result, "23");
    }
}
