use std::cmp;

fn directions(m: Vec<u32>, i: usize) -> [Vec<u32>; 4] {
    let size = (m.len() as f32).sqrt() as usize;
    let start = i - (i % size);

    let left = m.iter().skip(start).take(i % size).copied().rev().collect();
    let right = m
        .iter()
        .skip(start + i % size + 1)
        .take(size - 1 - i % size)
        .copied()
        .collect();

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

    [left, right, up, down]
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
            directions(field.clone(), t)
                .iter()
                .map(|dir| dir.iter().filter(|&t| t < size).count() == dir.len())
                .reduce(|acc, i| acc || i)
                .unwrap()
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
            directions(field.clone(), t)
                .iter()
                .map(|dir| cmp::min(dir.iter().take_while(|&t| t < size).count() + 1, dir.len()))
                .reduce(|acc, i| acc * i)
        })
        .max()
        .unwrap()
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
