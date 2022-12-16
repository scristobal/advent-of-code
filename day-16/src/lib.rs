use std::collections::{HashMap, HashSet};

use nom::character::complete::alpha1;
use nom::sequence::preceded;
use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};

use petgraph::algo::{dijkstra, min_spanning_tree};

use petgraph::dot::{Config, Dot};
use petgraph::Graph;

use petgraph::graph::NodeIndex;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    name: String,
    rate: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State<'a> {
    time_left: i32,
    flow: i32,
    path: Vec<&'a NodeIndex>,
    options: Vec<&'a NodeIndex>,
}

fn line(s: &str) -> IResult<&str, (Node, Vec<String>)> {
    let (s, name) = preceded(tag("Valve "), alpha1)(s)?;
    let (s, rate) = preceded(tag(" has flow rate="), complete::i32)(s)?;
    let (s, links) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list0(tag(", "), alpha1),
    )(s)?;

    let links = links.into_iter().map(|link| link.to_string()).collect();

    Ok((
        s,
        (
            Node {
                name: name.to_string(),
                rate,
            },
            links,
        ),
    ))
}

fn file(s: &str) -> IResult<&str, Vec<(Node, Vec<String>)>> {
    separated_list0(newline, line)(s)
}

pub fn solve_part1(input: &str) -> String {
    let (s, raw) = file(input).unwrap();

    let edges = raw
        .iter()
        .map(|(origin, destinations)| (origin.name.clone(), destinations.clone()))
        .collect::<HashMap<_, _>>();

    let valves = raw
        .iter()
        .filter_map(|(node, _)| {
            if node.rate > 0 {
                Some((node.name.clone(), node.rate))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut g = Graph::<Node, ()>::new();

    let node_index = raw
        .into_iter()
        .map(|(node, _)| {
            let name = node.name.clone();
            let node_index = g.add_node(node);
            (name, node_index)
        })
        .collect::<HashMap<_, _>>();

    for (name, index) in &node_index {
        let dests = edges.get(name).unwrap();

        for dest in dests {
            let index_2 = node_index.get(dest).unwrap();

            g.add_edge(*index, *index_2, ());
        }
    }

    let mut distances = HashMap::new();

    for a in node_index.values() {
        let dist_map = dijkstra(&g, *a, None, |_| 1);

        for (b, dist) in dist_map {
            distances.insert((*a, b), dist);
        }
    }

    let indexes = valves
        .iter()
        .map(|(name, _)| node_index.get(name).unwrap())
        .rev()
        .collect::<Vec<_>>();

    let rates = valves
        .iter()
        .map(|(name, rate)| (node_index.get(name).unwrap(), rate))
        .collect::<HashMap<_, _>>();

    let start = node_index.get("AA").unwrap();

    let initial = State {
        time_left: 30,
        flow: 0,
        path: vec![start],
        options: indexes,
    };

    let mut discovered: HashSet<State> = HashSet::new();

    let mut stack = Vec::new();
    stack.push(initial);

    let mut max_flow = 0;
    let mut count = 0;

    while let Some(state) = stack.pop() {
        if !discovered.contains(&state) {
            for next in &state.options {
                let node = *state.path.last().unwrap();

                let time_left = state.time_left - distances.get(&(*node, **next)).unwrap() - 1;

                let rate = *rates.get(next).unwrap();
                let flow = state.flow + rate * time_left;

                let mut path = state.path.clone();

                path.push(next);

                let mut options = state.options.clone();

                options.retain(|option| option != next);

                if options.is_empty() {
                    count += 1;
                    if max_flow < flow {
                        max_flow = flow;
                        println!("new max {} after {}", &max_flow, &count);
                    }
                } else {
                    stack.push(State {
                        time_left,
                        flow,
                        path,
                        options,
                    });
                }
            }
        };
        discovered.insert(state);
    }

    max_flow.to_string()
}

pub fn solve_part2(input: &str) -> String {
    dbg!(input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "1651");
    }

    #[ignore = "not implemented"]
    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "");
    }
}
