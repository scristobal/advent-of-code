use std::{cmp, collections::HashMap, fmt::Debug};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    *,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Block {
    Air,
    Rock,
    Sand,
}

use Block::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coords {
    x: u32,
    y: u32,
}

struct Map {
    layout: HashMap<Coords, Block>,
    min_width: u32,
    max_width: u32,
    min_height: u32,
    max_height: u32,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "\n".to_string();
        for y in 0..=(self.max_height + 10) {
            for x in (self.min_width - 10)..=(self.max_width + 10) {
                match self.layout.get(&Coords { x, y }) {
                    Some(Sand) => output += "o",
                    Some(Rock) => output += "#",
                    _ => output += ".",
                };
            }
            output += "\n";
        }

        f.write_str(&output)
    }
}

impl Map {
    fn coords(s: &str) -> IResult<&str, Coords> {
        let (s, (x, y)) = separated_pair(complete::u32, tag(","), complete::u32)(s)?;
        Ok((s, Coords { x, y }))
    }

    fn parser_line(s: &str) -> IResult<&str, Vec<Coords>> {
        let (s, v) = separated_list1(tag(" -> "), Self::coords)(s)?;

        let r = v
            .windows(2)
            .into_iter()
            .flat_map(|w| {
                let h = cmp::min(w[0].x, w[1].x)..=cmp::max(w[0].x, w[1].x);
                let v = cmp::min(w[0].y, w[1].y)..=cmp::max(w[0].y, w[1].y);

                h.cartesian_product(v).map(|(x, y)| Coords { x, y })
            })
            .collect::<Vec<_>>();

        Ok((s, r))
    }

    fn parser_input(s: &str) -> IResult<&str, Vec<Coords>> {
        let (s, v) = separated_list0(newline, Self::parser_line)(s)?;

        Ok((s, v.iter().flatten().copied().collect()))
    }

    pub fn from_str(s: &str) -> Map {
        let layout = Self::parser_input(s)
            .unwrap()
            .1
            .into_iter()
            .map(|v| (v, Block::Rock))
            .collect::<HashMap<_, _>>();

        let keys = layout.keys().copied().collect::<Vec<_>>();

        let min_width = *keys.iter().map(|Coords { x, y: _ }| x).min().unwrap();
        let max_width = *keys.iter().map(|Coords { x, y: _ }| x).max().unwrap();
        let min_height = *keys.iter().map(|Coords { x: _, y }| y).min().unwrap();
        let max_height = *keys.iter().map(|Coords { x: _, y }| y).max().unwrap();

        Map {
            layout,

            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    fn tick(&mut self) -> bool {
        let mut is_stable = true;

        let keys = self.layout.keys().copied().collect::<Vec<_>>();

        for coords in keys {
            let block = self.layout.get(&coords);

            let Some(Sand) = block  else { continue };

            let down = Coords {
                x: coords.x,
                y: coords.y + 1,
            };

            if self.layout.get(&down).is_none() {
                self.layout.remove(&coords);
                self.layout.insert(down, Sand);
                is_stable = false;
                continue;
            }

            let left = Coords {
                x: coords.x - 1,
                y: coords.y + 1,
            };

            if self.layout.get(&left).is_none() {
                self.layout.remove(&coords);
                self.layout.insert(left, Sand);
                is_stable = false;
                continue;
            }

            let right = Coords {
                x: coords.x + 1,
                y: coords.y + 1,
            };
            if self.layout.get(&right).is_none() {
                self.layout.remove(&coords);
                self.layout.insert(right, Sand);
                is_stable = false;
                continue;
            }
        }

        is_stable
    }

    fn is_over(&self) -> bool {
        let max_width = *self
            .layout
            .keys()
            .map(|Coords { x: _, y }| y)
            .max()
            .unwrap();

        max_width > self.max_width
    }

    fn pop(&mut self) {
        self.layout.insert(Coords { x: 500, y: 0 }, Sand);
    }
}

pub fn solve_part1(input: &str) -> String {
    let mut m = Map::from_str(input);

    m.pop();

    // dbg!(&m);

    while !m.is_over() {
        let stable = m.tick();

        if stable {
            // dbg!(&m);
            m.pop();
        }
    }

    dbg!(&m);

    (m.layout
        .values()
        .filter(|&block| matches!(block, Sand))
        .count()
        - 1)
    .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut m = Map::from_str(input);

    m.pop();

    for h in 0..=(m.max_height) {
        //dbg!(&m);
        let reversed = m
            .layout
            .clone()
            .into_iter()
            .into_group_map_by(|(_, block)| *block);

        let sand_blocks = reversed
            .get(&Sand)
            .unwrap()
            .iter()
            .filter(|(coords, _)| coords.y == h)
            .collect::<Vec<_>>();

        for (coords, _) in sand_blocks {
            let left = Coords {
                x: coords.x - 1,
                y: coords.y + 1,
            };

            if m.layout.get(&left).is_none() {
                m.layout.insert(left, Sand);
            }

            let down = Coords {
                x: coords.x,
                y: coords.y + 1,
            };

            if m.layout.get(&down).is_none() {
                m.layout.insert(down, Sand);
            }

            let right = Coords {
                x: coords.x + 1,
                y: coords.y + 1,
            };

            if m.layout.get(&right).is_none() {
                m.layout.insert(right, Sand);
            }
        }
    }

    dbg!(&m);

    (m.layout
        .values()
        .filter(|&block| matches!(block, Sand))
        .count())
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "24");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "93");
    }
}
