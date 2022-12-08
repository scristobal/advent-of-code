use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Tree {
    index: usize,
    size: u32,
}

fn row(m: Vec<u32>, i: usize) -> (Vec<u32>, Vec<u32>) {
    let size = (m.len() as f32).sqrt() as usize;
    let start = i - (i % size);

    let left = m.iter().skip(start).take(i % size).copied().rev().collect();
    let right = m
        .iter()
        .skip(start + i % size + 1)
        .take(size - 1 - i % size)
        .copied()
        .collect();

    (left, right)
}

fn col(m: Vec<u32>, i: usize) -> (Vec<u32>, Vec<u32>) {
    let size = (m.len() as f32).sqrt() as usize;
    let start = i % size;

    let up = m
        .iter()
        .skip(start)
        .step_by(size)
        .take(i / size)
        .rev()
        .copied()
        .collect();

    let down = m
        .iter()
        .skip(i + size)
        .step_by(size)
        .take(size - i / size)
        .copied()
        .collect();

    (up, down)
}

pub fn solve_part1(input: &str) -> String {
    let field = input
        .replace("\n", "")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    field
        .iter()
        .enumerate()
        .filter(|&(t, size)| {
            let (left, right) = row(field.clone(), t);
            let (up, down) = col(field.clone(), t);

            let vis_up = up.iter().filter(|&t| t < size).count() == up.len();
            let vis_down = down.iter().filter(|&t| t < size).count() == down.len();
            let vis_left = left.iter().filter(|&t| t < size).count() == left.len();
            let vis_right = right.iter().filter(|&t| t < size).count() == right.len();

            vis_up || vis_down || vis_left || vis_right
        })
        .count()
        .to_string()
}
pub fn solve_part2(input: &str) -> String {
    let field = input
        .replace("\n", "")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    field
        .iter()
        .enumerate()
        .map(|(t, size)| {
            let (left, right) = row(field.clone(), t);
            let (up, down) = col(field.clone(), t);

            let score_up = up.iter().take_while(|&t| t < size).count() + 1;
            let score_up = cmp::min(score_up, up.len());

            let score_down = down.iter().take_while(|&t| t < size).count() + 1;
            let score_down = cmp::min(score_down, down.len());

            let score_left = left.iter().take_while(|&t| t < size).count() + 1;
            let score_left = cmp::min(score_left, left.len());

            let score_right = right.iter().take_while(|&t| t < size).count() + 1;
            let score_right = cmp::min(score_right, right.len());

            score_up * score_down * score_left * score_right
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "8");
    }
}
