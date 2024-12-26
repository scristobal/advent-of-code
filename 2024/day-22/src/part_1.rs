use anyhow::Result;

fn mix(value: u128, secret: u128) -> u128 {
    value ^ secret
}

fn prune(secret: u128) -> u128 {
    secret.rem_euclid(16777216)
}

fn generate_secret_num(mut secret: u128, iterations: usize) -> u128 {
    for _ in 0..iterations {
        secret = prune(mix(secret * 64, secret));
        secret = prune(mix(secret / 32, secret));
        secret = prune(mix(secret * 2048, secret));
    }
    secret
}

pub fn solve(input: &'static str) -> Result<String> {
    let iterations = 2000;

    let mut res = 0;
    for seed in input.lines().map(|l| l.parse::<u128>().unwrap()) {
        res += generate_secret_num(seed, iterations);
    }

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
10
100
2024
").unwrap();

        assert_eq!(result, "37327623");
    }
}
