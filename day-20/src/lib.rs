use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult, Parser,
};

fn mix(list: &Vec<i64>, mixed: &mut Vec<i64>) {
    let len = list.len() as i64;

    for (ind, steps) in list.iter().enumerate() {
        let (ind, _) = mixed
            .iter()
            .enumerate()
            .find(|(_, item)| **item == ind as i64)
            .unwrap();

        let mut ind = ind as i64;

        let step = if *steps > 0 { 1 } else { -1 };

        let mut remaining = steps.abs().rem_euclid(len - 1);

        while remaining > 0 {
            let next = (ind + step).rem_euclid(len);

            if (ind == (len - 1) && next == 0) || ind == 0 && next == (len - 1) {
                remaining += 1;
            }

            let a = mixed.remove(ind as usize);
            mixed.insert(next as usize, a);

            ind = next;
            remaining -= 1;
        }
    }
}

fn demix(mixed: &[i64], list: &[i64]) -> Vec<i64> {
    mixed
        .iter()
        .map(|ind| list[*ind as usize])
        .collect::<Vec<_>>()
}

fn parse(s: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, complete::i64)(s)
}

fn total(list: Vec<i64>, mixed: Vec<i64>) -> i64 {
    let l = list.len();

    let indexes = [1000, 2000, 3000];

    let mut total = 0;

    let t = demix(&mixed, &list);

    let (zero_ind, _) = t.iter().enumerate().find(|(_, item)| **item == 0).unwrap();

    for ind in &indexes {
        total += t[(zero_ind + ind) % l]
    }

    total
}

pub fn solve_part1(input: &str) -> String {
    let (_, list) = parse(input).unwrap();

    let len = list.len() as i64;

    let mut mixed = (0..len).collect::<Vec<_>>(); // mixed[nex_index] = original_index ;

    mix(&list, &mut mixed);

    let total = total(list, mixed);

    total.to_string()
}

fn parse2(s: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, complete::i64.map(|v| 811589153 * v))(s)
}

pub fn solve_part2(input: &str) -> String {
    let (_, list) = parse2(input).unwrap();

    let len = list.len() as i64;

    let mut mixed = (0..len).collect::<Vec<_>>(); // mixed[nex_index] = original_index ;

    for _ in 0..10 {
        mix(&list, &mut mixed);
    }

    let total = total(list, mixed);

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "3");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "1623178306");
    }
}
