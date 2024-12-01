use std::collections::HashMap;
use anyhow::Result;
use scanf::sscanf;

pub fn solve(input: &str) -> Result<String> {
    let mut lhs = Vec::<usize>::with_capacity(input.len());
    let mut rhs = HashMap::<usize, usize>::with_capacity(input.len());

    let mut lh = 0;
    let mut rh = 0;

    for ln in input.lines() {
        sscanf!(ln, "{}  {}", lh, rh)?;

        lhs.push(lh);
        rhs.entry(rh).and_modify(|rh| *rh += 1).or_insert(1);
    }

    let sc = lhs
        .into_iter()
        .map(|lh| lh*rhs.get(&lh).unwrap_or(&0) )
        .sum::<usize>();

    Ok(sc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    // #[ignore = "not implemented"]
    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "31");
    }
}
