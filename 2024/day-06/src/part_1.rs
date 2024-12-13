use anyhow::Result;
use std::collections::{HashMap, HashSet};

enum Tile {
    Empty,
    Block,
}

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

pub fn solve(input: &'static str) -> Result<String> {
    let mut board = HashMap::<(i32, i32), Tile>::new();

    let mut player = Player {
        position: (0, 0),
        direction: Direction::Up,
    };

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
                    player.position = (x, y);
                }
                _ => unreachable!("bad input"),
            };
        }
    }

    let mut visited = HashSet::<(i32, i32)>::with_capacity(board.len());
    visited.insert(player.position);

    while let Some(tile) = board.get(&player.next_position()) {
        player.update(tile);
        visited.insert(player.position);
    }

    Ok(visited.len().to_string())
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
        assert_eq!(result, "41");
    }
}
