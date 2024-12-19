use anyhow::Result;
use std::collections::HashMap;

struct State {
    acc: u64,
    ind: usize,
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut result = 0;

    let mut num_digits_cache = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(": ");

        let target: u64 = parts.next().unwrap().parse().unwrap();
        let values: Vec<u64> = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();

        let mut stack = vec![State {
            acc: values[0],
            ind: 1,
        }];

        while let Some(state) = stack.pop() {
            if state.ind == values.len() && state.acc == target {
                result += target;
                break;
            }

            if state.ind >= values.len() {
                continue;
            }

            if (state.acc + values[state.ind]) <= target {
                stack.push(State {
                    acc: state.acc + values[state.ind],
                    ind: state.ind + 1,
                })
            }

            if (state.acc * values[state.ind]) <= target {
                stack.push(State {
                    acc: state.acc * values[state.ind],
                    ind: state.ind + 1,
                })
            }

            let digits = *num_digits_cache
                .entry(values[state.ind])
                .or_insert_with(|| {
                    let mut digits = 0;
                    while 10_u64.pow(digits) <= values[state.ind] {
                        digits += 1;
                    }
                    digits
                });

            if (state.acc * 10_u64.pow(digits) + values[state.ind]) <= target {
                stack.push(State {
                    acc: state.acc * 10_u64.pow(digits) + values[state.ind],
                    ind: state.ind + 1,
                })
            }
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "11387");
    }
}
