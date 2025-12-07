use ahash::{HashMap, HashMapExt};

fn main() {
    let s = include_str!("../../input/2025/day6.txt");

    println!("part 1: {}", solve_p1::<4>(s));
    println!("part 2: {}", solve_p2::<4>(s));
}

fn solve_p1<const N: usize>(s: &str) -> u128 {
    let ops = s
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .collect::<HashMap<usize, &str>>();

    let mut accs = HashMap::<usize, u128>::with_capacity(ops.len());

    for line in s.lines().take(N) {
        for (i, num) in line.split_whitespace().enumerate() {
            accs.entry(i)
                .and_modify(|v| {
                    let op = ops.get(&i).unwrap();
                    *v = match *op {
                        "*" => *v * num.parse::<u128>().unwrap(),
                        "+" => *v + num.parse::<u128>().unwrap(),
                        _ => unreachable!(),
                    };
                })
                .or_insert(num.parse().unwrap());
        }
    }

    accs.values().sum()
}

fn solve_p2<const N: usize>(s: &str) -> u128 {
    let ops = s
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .collect::<HashMap<usize, &str>>();

    let mut data = HashMap::<usize, [char; N]>::with_capacity(ops.len());

    for (j, line) in s.lines().take(N).enumerate() {
        for (i, char) in line.chars().enumerate() {
            data.entry(i).and_modify(|v| v[j] = char).or_insert({
                let mut new = [' '; N];
                new[0] = char;
                new
            });
        }
    }

    let mut separators = data
        .iter()
        .filter(|(_, v)| v.iter().all(|c| c.is_whitespace()))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    separators.sort();

    let mut cols = HashMap::<usize, [Vec<Option<u8>>; N]>::with_capacity(ops.len());

    for (j, line) in s.lines().take(N).enumerate() {
        for (i, char) in line.chars().enumerate() {
            if separators.contains(&&i) {
                continue;
            }
            let k = separators
                .iter()
                .position(|&s| &i <= s)
                .unwrap_or(ops.len() - 1);
            cols.entry(k)
                .and_modify(|rows| {
                    rows[j].push(char.to_digit(10).map(|d| d as u8));
                })
                .or_insert({
                    let mut empty = [const { Vec::new() }; N];
                    empty[j].push(char.to_digit(10).map(|d| d as u8));
                    empty
                });
        }
    }

    let mut accs = HashMap::<usize, u128>::with_capacity(ops.len());

    for (i, col) in cols {
        let col_size = col.iter().map(|v| v.len()).max().unwrap_or(0);

        for j in 0..col_size {
            let mut acc = 0;

            for row in &col {
                if let Some(Some(d)) = row.get(j) {
                    acc *= 10;
                    acc += *d as u32;
                }
            }

            accs.entry(i)
                .and_modify(|v| {
                    let op = ops.get(&i).unwrap();
                    *v = match *op {
                        "*" => *v * acc as u128,
                        "+" => *v + acc as u128,
                        _ => unreachable!(),
                    };
                })
                .or_insert(acc as u128);
        }
    }

    accs.values().sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn sample_p1() {
        assert_eq!(solve_p1::<3>(SAMPLE), 4277556)
    }

    #[test]
    fn sample_p2() {
        assert_eq!(solve_p2::<3>(SAMPLE), 3263827)
    }
}
