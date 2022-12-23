use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::From;
use std::fmt::Debug;

use itertools::Itertools;

const NORTH: [(i32, i32); 3] = [(1, -1), (0, -1), (-1, -1)];
const SOUTH: [(i32, i32); 3] = [(1, 1), (0, 1), (-1, 1)];
const WEST: [(i32, i32); 3] = [(-1, 1), (-1, 0), (-1, -1)];
const EAST: [(i32, i32); 3] = [(1, 1), (1, 0), (1, -1)];

#[derive(Debug)]
struct Rules(VecDeque<[(i32, i32); 3]>);

impl Rules {
    fn new() -> Self {
        Self(VecDeque::from(vec![NORTH, SOUTH, WEST, EAST]))
    }

    fn shift(&mut self) {
        self.0.rotate_left(1);
    }
}

struct Map {
    elfs: HashMap<(i32, i32), Option<(i32, i32)>>, // position -> proposal, None means not yet stated a proposal
    rules: Rules,
    ticks: i32,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let rules = Rules::new();

        let mut x = 1;
        let mut y = 1;

        let mut elfs = HashMap::new();

        for c in s.chars() {
            match c {
                '#' => {
                    elfs.insert((x, y), None);
                    x += 1;
                }
                '\n' => {
                    x = 1;
                    y += 1;
                }
                '.' => {
                    x += 1;
                }
                _ => unreachable!(),
            }
        }

        Self {
            elfs,
            rules,
            ticks: 0,
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "\n".to_string();

        let pad = 2;

        let max_width = *self.elfs.keys().map(|(x, _)| x).max().unwrap();
        let min_width = *self.elfs.keys().map(|(x, _)| x).min().unwrap();

        let max_height = *self.elfs.keys().map(|(_, y)| y).max().unwrap();
        let min_height = *self.elfs.keys().map(|(_, y)| y).min().unwrap();

        for y in (min_height - pad)..=(max_height + pad) {
            for x in (min_width - pad)..=(max_width + pad) {
                match self.elfs.contains_key(&(x, y)) {
                    true => output += "#",
                    false => output += ".",
                };
            }
            output += "\n";
        }

        f.write_str(&output)
    }
}

impl Map {
    fn freeze_isolated(&mut self) {
        let positions = self.elfs.keys().copied().collect::<Vec<_>>();

        let around = NORTH
            .iter()
            .chain(EAST.iter())
            .chain(SOUTH.iter())
            .chain(WEST.iter())
            .unique()
            .collect::<Vec<_>>();

        self.elfs.iter_mut().for_each(|(position, intention)| {
            if around
                .iter()
                .filter_map(|(x, y)| {
                    if positions.contains(&(position.0 + x, position.1 + y)) {
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
                == 0
            {
                *intention = Some(*position)
            }
        });
    }
    fn apply_rules(&mut self) {
        let positions = self.elfs.keys().copied().collect::<Vec<_>>();

        for (position, proposal) in self.elfs.iter_mut() {
            if proposal.is_some() {
                continue;
            }

            for rule in &self.rules.0 {
                if rule
                    .iter()
                    .filter_map(|(x, y)| {
                        if positions.contains(&(position.0 + x, position.1 + y)) {
                            Some(())
                        } else {
                            None
                        }
                    })
                    .count()
                    == 0
                {
                    *proposal = Some((position.0 + rule[1].0, position.1 + rule[1].1));
                    break;
                }
            }

            if proposal.is_none() {
                *proposal = Some(*position);
            }
        }

        self.rules.shift();
    }

    fn resolve_collisions(&mut self) {
        let proposals = self.elfs.values().copied().collect::<Vec<_>>();

        for (position, proposal) in self.elfs.iter_mut() {
            if proposals
                .iter()
                .filter_map(|other_proposal| {
                    if *other_proposal == *proposal {
                        return Some(());
                    }
                    None
                })
                .count()
                > 1
            {
                *proposal = Some(*position);
            }
        }
    }

    fn update(&mut self) -> bool {
        let locations = self.elfs.keys().copied().collect::<HashSet<_>>();

        let proposals = self.elfs.values().copied().collect::<Vec<_>>();

        let mut new_elfs = HashMap::new();

        for proposal in proposals {
            if let Some(proposal) = proposal {
                new_elfs.insert(proposal, None);
            } else {
                unreachable!()
            }
        }

        let new_locations = new_elfs.keys().copied().collect::<HashSet<_>>();

        self.elfs = new_elfs;

        new_locations == locations
    }

    pub fn tick(&mut self) -> bool {
        self.ticks += 1;

        self.freeze_isolated();
        self.apply_rules();
        self.resolve_collisions();

        self.update()
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut map = Map::from(input);

    for _ in 1..=10 {
        map.tick();
    }

    let width = map.elfs.keys().map(|(x, _)| x).max().unwrap()
        - map.elfs.keys().map(|(x, _)| x).min().unwrap()
        + 1;

    let height = map.elfs.keys().map(|(_, y)| y).max().unwrap()
        - map.elfs.keys().map(|(_, y)| y).min().unwrap()
        + 1;

    let spaces = (width * height) - map.elfs.len() as i32;

    spaces.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut map = Map::from(input);

    while !map.tick() {}

    map.ticks.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "110");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "20");
    }
}
