use anyhow::Result;
use std::collections::HashMap;

fn num_possibles<'a>(
    design: &'a str,
    towels: &[&str],
    cached: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut sum = 0;

    for towel in towels.iter().filter(|&towel| design.starts_with(towel)) {
        let left_overs = &design[towel.len()..];

        if let Some(cached) = cached.get(left_overs) {
            sum += cached;
        } else {
            let computed = num_possibles(&design[towel.len()..], towels, cached);
            sum += computed;
            cached.insert(left_overs, computed);
        }
    }

    sum
}

pub fn solve(input: &'static str) -> Result<String> {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels: Vec<_> = towels.split(", ").collect();
    let designs: Vec<_> = designs.lines().collect();

    let mut cache = HashMap::new();

    let num_possible_designs: usize = designs
        .iter()
        .map(|design| num_possibles(design, &towels, &mut cache))
        .sum();

    Ok(num_possible_designs.to_string())
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

        assert_eq!(result, "16");
    }
}
