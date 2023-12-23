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
                    .filter_map(|(x, c)| {
                        let tile = match c {
                            '.' => Tile::Path,
                            '#' => Tile::Forest,
                            '>' => Tile::Path,
                            '<' => Tile::Path,
                            '^' => Tile::Path,
                            'v' => Tile::Path,
                            _ => return None,
                        };

                        Some((IVec2::new(x as i32, y as i32), tile))
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
                };

                write!(f, "{}", c)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Env {
    fn adjacents(&self, p: &IVec2) -> Vec<(IVec2, usize)> {
        [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
            .iter()
            .filter_map(|&d| {
                let q = *p + d;

                match self.tiles.get(&q) {
                    Some(Path) => Some((q, 1)),
                    _ => None,
                }
            })
            .collect()
    }

    fn fast_forward(&self, p: &IVec2, d: &IVec2) -> (IVec2, usize) {
        let mut p = *p;
        let mut d = *d;
        let mut dist = 0;

        loop {
            let adjacents: Vec<_> = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                .iter()
                .filter_map(|&q_d| {
                    if (q_d + d) == IVec2::ZERO {
                        return None;
                    }

                    let q = p + q_d;

                    match self.tiles.get(&q) {
                        Some(Path) => Some((q, q_d, dist + 1)),
                        _ => None,
                    }
                })
                .collect();

            if adjacents.len() == 1 {
                (p, d, dist) = *adjacents.first().unwrap();
            } else {
                break (p, dist);
            }
        }
    }

    fn adjacents_fast_forwarded(&self, p: &IVec2) -> Vec<(IVec2, usize)> {
        [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
            .iter()
            .filter_map(|&d| {
                let q = *p + d;

                match self.tiles.get(&q) {
                    Some(Path) => {
                        Some((self.fast_forward(&q, &d).0, self.fast_forward(&q, &d).1 + 1))
                    }
                    _ => None,
                }
            })
            .collect()
    }
}

pub fn solve(input: &'static str) -> String {
    let env: Env = input.parse().unwrap();

    let s = IVec2::new(1, 0);
    let g = IVec2::new(env.size.x - 2, env.size.y - 1);

    let mut queue = VecDeque::new();
    queue.push_front(s);

    let mut visited = HashSet::new();

    let mut graph = DiGraph::<IVec2, usize>::new();

    while let Some(p) = queue.pop_back() {
        if !visited.contains(&p) {
            for (q, d) in env.adjacents_fast_forwarded(&p) {
                let n = graph
                    .node_indices()
                    .find(|i| graph[*i] == p)
                    .unwrap_or_else(|| graph.add_node(p));

                let m = graph
                    .node_indices()
                    .find(|i| graph[*i] == q)
                    .unwrap_or_else(|| graph.add_node(q));

                graph.add_edge(n, m, d);

                queue.push_back(q);
            }

            visited.insert(p);
        }
    }

    let s = graph.node_indices().find(|i| graph[*i] == s).unwrap();
    let g = graph.node_indices().find(|i| graph[*i] == g).unwrap();

    let ways = algo::all_simple_paths::<Vec<_>, _>(&graph, s, g, 0, None).collect::<Vec<_>>();

    (ways
        .iter()
        .map(|w| {
            let mut dist = 0;

            for i in 0..w.len() - 1 {
                let n = w[i];
                let m = w[i + 1];

                dist += graph.edge_weight(graph.find_edge(n, m).unwrap()).unwrap();
            }

            dist
        })
        .max()
        .unwrap())
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "154");
    }

    #[test]
    fn fast_forward_test() {
        let input = r"#########
#.......#
#########";

        let env = input.parse::<Env>().unwrap();

        let p = IVec2::new(1, 1);

        let adjacents = env.adjacents(&p);

        assert_eq!(adjacents.len(), 1);
    }

    #[test]
    fn fast_forward_test_2() {
        let input = r"#.#######
#....####
####....#
####.##.#
#######.#";

        let env = input.parse::<Env>().unwrap();

        let p = IVec2::new(1, 0);

        let adjacents = env.adjacents_fast_forwarded(&p);

        assert_eq!(adjacents, vec![(IVec2::new(4, 2), 5)]);
    }

    #[test]
    fn fast_forward_test_3() {
        let input = r"#.#######
#....####
####....#
#######.#";

        let env = input.parse::<Env>().unwrap();

        let p = IVec2::new(1, 0);

        let adjacents = env.adjacents_fast_forwarded(&p);

        assert_eq!(adjacents, vec![(IVec2::new(7, 3), 9)]);
    }
}
