use core::num;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::Add,
    time::Instant,
};

#[derive(Clone, Copy)]
enum Push {
    Left,
    Right,
}

#[derive(Clone)]
enum Shape {
    A,
    B,
    C,
    D,
    E,
}

use Shape::*;

struct Factory {
    shapes: [Shape; 5],
    shape_index: i32,
    pushes: Vec<Push>,
    push_index: i32,
}

impl Factory {
    fn new(pushes: Vec<Push>) -> Self {
        Self {
            shapes: [Shape::A, Shape::B, Shape::C, Shape::D, Shape::E],
            shape_index: -1,
            pushes,
            push_index: -1,
        }
    }

    fn next_shape(&mut self) -> Shape {
        self.shape_index = (self.shape_index + 1) % self.shapes.len() as i32;
        self.shapes[self.shape_index as usize].clone()
    }

    fn next_push(&mut self) -> Push {
        self.push_index = (self.push_index + 1) % self.pushes.len() as i32;
        self.pushes[self.push_index as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i64,
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone)]
struct Piece {
    body: Vec<Coords>,
}

impl Piece {
    pub fn new(shape: Shape, position: Coords) -> Self {
        let body = match shape {
            A => vec![
                Coords { x: 0, y: 0 },
                Coords { x: 1, y: 0 },
                Coords { x: 2, y: 0 },
                Coords { x: 3, y: 0 },
            ],
            B => vec![
                Coords { x: 1, y: 0 },
                Coords { x: 0, y: 1 },
                Coords { x: 1, y: 1 },
                Coords { x: 2, y: 1 },
                Coords { x: 1, y: 2 },
            ],
            C => vec![
                Coords { x: 0, y: 0 },
                Coords { x: 1, y: 0 },
                Coords { x: 2, y: 0 },
                Coords { x: 2, y: 1 },
                Coords { x: 2, y: 2 },
            ],
            D => vec![
                Coords { x: 0, y: 0 },
                Coords { x: 0, y: 1 },
                Coords { x: 0, y: 2 },
                Coords { x: 0, y: 3 },
            ],
            E => vec![
                Coords { x: 0, y: 0 },
                Coords { x: 1, y: 0 },
                Coords { x: 0, y: 1 },
                Coords { x: 1, y: 1 },
            ],
        };

        let body = body
            .iter()
            .map(|p| {
                *p + Coords {
                    x: position.x,
                    y: position.y,
                }
            })
            .collect();

        Self { body }
    }

    fn translate(&mut self, direction: &Coords) {
        self.body = self.body.iter().map(|p| *p + *direction).collect();
    }

    fn collision(&self, direction: &Coords, grid: &Grid) -> bool {
        self.body
            .iter()
            .map(|p| *p + *direction)
            .any(|p| grid.solid.contains(&p) || p.y <= 0 || (grid.width as i32) < p.x || p.x <= 0)
    }

    pub fn apply(&mut self, movement: &Push, grid: &Grid) -> bool {
        let mut moved = false;

        let v = &match movement {
            Push::Left => Coords { x: -1, y: 0 },
            Push::Right => Coords { x: 1, y: 0 },
        };

        if !self.collision(v, grid) {
            self.translate(v);
        }

        let v = &Coords { x: 0, y: -1 };

        if !self.collision(v, grid) {
            self.translate(v);
            moved = true;
        }

        moved
    }
}

struct Grid {
    solid: HashSet<Coords>,
    width: u32,
    floor: u64,
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "\n".to_string();
        for y in (0..=(self.grid.height() + 6)).rev() {
            for x in 0..=(self.grid.width + 1) {
                match self.grid.solid.contains(&Coords {
                    x: x as i32,
                    y: y as i64,
                }) {
                    true => output += "#",
                    false => {
                        if self.piece.body.contains(&Coords {
                            x: x as i32,
                            y: y as i64,
                        }) {
                            output += "@"
                        } else {
                            output += "."
                        }
                    }
                };
            }
            output += "\n";
        }

        f.write_str(&output)
    }
}

impl Grid {
    pub fn new(width: u32) -> Self {
        Grid {
            solid: HashSet::new(),
            width,
            floor: 0,
        }
    }

