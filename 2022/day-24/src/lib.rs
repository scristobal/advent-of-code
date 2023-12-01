use itertools::Itertools;
use num::Integer;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug)]
struct Strom {
    loc: (i32, i32),
    dir: (i32, i32),
}

#[derive(Debug)]
struct Valley {
    stroms: Vec<Strom>,
    width: i32,
    height: i32,
    free_spots: HashMap<i32, HashSet<(i32, i32)>>,
    all_locs: HashSet<(i32, i32)>,
}

impl From<&str> for Valley {
    fn from(s: &str) -> Self {
        let stroms = s
            .lines()
            .skip(1)
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().skip(1).enumerate().filter_map(move |(x, c)| {
                    let dir = match c {
                        '>' => (1, 0),
                        '<' => (-1, 0),
                        '^' => (0, -1),
                        'v' => (0, 1),
                        _ => return None,
                    };

                    Some(Strom {
                        loc: (x as i32, y as i32),
                        dir,
                    })
                })
            })
            .collect();

        let width = s.chars().take_while(|c| *c != '\n').count() as i32 - 2;
        let height = s.chars().filter(|c| *c == '\n').count() as i32 - 2;

        let all_locs = (0..width)
            .cartesian_product(0..height)
            .collect::<HashSet<_>>();

        let mut valley = Valley {
            stroms,
            width,
            height,
            all_locs,
            free_spots: HashMap::new(),
        };

        valley.simulate();

        valley
    }
}

impl Valley {
    fn simulate(&mut self) {
        let period = self.width.lcm(&self.height);

        for time in 1..=period {
            let mut storm_locs = HashSet::with_capacity(self.stroms.len());

            for i in 0..self.stroms.len() {
                let loc = (
                    (self.stroms[i].loc.0 + self.stroms[i].dir.0).rem_euclid(self.width),
                    (self.stroms[i].loc.1 + self.stroms[i].dir.1).rem_euclid(self.height),
                );

                self.stroms[i] = Strom {
                    loc,
                    dir: self.stroms[i].dir,
                };

                storm_locs.insert(loc);
            }

            let mut free_locs = self
                .all_locs
                .difference(&storm_locs)
                .copied()
                .collect::<HashSet<_>>();

            free_locs.insert((0, -1));
            free_locs.insert((self.width - 1, self.height));

            self.free_spots.insert(time, free_locs);
        }
    }

    fn free_spots(&self, time: i32) -> HashSet<(i32, i32)> {
        let mut time = time % self.width.lcm(&self.height);

        if time == 0 {
            time = self.width.lcm(&self.height)
        }

        self.free_spots.get(&time).unwrap().clone()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    position: (i32, i32),
    time: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (15 * self.position.0 + 2 * self.position.1)
            .cmp(&(15 * other.position.0 + 2 * other.position.1))
            .then_with(|| (self.time).cmp(&other.time))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    fn next_states(&self) -> Vec<Self> {
        let time = self.time + 1;

        vec![
            Self {
                position: (self.position.0 + 1, self.position.1),
                time,
            },
            Self {
                position: (self.position.0, self.position.1 + 1),
                time,
            },
            Self {
                position: (self.position.0, self.position.1),
                time,
            },
            Self {
                position: (self.position.0 - 1, self.position.1),
                time,
            },
            Self {
                position: (self.position.0, self.position.1 - 1),
                time,
            },
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct StateReversed {
    position: (i32, i32),
    time: i32,
}

impl Ord for StateReversed {
    fn cmp(&self, other: &Self) -> Ordering {
        (15 * other.position.0 + 2 * other.position.1)
            .cmp(&(15 * self.position.0 + 2 * self.position.1))
            .then_with(|| (self.time).cmp(&other.time))
    }
}

impl PartialOrd for StateReversed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl StateReversed {
    fn next_states(&self) -> Vec<Self> {
        let time = self.time + 1;

        vec![
            Self {
                position: (self.position.0 - 1, self.position.1),
                time,
            },
            Self {
                position: (self.position.0, self.position.1 - 1),
                time,
            },
            Self {
                position: (self.position.0, self.position.1),
                time,
            },
            Self {
                position: (self.position.0 + 1, self.position.1),
                time,
            },
            Self {
                position: (self.position.0, self.position.1 + 1),
                time,
            },
        ]
    }
}

fn find_min_time_forward(start: (i32, i32), exit: (i32, i32), time: i32, valley: &Valley) -> i32 {
    let mut min_time = i32::MAX;

    let initial_state = State {
        position: start,
        time,
    };

    let mut stack = BinaryHeap::from(vec![initial_state]);
    let mut visited = HashSet::new();

    while let Some(state) = stack.pop() {
        if !visited.contains(&state) {
            if state.position == exit {
                min_time = state.time.min(min_time);
                println!("> new min time {min_time}");
            } else {
                for state in state.next_states() {
                    if valley.free_spots(state.time).contains(&state.position)
                        && state.time
                            + ((exit.0 - state.position.0).abs()
                                + (exit.1 - state.position.1).abs())
                            < min_time
                        && !visited.contains(&State {
                            position: state.position,
                            time: state.time - valley.width.lcm(&valley.height),
                        })
                    {
                        stack.push(state);
                    }
                }
            }
        }
        visited.insert(state);
    }
    min_time
}

fn find_min_time_backawards(
    start: (i32, i32),
    exit: (i32, i32),
    time: i32,
    valley: &Valley,
) -> i32 {
    let mut min_time = i32::MAX;

    let initial_state = StateReversed {
        position: start,
        time,
    };

    let mut stack = BinaryHeap::from(vec![initial_state]);
    let mut visited = HashSet::new();

    while let Some(state) = stack.pop() {
        if !visited.contains(&state) {
            if state.position == exit {
                min_time = state.time.min(min_time);
                println!("> new min time {min_time}");
            } else {
                for state in state.next_states() {
                    if valley.free_spots(state.time).contains(&state.position)
                        && state.time
                            + ((exit.0 - state.position.0).abs()
                                + (exit.1 - state.position.1).abs())
                            < min_time
                        && !visited.contains(&StateReversed {
                            position: state.position,
                            time: state.time - valley.width.lcm(&valley.height),
                        })
                    {
                        stack.push(state);
                    }
                }
            }
        }
        visited.insert(state);
    }
    min_time
}

pub fn solve_part1(input: &str) -> String {
    let valley = Valley::from(input);

    let start = (0, -1);
    let exit = (valley.width - 1, valley.height);

    let min_time = find_min_time_forward(start, exit, 0, &valley);

    min_time.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let valley = Valley::from(input);

    let start = (0, -1);
    let exit = (valley.width - 1, valley.height);

    let time = find_min_time_forward(start, exit, 0, &valley);

    println!("one way took {time}");

    let start = (valley.width - 1, valley.height);
    let exit = (0, -1);

    let time = find_min_time_backawards(start, exit, time, &valley);

    println!("return took {time}");

    let start = (0, -1);
    let exit = (valley.width - 1, valley.height);

    let time = find_min_time_forward(start, exit, time, &valley);

    time.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "18");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "54");
    }
}
