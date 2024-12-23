use anyhow::Result;

pub fn solve(input: &'static str) -> Result<String> {
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        #[rustfmt::skip]
        let result = solve(
"").unwrap();

        assert_eq!(result, "");
    }
}
