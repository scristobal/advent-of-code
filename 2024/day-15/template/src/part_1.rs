use anyhow::Result;

pub fn solve(input: &'static str) -> Result<String> {
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "");
    }
}
