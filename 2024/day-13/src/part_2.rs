use anyhow::Result;
use scanf::sscanf;

pub fn solve(input: &'static str) -> Result<String> {
    let mut cost = 0;

    for eq in input.split("\n\n") {
        let mut a1 = 0_i64;
        let mut a2 = 0_i64;
        let mut b1 = 0_i64;
        let mut b2 = 0_i64;
        let mut c1 = 0_i64;
        let mut c2 = 0_i64;

        sscanf!(
            &eq,
            "Button A: X+{}, Y+{}
Button B: X+{}, Y+{}
Prize: X={}, Y={}",
            a1,
            a2,
            b1,
            b2,
            c1,
            c2
        )
        .unwrap();

        c1 += 10_000_000_000_000_i64;
        c2 += 10_000_000_000_000_i64;

        let det = a1 * b2 - b1 * a2;

        let mut x = c1 * b2 - b1 * c2;
        let mut y = a1 * c2 - c1 * a2;

        if det != 0 && x % det == 0 && y % det == 0 {
            x /= det;
            y /= det;

            if 0 <= x && 0 <= y {
                cost += 3 * x + y;
            }
        }
    }

    Ok(cost.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "480");
    }
}
