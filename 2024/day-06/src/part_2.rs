use anyhow::Result;
use std::collections::{HashMap, HashSet};

enum Tile {
    Empty,
    Block,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Player {
    position: (i32, i32),
    direction: Direction,
}

impl Player {
    fn next_position(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        }
    }
    fn update(&mut self, tile: &Tile) {
        match tile {
            Tile::Empty => self.position = self.next_position(),
            Tile::Block => self.direction = self.direction.rotate(),
        }
    }
}

fn visited(player: &mut Player, board: &HashMap<(i32, i32), Tile>) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::<(i32, i32)>::with_capacity(board.len());
    visited.insert(player.position);

    while let Some(tile) = board.get(&player.next_position()) {
        player.update(tile);
        visited.insert(player.position);
    }

    visited
}

fn is_loop(mut player: Player, board: &HashMap<(i32, i32), Tile>) -> bool {
    let mut visited = HashSet::<Player>::with_capacity(board.len());

    visited.insert(player);

    loop {
        let Some(tile) = board.get(&player.next_position()) else {
            return false;
        };

        player.update(tile);

        if visited.contains(&player) {
            return true;
        }

        visited.insert(player);
    }
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut board = HashMap::<(i32, i32), Tile>::new();

    let mut initial_position = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match char {
                '.' => {
                    board.insert((x, y), Tile::Empty);
                }
                '#' => {
                    board.insert((x, y), Tile::Block);
                }
                '^' => {
                    board.insert((x, y), Tile::Empty);
                    initial_position = (x, y);
                }
                _ => unreachable!("bad input"),
            };
        }
    }

    let mut player = Player {
        position: initial_position,
        direction: Direction::Up,
    };

    let mut visited = visited(&mut player, &board);

    visited.remove(&initial_position);

    let mut loop_options_count = 0;

    for location in visited {
        player = Player {
            position: initial_position,
            direction: Direction::Up,
        };

        board.entry(location).and_modify(|tile| *tile = Tile::Block);

        if is_loop(player, &board) {
            loop_options_count += 1;
        }

        board.entry(location).and_modify(|tile| *tile = Tile::Empty);
    }

    Ok(loop_options_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "6");
    }
}
