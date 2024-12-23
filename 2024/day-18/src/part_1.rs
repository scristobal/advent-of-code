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
    // path: Vec<(i32, i32)>,
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

pub fn solve_params(input: &'static str, side: i32, steps: usize) -> Result<String> {
    let mut blocks = HashSet::new();

    for z in -1..=(side + 1) {
        blocks.insert((z, -1));
        blocks.insert((z, side + 1));
        blocks.insert((-1, z));
        blocks.insert((side + 1, z));
    }

    let mut count = 0;

    for line in input.lines() {
        if count >= steps {
            break;
        }

        count += 1;

        let (x, y) = line.split_once(",").unwrap();

        blocks.insert((x.parse().unwrap(), y.parse().unwrap()));
    }

    let start = (0, 0);
    let end = (side, side);

    let mut queue = BinaryHeap::<State>::new();

    queue.push(State {
        // path: vec![start],
        coords: start,
        steps: 0,
    });

    let mut visited = HashMap::<(i32, i32), usize>::new();

    let mut min_steps = State {
        steps: usize::MAX,
        coords: (0, 0),
        // path: vec![],
    };

    while let Some(state) = queue.pop() {
        if let Some(steps) = visited.get(&state.coords) {
            if *steps <= state.steps {
                continue;
            }
        }

        visited.insert(state.coords, state.steps);

        if state.coords == end && state.steps < min_steps.steps {
            min_steps = State {
                coords: state.coords,
                steps: state.steps,
                // path: state.path.clone(),
            };

            continue;
        }

        if state.coords == end {
            continue;
        }

        let step_coords = (state.coords.0 - 1, state.coords.1);
        if !blocks.contains(&step_coords) {
            // let mut new_path = state.path.clone();
            // new_path.push(step_coords);
            queue.push(State {
                // path: new_path,
                coords: step_coords,
                steps: state.steps + 1,
            });
        }

        let step_coords = (state.coords.0 + 1, state.coords.1);
        if !blocks.contains(&step_coords) {
            // let mut new_path = state.path.clone();
            // new_path.push(step_coords);
            queue.push(State {
                // path: new_path,
                coords: step_coords,
                steps: state.steps + 1,
            });
        }

        let step_coords = (state.coords.0, state.coords.1 - 1);
        if !blocks.contains(&step_coords) {
            // let mut new_path = state.path.clone();
            // new_path.push(step_coords);
            queue.push(State {
                // path: new_path,
                coords: step_coords,
                steps: state.steps + 1,
            });
        }

        let step_coords = (state.coords.0, state.coords.1 + 1);
        if !blocks.contains(&step_coords) {
            // let mut new_path = state.path.clone();
            // new_path.push(step_coords);
            queue.push(State {
                // path: new_path,
                coords: step_coords,
                steps: state.steps + 1,
            });
        }
    }

    // for y in 0..=(side) {
    //     for x in 0..=(side) {
    //         if blocks.contains(&(x, y)) {
    //             print!("#")
    //         } else if min_steps.path.contains(&(x, y)) {
    //             print!("O")
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!()
    // }

    Ok(min_steps.steps.to_string())
}
pub fn solve(input: &'static str) -> Result<String> {
    solve_params(input, 70, 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        #[rustfmt::skip]
        let result = solve_params(
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
",6, 12).unwrap();

        assert_eq!(result, "22");
    }
}
