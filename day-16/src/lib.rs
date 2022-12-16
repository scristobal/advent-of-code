use std::collections::{HashMap, HashSet};

use nom::character::complete::alpha1;
use nom::sequence::preceded;
use nom::IResult;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list0,
};

use petgraph::algo::dijkstra;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SingleState<'a> {
    time_left: i32,
    flow: i32,
    path: Vec<&'a NodeIndex>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct PairState<'a> {
    me: SingleState<'a>,
    elephant: SingleState<'a>,
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
    let (_, raw) = file(input).unwrap();

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

                let time_left =
                    (state.time_left - distances.get(&(*node, **next)).unwrap() - 1).max(0);

                let rate = *rates.get(next).unwrap();
                let flow = state.flow + rate * time_left;

                let mut path = state.path.clone();

                path.push(next);

                let mut options = state.options.clone();

                options.retain(|option| option != next);

                options.sort_unstable_by(|a, b| {
                    let a_rate = *rates.get(a).unwrap();
                    let b_rate = *rates.get(b).unwrap();

                    b_rate.cmp(a_rate)
                });

                if options.is_empty() || time_left == 0 {
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
    let (_, raw) = file(input).unwrap();

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

    let mut indexes = valves
        .iter()
        .map(|(name, _)| node_index.get(name).unwrap())
        .rev()
        .collect::<Vec<_>>();

    let rates = valves
        .iter()
        .map(|(name, rate)| (node_index.get(name).unwrap(), rate))
        .collect::<HashMap<_, _>>();

    indexes.sort_unstable_by(|a, b| {
        let a_rate = *rates.get(a).unwrap();
        let b_rate = *rates.get(b).unwrap();

        b_rate.cmp(a_rate)
    });

    let start = node_index.get("AA").unwrap();

    let initial = SingleState {
        time_left: 26,
        flow: 0,
        path: vec![start],
    };

    let initial = PairState {
        me: initial.clone(),
        elephant: initial.clone(),
        options: indexes,
    };

    let mut discovered: HashSet<PairState> = HashSet::new();

    let mut stack = Vec::new();
    stack.push(initial);

    let mut max_flow = 0;
    let mut count = 0;

    while let Some(state) = stack.pop() {
        if !discovered.contains(&state) {
            for next in &state.options {
                let rate = *rates.get(next).unwrap();

                let me_node = *state.me.path.last().unwrap();

                let me_time_left =
                    (state.me.time_left - distances.get(&(*me_node, **next)).unwrap() - 1).max(0);

                let me_flow = state.me.flow + rate * me_time_left;
                let mut me_path = state.me.path.clone();
                me_path.push(next);

                let elephant_node = *state.elephant.path.last().unwrap();

                let elephant_time_left = (state.elephant.time_left
                    - distances.get(&(*elephant_node, **next)).unwrap()
                    - 1)
                .max(0);

                let elephant_flow = state.elephant.flow + rate * elephant_time_left;
                let mut elephant_path = state.elephant.path.clone();
                elephant_path.push(next);

                let mut options = state.options.clone();

                options.retain(|option| {
                    if option != next {
                        return true;
                    }

                    let d = *distances.get(&(**next, **option)).unwrap();

                    d >= (me_time_left.max(elephant_time_left) + 1)
                });

                if options.is_empty() || (me_time_left == 0 && elephant_time_left == 0) {
                    count += 1;

                    let both_flow =
                        (me_flow + state.elephant.flow).max(elephant_flow + state.me.flow);

                    if max_flow < both_flow {
                        max_flow = both_flow;
                        println!("new max {} after {}", &max_flow, &count);
                    }
                } else {
                    let me_state = SingleState {
                        time_left: me_time_left,
                        flow: me_flow,
                        path: me_path,
                    };

                    let elephant_state = SingleState {
                        time_left: elephant_time_left,
                        flow: elephant_flow,
                        path: elephant_path,
                    };

                    if elephant_time_left > me_time_left {
                        if elephant_time_left > 0 {
                            stack.push(PairState {
                                me: state.me.clone(),
                                elephant: elephant_state,
                                options: options.clone(),
                            });
                        }

                        if me_time_left > 0 {
                            stack.push(PairState {
                                me: me_state,
                                elephant: state.elephant.clone(),
                                options: options.clone(),
                            });
                        }
                    } else {
                        if me_time_left > 0 {
                            stack.push(PairState {
                                me: me_state,
                                elephant: state.elephant.clone(),
                                options: options.clone(),
                            });
                        }
                        if elephant_time_left > 0 {
                            stack.push(PairState {
                                me: state.me.clone(),
                                elephant: elephant_state,
                                options: options.clone(),
                            });
                        }
                    }
                }
            }
        };
        discovered.insert(state);
    }

    max_flow.to_string()
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

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "1707");
    }
}
