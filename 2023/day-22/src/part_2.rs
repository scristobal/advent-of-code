/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use glam::UVec3;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Block {
    start: UVec3,
    end: UVec3,
}

impl FromStr for Block {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();

        let start: Vec<_> = start
            .split(',')
            .take(3)
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let start = UVec3::new(start[0], start[1], start[2]);

        let end: Vec<_> = end
            .split(',')
            .take(3)
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let end = UVec3::new(end[0], end[1], end[2]);

        Ok(Block { start, end })
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.z.cmp(&other.start.z)
    }
}

impl Block {
    fn points_inside(&self) -> Vec<UVec3> {
        let mut interior = vec![];

        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    interior.push(UVec3::new(x, y, z));
                }
            }
        }

        interior
    }

    fn points_below(&self) -> Vec<UVec3> {
        let mut floor = vec![];

        if self.start.z == 0 {
            return floor;
        }

        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                floor.push(UVec3::new(x, y, self.start.z - 1));
            }
        }

        floor
    }

    fn is_on_top(&self, other: &Block) -> bool {
        let points_below = self.points_below();

        let points_inside_other = other.points_inside();

        points_below.iter().any(|p| points_inside_other.contains(p))
    }

    fn move_down(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
    }
}

#[derive(Debug)]
struct World {
    blocks: Vec<Block>,
    z_floor: u32,
}

impl World {
    pub fn new(mut blocks: Vec<Block>) -> Self {
        let z_floor = 1;

        blocks.sort();

        Self { blocks, z_floor }
    }

    // returns true iff world has changed
    fn apply_z_force(&mut self) -> bool {
        let mut changed = false;

        // remember blocks are ordered in ascending z-axis
        for i in 0..self.blocks.len() {
            let blocks = &mut self.blocks;

            let current_block = &blocks[i];

            let is_over_ground = current_block.start.z == self.z_floor;

            let is_over_other_block = blocks[0..i].iter().any(|b| current_block.is_on_top(b));

            if !is_over_ground && !is_over_other_block {
                let current_block = &mut blocks[i];
                current_block.move_down();

                changed = true;
            };
        }

        changed
    }

    fn simulate(&mut self) {
        while self.apply_z_force() {}
    }
}

pub fn solve(input: &'static str) -> String {
    let blocks: Vec<_> = input.lines().map(|l| l.parse::<Block>().unwrap()).collect();

    let mut world = World::new(blocks);

    let block_names: Vec<char> = (0..world.blocks.len())
        .scan('A', |state, _| {
            let name = *state;
            *state = (*state as u8 + 1) as char;
            Some(name)
        })
        .collect();

    world.simulate();

    let mut edges = vec![];

    for i in 0..world.blocks.len() {
        let blocks = &world.blocks;
        let current_block = &blocks[i];

        let current_block_holds: Vec<_> = blocks
            .iter()
            .enumerate()
            .filter_map(|(j, b)| (b != current_block && b.is_on_top(current_block)).then_some(j))
            .collect();

        // k supports i
        for k in current_block_holds.into_iter() {
            edges.push((i + 1, k + 1))
        }

        if current_block.start.z == 1 {
            edges.push((0, i + 1));
        }
    }

    let supports: DiGraphMap<usize, ()> = edges.iter().collect();

    let mut total_fall = 0;

    for n in 1..=world.blocks.len() {
        let mut removed = supports.clone();
        removed.remove_node(n);

        let num_reachable = dijkstra(&removed, 0, None, |_| 1);

        total_fall += world.blocks.len() - num_reachable.len()
    }

    total_fall.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "17");
    }
}
