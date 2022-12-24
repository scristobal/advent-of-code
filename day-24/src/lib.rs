use itertools::Itertools;
use num::{integer::lcm, Integer};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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

            let free_locs = self
                .all_locs
                .difference(&storm_locs)
                .copied()
                .collect::<HashSet<_>>();

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

fn print_grid(free_slots: HashSet<(i32, i32)>) {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    position: (i32, i32),
    time: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (15 * self.position.0 + 2 * self.position.1)
            .cmp(&(15 * other.position.0 + 2 * self.position.1))
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
            State {
                position: (self.position.0 + 1, self.position.1),
                time,
            },
            State {
                position: (self.position.0, self.position.1 + 1),
                time,
            },
            State {
                position: (self.position.0, self.position.1),
                time,
            },
            State {
                position: (self.position.0 - 1, self.position.1),
                time,
            },
            State {
                position: (self.position.0, self.position.1 - 1),
                time,
            },
        ]
    }
}

pub fn solve_part1(input: &str) -> String {
    let valley = Valley::from(input);

    let initial_state = State {
        position: (0, -1),
        time: 0,
    };

    let exit = (valley.width - 1, valley.height - 1);

    let mut min_time = i32::MAX;

    let mut stack = BinaryHeap::from(vec![initial_state]);
    let mut visited = HashSet::new();

    while let Some(state) = stack.pop() {
        if !visited.contains(&state) {
            if state.position == exit {
                min_time = state.time.min(min_time);
                println!("new min time {min_time}");
            } else {
                for state in state.next_states() {
                    if valley.free_spots(state.time).contains(&state.position)
                        && state.time + (exit.0 - state.position.0 + exit.1 - state.position.1)
                            < min_time
                    // TODO: && visited same spot on a multiple of time period
                    {
                        stack.push(state);
                    }
                }
            }
        }
        visited.insert(state);
    }

    (min_time + 1).to_string()
}

pub fn solve_part2(input: &str) -> String {
    dbg!(input);
    todo!()
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

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "");
    }
}
