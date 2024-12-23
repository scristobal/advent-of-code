use anyhow::Result;
use std::collections::HashSet;

fn is_possible(design: &str, towels: &[&str]) -> bool {
    let mut queue: Vec<String> = vec!["".to_string()];
    let mut visited: HashSet<String> = HashSet::new();

    while let Some(pattern) = queue.pop() {
        if visited.contains(&pattern) {
            continue;
        }

        for towel in towels {
            let mut pattern = pattern.clone();
            pattern.push_str(towel);

            if design == pattern {
                return true;
            };

            if design.starts_with(&pattern) {
                queue.push(pattern);
            }
        }

        visited.insert(pattern);
    }

    false
}

pub fn solve(input: &'static str) -> Result<String> {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels: Vec<_> = towels.split(", ").collect();
    let designs: Vec<_> = designs.lines().collect();

    let num_possible_design = designs
        .iter()
        .filter(|design| is_possible(design, &towels))
        .count();

    Ok(num_possible_design.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        #[rustfmt::skip]
        let result = solve(
"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
").unwrap();

        assert_eq!(result, "6");
    }
}
