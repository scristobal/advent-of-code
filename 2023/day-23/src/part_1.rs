/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use glam::IVec2;
use petgraph::algo;
use petgraph::graph::DiGraph;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

#[derive(Debug)]
enum Tile {
    Path,
    Forest,
    Slope(IVec2),
}

use Tile::*;

#[derive(Debug)]
struct Env {
    tiles: HashMap<IVec2, Tile>,
    size: IVec2,
}

impl FromStr for Env {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let tile = match c {
                            '.' => Tile::Path,
                            '#' => Tile::Forest,
                            '>' => Tile::Slope(IVec2::X),
                            '<' => Tile::Slope(IVec2::NEG_X),
                            '^' => Tile::Slope(IVec2::NEG_Y),
                            'v' => Tile::Slope(IVec2::Y),
                            _ => panic!("invalid tile"),
                        };

                        (IVec2::new(x as i32, y as i32), tile)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let size = IVec2::new(
            s.lines().count() as i32,
            s.lines().next().unwrap().len() as i32,
        );

        Ok(Env { tiles, size })
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let tile = self.tiles.get(&IVec2::new(x, y)).unwrap();

                let c = match tile {
                    Path => '.',
                    Forest => '#',
                    Slope(IVec2::X) => '>',
                    Slope(IVec2::NEG_X) => '<',
                    Slope(IVec2::NEG_Y) => '^',
                    Slope(IVec2::Y) => 'v',
                    _ => panic!("invalid tile"),
                };

                write!(f, "{}", c)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

const DIRECTIONS: &[IVec2; 4] = &[IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

pub fn solve(input: &'static str) -> String {
    let env: Env = input.parse().unwrap();

    let s = IVec2::new(1, 0);
    let g = IVec2::new(env.size.x - 2, env.size.y - 1);

    let generate = move |p| {
        if let Some(Slope(s)) = env.tiles.get(&p) {
            return vec![(p + *s, 1)];
        };

        DIRECTIONS
            .iter()
            .filter_map(|&d| {
                let q = p + d;

                match env.tiles.get(&q) {
                    Some(Path) => Some((q, 1)),
                    Some(Slope(_)) => Some((q, 1)),
                    _ => None,
                }
            })
            .collect()
    };

    let mut queue = VecDeque::new();
    queue.push_front(s);

    let mut visited = HashSet::new();

    let mut graph = DiGraph::<IVec2, usize>::new();

    while let Some(p) = queue.pop_back() {
        if !visited.contains(&p) {
            let gs = generate(p);
            for (q, w) in gs {
                let n = match graph.node_indices().find(|i| graph[*i] == p) {
                    Some(n) => n,
                    None => graph.add_node(p),
                };

                let m = match graph.node_indices().find(|i| graph[*i] == q) {
                    Some(m) => m,
                    None => graph.add_node(q),
                };

                graph.add_edge(n, m, w);

                queue.push_back(q);
            }

            visited.insert(p);
        }
    }

    let s = graph.node_indices().find(|i| graph[*i] == s).unwrap();
    let g = graph.node_indices().find(|i| graph[*i] == g).unwrap();

    let ways = algo::all_simple_paths::<Vec<_>, _>(&graph, s, g, 0, None).collect::<Vec<_>>();

    (ways.iter().map(|w| w.len()).max().unwrap() - 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "94");
    }
}
