#![feature(array_zip)]

use rayon::prelude::*;
use std::collections::BinaryHeap;
use std::{cmp::Ordering, collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RobotFactory {
    id: i32,
    costs: [[i32; 4]; 4], // [Ore [Ore, Clay, Obsidian, Geode], Clay, Obsidian, Geode]
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    robots: [i32; 4], // [Ore, Clay, Obsidian, Geode]
    materials: [i32; 4],
    time: i32,
    // history: String,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("Ore", &self.materials[0])
            .field("Clay", &self.materials[1])
            .field("Obisdian", &self.materials[2])
            .field("Geode", &self.materials[3])
            .field("Ore Bot ", &self.robots[0])
            .field("Clay Bot", &self.robots[1])
            .field("Obisdian Bot", &self.robots[2])
            .field("Geode Bot", &self.robots[3])
            .field("time", &self.time)
            .finish()
    }
}

impl State {
    pub fn initial() -> Self {
        State {
            robots: [1, 0, 0, 0],
            materials: [0; 4],
            time: 1,
            //  history: "".to_string(),
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.materials[3] + self.robots[3] * self.time)
            .cmp(&(other.materials[3] + other.robots[3] * other.time))
            .then_with(|| (self.materials[2]).cmp(&other.materials[2]))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RobotFactory {
    pub fn new(costs: [[i32; 4]; 4], id: i32) -> Self {
        RobotFactory { costs, id }
    }

    pub fn next_states(&self, state: &State) -> Vec<State> {
        let mut states = vec![];

        states.push(Self::collect(state));

        let collected_state = Self::collect(state);

        'robots: for i in (0..=3).rev() {
            for j in 0..=3 {
                if state.materials[j] < self.costs[i][j] {
                    continue 'robots;
                }
            }

            let state = self.build(&collected_state, i);

            states.push(state)
        }

        states
    }

    fn build(&self, state: &State, j: usize) -> State {
        let mut robots = state.robots;
        robots[j] += 1;

        State {
            robots,
            time: state.time,
            materials: state.materials.zip(self.costs[j]).map(|(m, c)| m - c),
            /*  history: state.history.clone()
            + " +++ "
            + &state.time.to_string()
            + " build "
            + &j.to_string()
            + " +++ ",*/
        }
    }

    fn collect(state: &State) -> State {
        let mut new_state = State {
            time: state.time + 1,
            materials: state.materials.zip(state.robots).map(|(m, r)| m + r),
            robots: state.robots,
            // history: state.history.clone(),
        };

        /* new_state.history =
        new_state.history.clone() + " >>> " + &new_state.clone().to_string() + " "; */

        new_state
    }
}

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

// Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 3 ore and 7 obsidian.

fn geode(s: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        tag("Each geode robot costs "),
        separated_pair(complete::i32, tag(" ore and "), complete::i32),
        tag(" obsidian."),
    )(s)
}

fn obsidian(s: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        tag("Each obsidian robot costs "),
        separated_pair(complete::i32, tag(" ore and "), complete::i32),
        tag(" clay. "),
    )(s)
}

fn clay(s: &str) -> IResult<&str, i32> {
    delimited(tag("Each clay robot costs "), complete::i32, tag(" ore. "))(s)
}

fn ore(s: &str) -> IResult<&str, i32> {
    delimited(tag("Each ore robot costs "), complete::i32, tag(" ore. "))(s)
}

fn id(s: &str) -> IResult<&str, i32> {
    delimited(tag("Blueprint "), complete::i32, tag(": "))(s)
}

fn blueprint(s: &str) -> IResult<&str, RobotFactory> {
    let (s, id) = id(s)?;

    let (s, ore) = ore(s)?;
    let ore_costs = [ore, 0, 0, 0];

    let (s, ore) = clay(s)?;
    let clay_costs = [ore, 0, 0, 0];

    let (s, (ore, clay)) = obsidian(s)?;
    let obsidian_costs = [ore, clay, 0, 0];

    let (s, (ore, obsidian)) = geode(s)?;
    let geode_costs = [ore, 0, obsidian, 0];

    Ok((
        s,
        RobotFactory::new([ore_costs, clay_costs, obsidian_costs, geode_costs], id),
    ))
}

fn parse(s: &str) -> IResult<&str, Vec<RobotFactory>> {
    separated_list1(newline, blueprint)(s)
}

pub fn solve_part1(input: &str) -> String {
    let (_, factories) = parse(input).unwrap();

    let mut sum_quality_levels = 0;

    for factory in factories {
        let mut visited = HashSet::new();

        let mut stack = BinaryHeap::from(vec![State::initial()]);

        let mut max_geode = 0;

        while let Some(state) = stack.pop() {
            if !visited.contains(&state) {
                if state.time <= 24 {
                    for state in factory.next_states(&state) {
                        stack.push(state);
                    }
                } else {
                    max_geode = state.materials[3].max(max_geode);
                }
            }
            visited.insert(state);
        }

        dbg!(&max_geode);

        sum_quality_levels += factory.id * max_geode;
    }

    sum_quality_levels.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (_, factories) = parse(input).unwrap();

    let factories = factories.iter().take(3).collect::<Vec<_>>();

    factories
        .par_iter()
        .map(|factory| {
            let mut visited = HashSet::new();

            let mut stack = BinaryHeap::from(vec![State::initial()]);

            let mut max_geode = 0;

            while let Some(state) = stack.pop() {
                if !visited.contains(&state) {
                    if state.time <= 32 {
                        for state in factory.next_states(&state) {
                            let time_left = 32 - state.time + 1;
                            let geode_bound = state.materials[3]
                                + state.robots[3] * time_left
                                + time_left * (time_left) / 2;

                            if geode_bound > max_geode {
                                stack.push(state);
                            } else {
                                //   println!("removed a branch of time {}", &state.time)
                            }
                        }
                    } else {
                        max_geode = state.materials[3].max(max_geode);
                        if max_geode == state.materials[3] {
                            println!(
                                "new max for factory {} found!! --> {}.",
                                factory.id, max_geode
                            )
                        }
                    }
                }
                visited.insert(state);
            }

            dbg!(&max_geode);

            max_geode
        })
        .product::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "33");
    }

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "62");
    }
}
