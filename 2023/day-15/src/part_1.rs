/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

fn hash_algorithm(input: &str) -> u32 {
    input
        .as_ascii()
        .unwrap()
        .as_bytes()
        .iter()
        .fold(0_u32, |acc, item| ((acc + (*item as u32)) * 17) % 256)
}

pub fn solve(input: &'static str) -> String {
    let input = input.replace('\n', "");

    input
        .split(',')
        .map(hash_algorithm)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "1320");
    }
}
