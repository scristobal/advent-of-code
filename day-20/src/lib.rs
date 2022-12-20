use core::num;
use std::fmt::{Debug, Display};

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Debug, Clone)]
struct MixedValue {
    value: i64,
    original_ind: i64,
    mixed_ind: i64,
}

struct Mixer(Vec<MixedValue>);

impl Debug for Mixer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mixer").field(&self.0).finish()
    }
    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     let mut s = "".to_string();

    //     for ind in 0..self.0.len() {
    //         let item = self
    //             .0
    //             .iter()
    //             .find(|item| item.mixed_ind == ind as i64)
    //             .unwrap();

    //         s += &item.value.to_string();
    //         s += ","
    //     }

    //     f.write_str(&s)
    // }
}

impl Display for Mixer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();

        for ind in 0..self.0.len() {
            let item = self
                .0
                .iter()
                .find(|item| item.mixed_ind == ind as i64)
                .unwrap();

            s += &item.value.to_string();
            s += ","
        }

        f.write_str(&s)
    }
}

impl Mixer {
    fn new(vec: &[i64]) -> Self {
        Self(
            vec.iter()
                .enumerate()
                .map(|(i, v)| MixedValue {
                    original_ind: i as i64,
                    mixed_ind: i as i64,
                    value: *v,
                })
                .collect(),
        )
    }
    fn mix(&mut self, original_ind: i64, value_step: i64) {
        //dbg!("---input---", &original_ind, &value_step, &self);
        //println!("before: {}", &self);

        let len = self.0.len() as i64;

        let item = self
            .0
            .iter()
            .find(|v| v.original_ind == original_ind)
            .unwrap();

        let old_ind = item.mixed_ind;

        let mut new_ind = (old_ind + value_step).rem_euclid(len);

        if value_step > 0 {
            let num_overflows = (old_ind + value_step) / len;
            // dbg!(&num_overflows);
            let num_overflows = num_overflows.rem_euclid(len);

            new_ind = (new_ind + num_overflows).rem_euclid(len);
        }

        if value_step < 0 {
            let mut num_underflows = value_step.abs() / len;

            if value_step.abs() >= old_ind {
                num_underflows += 1
            }
            //dbg!(&num_underflows);

            let num_underflows = num_underflows.rem_euclid(len);

            new_ind = (new_ind - num_underflows).rem_euclid(len);
        }

        //dbg!(&value_step, &old_ind, &new_ind);

        let items = self
            .0
            .iter_mut()
            .filter(|v| {
                (old_ind.min(new_ind) <= v.mixed_ind) && v.mixed_ind <= new_ind.max(old_ind)
            })
            .collect::<Vec<_>>();

        //dbg!(&items);

        for item in items {
            if new_ind < old_ind {
                item.mixed_ind += 1
            } else {
                item.mixed_ind -= 1
            }

            if item.original_ind == original_ind {
                item.mixed_ind = new_ind
            }
        }

        //dbg!("---output---", &self);
        //println!("after: {}", &self);
    }
}

fn parse(s: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, complete::i64)(s)
}

fn total(list: Vec<i64>, mixed: Vec<i64>) -> i64 {
    let l = list.len();

    let indexes = [1000, 2000, 3000];

    let mut total = 0;

    let t = mixed
        .iter()
        .map(|ind| list[*ind as usize])
        .collect::<Vec<_>>();

    let (zero_ind, _) = t.iter().enumerate().find(|(_, item)| **item == 0).unwrap();

    for ind in &indexes {
        total += t[(zero_ind + ind) % l]
    }

    total
}

fn mix(list: &Vec<i64>, mixed: &mut Vec<i64>) {
    let len = list.len() as i64;

    // let t = mixed
    //     .iter()
    //     .map(|ind| list[*ind as usize])
    //     .collect::<Vec<_>>();

    // println!("{:?}", t);

    for (ind, steps) in list.iter().enumerate() {
        let (ind, _) = mixed
            .iter()
            .enumerate()
            .find(|(_, item)| **item == ind as i64)
            .unwrap();

        let mut ind = ind as i64;

        let step = if *steps > 0 { 1 } else { -1 };

        //dbg!(&steps);

        let mut remaining = steps.abs().rem_euclid(len - 1);

        while remaining > 0 {
            let next = (ind + step).rem_euclid(len);
            if (ind == (len - 1) && next == 0) || ind == 0 && next == (len - 1) {
                remaining += 1;
            }
            let a = mixed.remove(ind as usize);

            //dbg!(&ind, &next, &a);

            mixed.insert(next as usize, a);

            ind = next;

            remaining -= 1;
        }

        // let t = mixed
        //     .iter()
        //     .map(|ind| list[*ind as usize])
        //     .collect::<Vec<_>>();

        // println!("{:?}", t);
    }
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

    let t = mixed
        .iter()
        .map(|ind| list[*ind as usize])
        .collect::<Vec<_>>();

    println!("{:?}", t);

    for round in 0..10 {
        println!("start round {} ", round);
        mix(&list, &mut mixed);
        let t = mixed
            .iter()
            .map(|ind| list[*ind as usize])
            .collect::<Vec<_>>();

        println!("{:?}", t);
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
