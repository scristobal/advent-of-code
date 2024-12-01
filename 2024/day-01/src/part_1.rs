use anyhow::Result;
use scanf::sscanf;
use std::iter::zip;

pub fn solve(input: &str) -> Result<String> {
    let mut lhs = Vec::<i32>::with_capacity(input.len());
    let mut rhs = Vec::<i32>::with_capacity(input.len());

    let mut lh = 0;
    let mut rh = 0;

    for ln in input.lines() {
        sscanf!(ln, "{}  {}", lh, rh)?;

        lhs.push(lh);
        rhs.push(rh);
    }

    lhs.sort();
    rhs.sort();

    let dst = zip(lhs, rhs).map(|(lh, rh)| (lh - rh).abs()).sum::<i32>();

    Ok(dst.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    // #[ignore = "not implemented"]
    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "11");
    }
}
