use anyhow::Result;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    coords: (i32, i32),
    steps: usize,
}

fn dist(coords: &(i32, i32)) -> i32 {
    let max = coords.0.max(coords.1);
    (max - coords.0).abs() + (max - coords.1).abs()
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        dist(&other.coords).cmp(&dist(&self.coords))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &'static str, side: i32) -> HashMap<(i32, i32), usize> {
    let mut blocks = HashMap::new();

    for z in -1..=(side + 1) {
        blocks.insert((z, -1), 0);
        blocks.insert((z, side + 1), 0);
        blocks.insert((-1, z), 0);
        blocks.insert((side + 1, z), 0);
    }

    for (ind, line) in input.lines().enumerate() {
        let (x, y) = line.split_once(",").unwrap();
        blocks.insert((x.parse().unwrap(), y.parse().unwrap()), ind);
    }

    blocks
}

pub fn solve_params(all_blocks: &HashMap<(i32, i32), usize>, side: i32, time_out: usize) -> usize {
    let start = (0, 0);
    let end = (side, side);

    let blocks: HashSet<(i32, i32)> = all_blocks
        .iter()
        .filter_map(|(&c, &t)| (t <= time_out).then_some(c))
        .collect();

    let mut queue = BinaryHeap::<State>::new();

    queue.push(State {
        coords: start,
        steps: 0,
    });

    let mut visited = HashMap::<(i32, i32), usize>::new();

    let mut min_state = State {
        steps: usize::MAX,
        coords: (0, 0),
    };

    while let Some(state) = queue.pop() {
        if let Some(steps) = visited.get(&state.coords) {
            if *steps <= state.steps {
                continue;
            }
        }

        visited.insert(state.coords, state.steps);

        if state.coords == end && state.steps < min_state.steps {
            min_state = State {
                coords: state.coords,
                steps: state.steps,
            };

            continue;
        }

        if state.coords == end {
            continue;
        }

        let step_coords = (state.coords.0 - 1, state.coords.1);
        if !blocks.contains(&step_coords) {
            queue.push(State {
                coords: step_coords,
                steps: state.steps + 1,
            });
        }

        let step_coords = (state.coords.0 + 1, state.coords.1);
        if !blocks.contains(&step_coords) {
            queue.push(State {
                coords: step_coords,
                steps: state.steps + 1,
            });
        }

        let step_coords = (state.coords.0, state.coords.1 - 1);
        if !blocks.contains(&step_coords) {
            queue.push(State {
                coords: step_coords,
                steps: state.steps + 1,
            });
        }

        let step_coords = (state.coords.0, state.coords.1 + 1);
        if !blocks.contains(&step_coords) {
            queue.push(State {
                coords: step_coords,
                steps: state.steps + 1,
            });
        }
    }

    min_state.steps
}

pub fn find_blocker(blocks: &HashMap<(i32, i32), usize>, side: i32) -> Result<String> {
    for time_out in 0..=blocks.len() {
        if solve_params(blocks, side, time_out) == usize::MAX {
            dbg!(time_out);
            return blocks
                .iter()
                .find(|(_, &i)| i == time_out)
                .map(|(&c, _)| format!("{},{}", c.0.to_string(), c.1.to_string()))
                .ok_or_else(|| unreachable!());
        }
    }
    unreachable!()
}

pub fn solve(input: &'static str) -> Result<String> {
    let blocks = parse(input, 70);
    find_blocker(&blocks, 70)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        #[rustfmt::skip]
        let blocks = parse(
"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
",6);

        let result = find_blocker(&blocks, 6).unwrap();

        assert_eq!(result, "6,1");
    }
}
