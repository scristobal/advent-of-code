use anyhow::Result;
use std::collections::{HashMap, HashSet};

struct State {
    coord: (i32, i32),
    head: (i32, i32),
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut map = HashMap::new();

    let mut heads = HashMap::<(i32, i32), HashSet<(i32, i32)>>::new();
    let mut queue = vec![];

    for (x, line) in input.lines().enumerate() {
        for (y, val) in line
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .enumerate()
        {
            let coord = (x as i32, y as i32);

            map.insert(coord, val);

            if val == 0 {
                queue.push(State { coord, head: coord });
            }
        }
    }

    while let Some(state) = queue.pop() {
        let Some(val) = map.get(&state.coord) else {
            continue;
        };

        if *val == 9 {
            heads
                .entry(state.head)
                .and_modify(|e| {
                    e.insert(state.coord);
                })
                .or_insert(HashSet::from([state.coord]));

            continue;
        }

        if let Some(next_val) = map.get(&(state.coord.0 + 1, state.coord.1)) {
            if *next_val == (val + 1) {
                queue.push(State {
                    coord: (state.coord.0 + 1, state.coord.1),
                    head: state.head,
                });
            }
        }
        if let Some(next_val) = map.get(&(state.coord.0 - 1, state.coord.1)) {
            if *next_val == (val + 1) {
                queue.push(State {
                    coord: (state.coord.0 - 1, state.coord.1),
                    head: state.head,
                });
            }
        }
        if let Some(next_val) = map.get(&(state.coord.0, state.coord.1 + 1)) {
            if *next_val == (val + 1) {
                queue.push(State {
                    coord: (state.coord.0, state.coord.1 + 1),
                    head: state.head,
                });
            }
        }
        if let Some(next_val) = map.get(&(state.coord.0, state.coord.1 - 1)) {
            if *next_val == (val + 1) {
                queue.push(State {
                    coord: (state.coord.0, state.coord.1 - 1),
                    head: state.head,
                });
            }
        }
    }
    Ok(heads.values().map(|d| d.len()).sum::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "36");
    }
}
