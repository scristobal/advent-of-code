use std::convert::TryFrom;

use anyhow::*;

#[derive(Debug)]
struct Monkey<'a> {
    items: Vec<u128>,
    num_inspected: u128,
    operation: (&'a str, &'a str),
    test: u128,
    next_if_true: usize,
    next_if_false: usize,
}

#[derive(Debug)]
struct Monkeys<'a>(Vec<Monkey<'a>>);

impl<'a> TryFrom<&'a str> for Monkey<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let (_, s) = s.split_once('\n').unwrap();

        let (items, s) = s.split_once('\n').unwrap();

        let items = items.strip_prefix("  Starting items: ").unwrap();

        let mut items = items
            .split(", ")
            .filter_map(|s| s.parse::<_>().ok())
            .collect::<Vec<_>>();

        items.reverse();

        let (operation, s) = s.split_once('\n').unwrap();

        let mut operation = operation
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>();

        let argument = operation.pop().unwrap();

        let symbol = operation.pop().unwrap();

        let operation = (argument, symbol);

        let (test, s) = s.split_once('\n').unwrap();

        let test = test
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<_>()?;

        let (next_if_true, s) = s.split_once('\n').unwrap();

        let next_if_true = next_if_true
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()?;

        let next_if_false = s
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()?;

        Ok(Monkey {
            items,
            num_inspected: 0,
            operation,
            test,
            next_if_true,
            next_if_false,
        })
    }
}

impl<'a> Monkeys<'a> {
    fn new_from_str(input: &str) -> Monkeys {
        Monkeys(
            input
                .split("\n\n")
                .map(|block| block.try_into().unwrap())
                .collect(),
        )
    }

    fn round(&mut self, worry_factor: u128) {
        for i in 0..self.0.len() {
            let monkey = &mut self.0[i];

            let distribution_items = monkey
                .items
                .iter()
                .map(|item| {
                    monkey.num_inspected += 1;

                    let argument = match monkey.operation.0 {
                        "old" => *item,
                        _ => monkey.operation.0.parse::<_>().unwrap(),
                    };

                    let item = item % (11 * 2 * 5 * 7 * 17 * 19 * 3 * 13 * 19 * 23);

                    let item = match monkey.operation.1 {
                        "*" => item * argument,
                        "+" => item + argument,
                        _ => unreachable!(),
                    } / worry_factor;

                    if item % monkey.test == 0 {
                        (monkey.next_if_true, item)
                    } else {
                        (monkey.next_if_false, item)
                    }
                })
                .collect::<Vec<_>>();

            monkey.items.clear();

            for distribution in distribution_items {
                self.0[distribution.0].items.push(distribution.1);
            }
        }
    }

    fn monkey_business(&self) -> u128 {
        let mut num_inspected = self
            .0
            .iter()
            .map(|monkey| monkey.num_inspected)
            .collect::<Vec<_>>();

        num_inspected.sort();

        num_inspected.iter().rev().take(2).product()
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut monkeys = Monkeys::new_from_str(input);

    for _ in 1..=20 {
        monkeys.round(3);
    }
    monkeys.monkey_business().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut monkeys = Monkeys::new_from_str(input);

    for _ in 1..=10_000 {
        monkeys.round(1);
    }

    monkeys.monkey_business().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
