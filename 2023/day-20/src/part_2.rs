/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pulse {
    origin: usize,
    destination: usize,
    pulse_type: bool, // high = true, low = false
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
struct FlipFlop {
    state: bool, // on = true, off = false
    input: usize,
    outputs: Vec<usize>,
}

impl FlipFlop {
    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.pulse_type {
            return vec![];
        }

        self.state = !self.state;

        self.outputs
            .iter()
            .map(|o| Pulse {
                origin: pulse.destination,
                destination: *o,
                pulse_type: self.state,
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Conjunction {
    last_inputs_state: Vec<bool>, // on = high, off = low
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

impl Conjunction {
    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let ind = self.inputs.iter().position(|i| *i == pulse.origin).unwrap();

        self.last_inputs_state[ind] = pulse.pulse_type;

        let pulse_type = !self.last_inputs_state.iter().all(|i| *i);

        self.outputs
            .iter()
            .map(|o| Pulse {
                origin: pulse.destination,
                destination: *o,
                pulse_type,
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Broadcaster {
    outputs: Vec<usize>,
}

impl Broadcaster {
    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.outputs
            .iter()
            .map(|o| Pulse {
                origin: pulse.destination,
                destination: *o,
                pulse_type: pulse.pulse_type,
            })
            .collect()
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]

struct Stub {
    inputs: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
    Stub(Stub),
}

impl Module {
    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match self {
            Module::FlipFlop(flip_flop) => flip_flop.handle_pulse(pulse),
            Module::Conjunction(conjunction) => conjunction.handle_pulse(pulse),
            Module::Broadcaster(broadcaster) => broadcaster.handle_pulse(pulse),
            Module::Stub(_) => vec![],
        }
    }
}

type Board = BTreeMap<usize, Module>;

fn parse(input: &str) -> (Board, Vec<usize>) {
    let mut inputs: HashMap<usize, Vec<usize>> = HashMap::new();

    let name_table: BTreeSet<_> = input
        .lines()
        .flat_map(|l| {
            let (desc, out_names) = l.split_once(" -> ").unwrap();

            let name = match desc.chars().next() {
                Some('%' | '&') => &desc[1..],
                _ => desc,
            };

            let out_names: Vec<_> = out_names.split(", ").collect();

            out_names.into_iter().chain(std::iter::once(name))
        })
        .collect();

    let mut board: BTreeMap<_, _> = input
        .lines()
        .map(|l| {
            let (desc, out) = l.split_once(" -> ").unwrap();

            let name = match desc.chars().next() {
                Some('%' | '&') => &desc[1..],
                _ => desc,
            };

            let name_key = name_table.iter().position(|n| *n == name).unwrap();

            let outputs: Vec<_> = out
                .split(", ")
                .map(|name| name_table.iter().position(|n| *n == name).unwrap())
                .collect();

            for name_out in &outputs {
                inputs
                    .entry(*name_out)
                    .and_modify(|out| out.push(name_key))
                    .or_insert(vec![name_key]);
            }

            let module = match desc.chars().next() {
                Some('%') => Module::FlipFlop(FlipFlop {
                    state: false,
                    input: 0,
                    outputs,
                }),
                Some('&') => Module::Conjunction(Conjunction {
                    last_inputs_state: Vec::new(),
                    inputs: Vec::new(),
                    outputs,
                }),
                Some(_) if name == "broadcaster" => Module::Broadcaster(Broadcaster { outputs }),
                e => panic!("Unknown type {:?} of name {} with id {}", e, name, name_key),
            };

            (name_key, module)
        })
        .collect();

    // complete board with modules that are in the output list but not in the input
    for i in 0..name_table.len() {
        board
            .entry(i)
            .or_insert(Module::Stub(Stub { inputs: vec![] }));
    }

    // update board with the lists of inputs
    board.iter_mut().for_each(|(key, module)| {
        let Some(out) = inputs.get(key) else {
            return;
        };

        match module {
            Module::FlipFlop(flip_flop) => {
                flip_flop.input = *out.first().unwrap();
            }
            Module::Conjunction(conjunction) => {
                conjunction.inputs = out.to_vec();
                conjunction.last_inputs_state = vec![false; out.len()]
            }
            Module::Stub(stub) => {
                stub.inputs = out.to_vec();
            }
            Module::Broadcaster(_) => {}
        }
    });

    // find precursors to target module 'rx'.
    // Unfortunately, this is input dependant, will panic on not carefully designed inputs :(
    let rx = name_table.iter().position(|n| *n == "rx").unwrap();

    let Some(Module::Stub(rx)) = board.get(&rx) else {
        panic!();
    };

    let Some(Module::Conjunction(precursors)) = board.get(rx.inputs.first().unwrap()) else {
        panic!();
    };

    let precursors = precursors
        .inputs
        .iter()
        .map(|precursor| {
            *board
                .iter()
                .find(|(_, module)| match module {
                    Module::Conjunction(conjunction) => {
                        conjunction.outputs.iter().any(|o| o == precursor)
                    }
                    _ => false,
                })
                .unwrap()
                .0
        })
        .collect();

    (board, precursors)
}

pub fn solve(input: &str) -> String {
    let (mut board, precursors) = parse(input);

    let broadcaster = *board
        .iter()
        .find(|(_, module)| matches!(module, Module::Broadcaster(_)))
        .unwrap()
        .0;

    let mut queue = VecDeque::new();

    queue.push_back(Pulse {
        origin: usize::MAX,
        destination: broadcaster,
        pulse_type: false,
    });

    let mut num_button_presses = 1;

    let mut cycles: HashMap<usize, usize> = HashMap::new();

    while cycles.len() != precursors.len() {
        match queue.pop_front() {
            Some(pulse) => {
                if let Some(module) = board.get_mut(&pulse.destination) {
                    queue.extend(module.handle_pulse(pulse));
                };

                for precursor_key in precursors.iter() {
                    let (_, precursor) = board.iter().find(|(k, _)| *k == precursor_key).unwrap();

                    if let Module::Conjunction(precursor) = precursor {
                        if precursor.last_inputs_state.iter().all(|s| *s) {
                            cycles.insert(*precursor_key, num_button_presses);
                        }
                    }
                }
            }
            None => {
                queue.push_back(Pulse {
                    origin: usize::MAX,
                    destination: broadcaster,
                    pulse_type: false,
                });

                num_button_presses += 1;
            }
        }
    }

    cycles
        .values()
        .fold(1_u128, |acc, item| num::integer::lcm(acc, *item as u128))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn solve_sample_1() {
        let result = solve(SAMPLE);
        assert_eq!(result, "32000000");
    }

    const SAMPLE_2: &str = include_str!("../sample2.txt");

    #[test]
    fn solve_sample_2() {
        let result = solve(SAMPLE_2);
        assert_eq!(result, "11687500");
    }
}
