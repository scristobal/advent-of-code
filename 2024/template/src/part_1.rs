use anyhow::Result;

pub fn solve(input: &str) -> Result<String> {
    dbg!(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[ignore = "not implemented"]
    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "");
    }
}
