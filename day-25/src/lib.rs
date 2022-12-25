fn snafu_dec(s: &str) -> i64 {
    s.chars()
        .rev()
        .fold((0, 1), |acc, c| {
            let v = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            };

            (v * acc.1 + acc.0, acc.1 * 5)
        })
        .0
}
fn dec_snafu(d: i64) -> String {
    let mut snafu = String::from("");
    let mut rem = d;
    let base = 5;

    while rem > 0 {
        let digit = rem.rem_euclid(base);

        if digit < 3 {
            snafu += &digit.to_string()
        } else {
            match digit {
                3 => {
                    snafu += "=";
                }
                4 => {
                    snafu += "-";
                }
                _ => unreachable!(),
            }

            rem += base - digit;
        }

        rem /= base;
    }

    snafu.chars().rev().collect()
}

pub fn solve_part1(input: &str) -> String {
    let sum: i64 = input.lines().map(snafu_dec).sum();

    assert_eq!(sum, snafu_dec(&dec_snafu(sum)));

    dec_snafu(sum)
}

pub fn solve_part2(input: &str) -> String {
    dbg!(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "2=-1=0");
    }

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "");
    }
}