    pub fn consolidate(&mut self, pieze: &Piece) {
        for p in &pieze.body {
            self.solid.insert(*p);
        }

        let bounds = self.heights();

        let bound = bounds.iter().min().unwrap_or(&0);

        self.solid.retain(|p| p.y >= *bound);
        self.floor = *bound as u64;
    }

    pub fn height(&self) -> i64 {
        self.solid.iter().map(|p| p.y).max().unwrap_or(0)
    }

    pub fn heights(&self) -> Vec<i64> {
        let mut bounds = vec![];

        for x in 1..=self.width {
            let max_y = self
                .solid
                .iter()
                .filter_map(|p| if p.x == x as i32 { Some(p.y) } else { None })
                .max()
                .unwrap_or(0);

            bounds.push(max_y);
        }

        bounds
    }
}

struct Board {
    grid: Grid,
    piece: Piece,
    factory: Factory,
}

impl Board {
    pub fn new(grid: Grid, factory: Factory) -> Self {
        let piece = Piece { body: vec![] };

        Board {
            piece,
            grid,
            factory,
        }
    }

    fn pop(position: Coords, shape: Shape) -> Piece {
        Piece::new(shape, position)
    }

    pub fn pop_and_drop(&mut self) {
        let shape = self.factory.next_shape();
        //dbg!(self.grid.height());
        self.piece = Self::pop(
            Coords {
                x: 3,
                y: self.grid.height() + 4,
            },
            shape,
        );

        //dbg!("POP", &self);

        let mut pieze_moved = true;

        while pieze_moved {
            let push = self.factory.next_push();
            pieze_moved = self.piece.apply(&push, &self.grid);
            //dbg!("MOVE", &self);
        }

        self.grid.consolidate(&self.piece);
        //dbg!("CONS", &self);
    }

    pub fn state(&self) -> State {
        State {
            shape_index: self.factory.shape_index,
            push_index: self.factory.push_index,
            relative_heights: self
                .grid
                .heights()
                .iter()
                .map(|h| h - self.grid.height())
                .collect(),
        }
    }
}

fn parser(s: &str) -> Vec<Push> {
    s.chars()
        .filter_map(|c| match c {
            '>' => Some(Push::Right),
            '<' => Some(Push::Left),
            _ => None,
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    shape_index: i32,
    push_index: i32,
    relative_heights: Vec<i64>,
}

pub fn solve_part1(input: &str, num_pieces: i32) -> String {
    let pushes = parser(input);

    let grid = Grid::new(7);
    let factory = Factory::new(pushes);

    let mut board = Board::new(grid, factory);

    let mut count = 0;

    while count < num_pieces {
        count += 1;
        board.pop_and_drop();
    }

    board.grid.height().to_string()
}

pub fn solve_part2(input: &str, num_pieces: i64) -> String {
    let pushes = parser(input);

    let grid = Grid::new(7);
    let factory = Factory::new(pushes);

    let mut board = Board::new(grid, factory);

    let mut cache = HashMap::<State, (u64, u64)>::new();

    let mut count = 0;

    let (pieces, height) = loop {
        count += 1;

        board.pop_and_drop();

        let state = board.state();

        if count % 216 == 0 && cache.contains_key(&state) {
            let prev = cache.get(&state).unwrap();

            println!(
                "Jack pot!! {:?} seen at {} and then at {}, with heghts {} and then {}",
                state,
                prev.0,
                count,
                prev.1,
                board.grid.height()
            );

            println!("Pieces difference is {}", count - prev.0 as i64);
            println!(
                "Height difference is {}",
                board.grid.height() - prev.1 as i64
            );

            break (count - prev.0 as i64, board.grid.height() - prev.1 as i64);
        }

        cache.insert(state, (count as u64, board.grid.height() as u64));
    };

    let mut total_height: u128 = (num_pieces as u128 / pieces as u128) * height as u128;

    let left_pieces = num_pieces % pieces;

    total_height += solve_part1(input, left_pieces as i32)
        .parse::<u128>()
        .unwrap();

    total_height.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT, 2022);
        assert_eq!(result, "3068");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT, 1_000_000_000_000);
        assert_eq!(result, "1514285714288");
    }
}
