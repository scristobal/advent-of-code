/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use glam::UVec3;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::borrow::BorrowMut;
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
    fn is_on_top(&self, other: &Block) -> bool {
        let overlap_x = self.start.x <= other.end.x && other.start.x <= self.end.x;
        let overlap_y = self.start.y <= other.end.y && other.start.y <= self.end.y;
        let overlap_z = (self.start.z - 1) <= other.end.z && other.start.z <= (self.end.z - 1);

        overlap_x && overlap_y && overlap_z
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

        Self { blocks, z_floor }
    }

    fn apply_z_force(&mut self) {
        // remember blocks are ordered in ascending z-axis
        for i in 0..self.blocks.len() {
            let blocks = &mut self.blocks;

            let current_block = &blocks[i];

            let mut is_over_ground = current_block.start.z == self.z_floor;

            let mut is_over_other_block = blocks[0..i].iter().any(|b| current_block.is_on_top(b));

            while !is_over_ground && !is_over_other_block {
                let current_block = &mut blocks[i];

                current_block.move_down();

                let current_block = &blocks[i];

                is_over_other_block = blocks[0..i].iter().any(|b| current_block.is_on_top(b));
                is_over_ground = current_block.start.z == self.z_floor;
            }
        }
    }
}

pub fn solve(input: &'static str) -> String {
    let blocks: Vec<_> = input.lines().map(|l| l.parse::<Block>().unwrap()).collect();

    let mut world = World::new(blocks);

    world.apply_z_force();

    let mut edges = vec![];

    for i in 0..world.blocks.len() {
        let blocks = &world.blocks;
        let current_block = &blocks[i];

        let current_block_holds: Vec<_> = blocks
            .iter()
            .enumerate()
            .filter_map(|(j, b)| (b.is_on_top(current_block)).then_some(j))
            .collect();

        // k supports i
        for k in current_block_holds.into_iter() {
            edges.push((i + 1, k + 1))
        }

        // ground blocks are supported by the floor
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
