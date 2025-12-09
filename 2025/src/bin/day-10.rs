use std::collections::{HashSet, VecDeque};
use z3::{
    Optimize,
    ast::{Ast, Int},
};

fn main() {
    let s = include_str!("../../input/2025/day10.txt");

    println!("part 1: {}", solve_p1(s));
    println!("part 2: {}", solve_p2(s));
}

fn solve_p1(s: &str) -> usize {
    let mut res = 0;

    for line in s.lines() {
        let mut sections = line.split_whitespace();

        let end_state = sections
            .next()
            .unwrap()
            .trim_matches(|c| c == '[' || c == ']')
            .chars()
            .map(|c| c == '#')
            .enumerate()
            .fold(0u64, |acc, (i, b)| acc | (b as u64) << i);

        let actions = sections
            .take_while(|&s| s.starts_with('('))
            .map(|section| {
                section
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .fold(0u64, |acc, b| acc | (1u64 << b))
            })
            .collect::<Vec<_>>();

        //                  (state, depth)
        let initial_state = ( 0, 0 );

        let mut visited: HashSet<u64> = HashSet::new();
        let mut queue: VecDeque<_> = vec![initial_state].into();

        while let Some(state) = queue.pop_back() {
            if visited.contains(&state.0) {
                continue;
            }

            if state.0 == end_state {
                res += state.1;
                break;
            }

            let steps = state.1 + 1;

            for action in actions.iter() {
                queue.push_front((state.0 ^ *action, steps));
            }

            visited.insert(state.0);
        }
    }

    res
}

fn solve_p2(s: &str) -> u64 {
    let mut res = 0;

    for line in s.lines() {
        let optimize = Optimize::new();

        let targets = line
            .split_whitespace()
            .last()
            .unwrap()
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .map(|n| Int::from_u64(n))
            .collect::<Box<[Int]>>();

        let coeficients = line
            .split_whitespace()
            .skip(1)
            .take_while(|&s| s.starts_with('('))
            .map(|section| {
                section
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Box<_>>()
            })
            .collect::<Vec<Box<[usize]>>>();

        let coeficients = coeficients
            .into_iter()
            .map(|indexes| {
                let mut expanded = vec![Int::from_u64(0u64); targets.len()].into_boxed_slice();
                for ind in indexes {
                    expanded[ind] = Int::from_u64(1);
                }
                expanded
            })
            .collect::<Vec<_>>();

        let variables = (0..coeficients.len())
            .map(|i| Int::new_const(format!("n_{i}")))
            .collect::<Vec<_>>();

        for var in &variables {
            optimize.assert(&var.ge(Int::from_u64(0)));
        }

        let equations = coeficients
            .into_iter()
            .zip(&variables)
            .map(|(coef, var)| coef.into_iter().map(|c| c * var).collect::<Vec<_>>())
            .fold(vec![Int::from_u64(0); targets.len()], |acc, term| {
                acc.into_iter()
                    .zip(term)
                    .map(|(acc, term)| acc + term)
                    .collect()
            });

        let equations = equations
            .into_iter()
            .zip(targets)
            .map(|(equations, target)| equations.eq(&target))
            .collect::<Vec<_>>();

        for equation in &equations {
            optimize.assert(equation);
        }

        let sum_vars = variables
            .into_iter()
            .fold(Int::from_u64(0), |acc, var| acc + var)
            .simplify();

        optimize.minimize(&sum_vars);

        assert!(matches!(optimize.check(&[]), z3::SatResult::Sat));

        let model = optimize.get_model().unwrap();

        let sum_vars = model.eval(&sum_vars, true).unwrap();

        res += sum_vars.as_u64().unwrap();
    }

    res
}

#[derive(Clone)]
struct StateBox {
    state: Box<[u64]>,
    steps: usize,
}

pub fn solve_p2b(s: &str) -> usize {
    let mut res = 0;

    for line in s.lines() {
        let mut actions = line
            .split_whitespace()
            .skip(1)
            .take_while(|&s| s.starts_with('('))
            .map(|section| {
                section
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect::<Vec<Box<[usize]>>>();

        actions.sort_by_key(|a| -(a.len() as i64));

        let target_state = line
            .split_whitespace()
            .last()
            .unwrap()
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Box<[u64]>>();

        let initial_state = StateBox {
            state: vec![0; target_state.len()].into_boxed_slice(),
            steps: 0,
        };

        let mut visited: HashSet<Box<[u64]>> = HashSet::new();
        let mut queue: VecDeque<_> = vec![initial_state].into();

        while let Some(state) = queue.pop_back() {
            if visited.contains(&state.state) {
                continue;
            }

            if state.state == target_state {
                res += state.steps;
                break;
            }

            let steps = state.steps + 1;

            'outer: for action in actions.iter() {
                let mut state = state.state.clone();

                for item in action {
                    if state[*item] >= target_state[*item] {
                        break 'outer;
                    }
                    state[*item] += 1;
                }

                queue.push_front(StateBox { state, steps });
            }

            visited.insert(state.state);
        }
    }

    res
}

#[cfg(test)]
mod test {
    use z3::Solver;

    use crate::*;

    const SAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn sample_p1() {
        assert_eq!(solve_p1(SAMPLE), 7)
    }

    #[test]
    fn sample_p2() {
        assert_eq!(solve_p2(SAMPLE), 33)
    }

    #[test]
    fn test_z3() {
        let solver = Solver::new();

        let x = Int::new_const("x");

        let one = Int::from_u64(1);
        let two = Int::from_u64(2);

        let eq = (&x * one).eq(two);

        solver.assert(&eq);

        assert!(matches!(solver.check(), z3::SatResult::Sat));

        let model = solver.get_model().unwrap();
        let s = model.eval(&x, true).unwrap();

        assert_eq!(s.as_u64().unwrap(), 2);
    }
}
