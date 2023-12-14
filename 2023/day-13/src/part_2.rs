/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::cmp;

fn solve_one(input: &'static str) -> (usize, bool) {
    let width = input.find('\n').unwrap();
    let height = input.chars().filter(|c| *c == '\n').count() + 1;

    let input = input.replace('\n', "");

    let get_col = |c| input.chars().skip(c).step_by(width).collect::<String>();

    for axis in 1..width {
        let mut differences = 0;

        let low = cmp::max(2 * (axis as i32) - (width as i32), 0) as usize;
        for j in (low..axis).rev() {
            // k = width - 1 -> i + (i - j - 1) = width - 1 ->  2*i - j - 1 = width - 1  -> 2*i - width < j
            let k = axis + (axis - j - 1);

            differences += get_col(j)
                .chars()
                .zip(get_col(k).chars())
                .filter(|(c1, c2)| c1 != c2)
                .count()
        }

        if differences == 1 {
            return (axis, true);
        }
    }

    let get_row = |r: usize| {
        input
            .chars()
            .skip(r * width)
            .take(width)
            .collect::<String>()
    };

    for axis in 1..height {
        let mut differences = 0;

        let low = cmp::max(2 * (axis as i32) - (height as i32), 0) as usize;
        for j in (low..axis).rev() {
            let k = axis + (axis - j - 1);

            differences += get_row(j)
                .chars()
                .zip(get_row(k).chars())
                .filter(|(c1, c2)| c1 != c2)
                .count()
        }
        if differences == 1 {
            return (axis, false);
        }
    }

    unreachable!("is there no symmetry ??")
}

pub fn solve(input: &'static str) -> String {
    input
        .split("\n\n")
        .map(solve_one)
        .map(|(n, v)| if v { n } else { n * 100 })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "400");
    }
}
