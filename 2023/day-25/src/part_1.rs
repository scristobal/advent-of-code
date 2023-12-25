/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use rustworkx_core::connectivity::stoer_wagner_min_cut;

use std::collections::HashSet;
use std::collections::VecDeque;

use petgraph::prelude::*;

fn parse(input: &str) -> Vec<(u32, u32)> {
    let mut lexicon = Vec::new();

    let mut set = HashSet::new();
    input.lines().for_each(|l| {
        let (n, d) = l.split_once(':').unwrap();

        let n = match lexicon.iter().position(|&e| e == n) {
            Some(l) => l,
            None => {
                lexicon.push(n);
                lexicon.len() - 1
            }
        };

        let d = d
            .trim()
            .split(' ')
            .map(|n| match lexicon.iter().position(|&e| e == n) {
                Some(l) => l,
                None => {
                    lexicon.push(n);
                    lexicon.len() - 1
                }
            })
            .collect::<VecDeque<_>>();

        for m in d {
            set.insert((n as u32, m as u32));
        }
    });

    set.into_iter().collect()
}

pub fn solve(input: &'static str) -> String {
    let edges = parse(input);

    let graph: UnGraph<u32, ()> = UnGraph::from_edges(edges);

    let (min_cut, partition) = stoer_wagner_min_cut(&graph, |_| Ok::<u32, ()>(1_u32))
        .unwrap()
        .unwrap();

    assert_eq!(min_cut, 3);

    ((graph.node_count() - partition.len()) * partition.len()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "54");
    }
}
