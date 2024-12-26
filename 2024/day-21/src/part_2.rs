use anyhow::Result;
use bimap::BiHashMap;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Hash)]
struct NumpadState {
    coord: (i32, i32),
    last_instr: char,
    input_len: usize,
}

fn numpad_successors(
    state: &NumpadState,
    code: &Vec<char>,
    inception: usize,
    numpad: &BiHashMap<char, (i32, i32)>,
    arrowpad: &BiHashMap<char, (i32, i32)>,
    dirs: &HashMap<char, (i32, i32)>,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> Vec<(NumpadState, usize)> {
    let mut res = vec![];

    for ins in arrowpad.left_values() {
        match ins {
            '^' | 'v' | '<' | '>' => {
                let delta = dirs.get(ins).unwrap();
                let coord_next = (state.coord.0 + delta.0, state.coord.1 + delta.1);

                if numpad.contains_right(&coord_next) {
                    res.push((
                        NumpadState {
                            coord: coord_next,
                            last_instr: *ins,
                            input_len: state.input_len,
                        },
                        ins_cost(ins, &state.last_instr, inception, arrowpad, dirs, cache),
                    ))
                }
            }
            'A' => {
                if *numpad.get_by_right(&state.coord).unwrap() == code[state.input_len] {
                    res.push((
                        NumpadState {
                            coord: state.coord,
                            last_instr: *ins,
                            input_len: state.input_len + 1,
                        },
                        ins_cost(ins, &state.last_instr, inception, arrowpad, dirs, cache),
                    ));
                }
            }
            _ => unreachable!(),
        }
    }

    res
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct ArrowpadState {
    coord: (i32, i32),
    last_instr: char,
    pressed: bool,
}

fn arrowpad_successors(
    state: &ArrowpadState,
    inception: usize,
    arrowpad: &BiHashMap<char, (i32, i32)>,
    dirs: &HashMap<char, (i32, i32)>,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> Vec<(ArrowpadState, usize)> {
    let mut res = vec![];

    for ins in arrowpad.left_values() {
        match ins {
            '^' | 'v' | '<' | '>' => {
                let delta = dirs.get(ins).unwrap();
                let coord_next = (state.coord.0 + delta.0, state.coord.1 + delta.1);

                if arrowpad.contains_right(&coord_next) {
                    res.push((
                        ArrowpadState {
                            coord: coord_next,
                            last_instr: *ins,
                            pressed: false,
                        },
                        ins_cost(ins, &state.last_instr, inception - 1, arrowpad, dirs, cache),
                    ));
                }
            }
            'A' => res.push((
                ArrowpadState {
                    coord: state.coord,
                    last_instr: *ins,
                    pressed: true,
                },
                ins_cost(ins, &state.last_instr, inception - 1, arrowpad, dirs, cache),
            )),
            _ => unreachable!(),
        }
    }

    res
}

fn ins_cost(
    end: &char,
    start: &char,
    inception: usize,
    arrowpad: &BiHashMap<char, (i32, i32)>,
    dirs: &HashMap<char, (i32, i32)>,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if inception == 0 {
        return 1;
    }

    if let Some(res) = cache.get(&(*end, *start, inception)) {
        return *res;
    }

    let init = ArrowpadState {
        coord: *arrowpad.get_by_left(start).unwrap(),
        last_instr: 'A',
        pressed: false,
    };

    let (_, cost) = dijkstra(
        &init,
        |s| arrowpad_successors(s, inception, arrowpad, dirs, cache),
        |s| s.pressed == true && arrowpad.get_by_right(&s.coord).unwrap() == end,
    )
    .unwrap();

    cache.insert((*end, *start, inception), cost);

    cost
}

pub fn solve(input: &'static str) -> Result<String> {
    let inception = 25;

    let mut numpad = BiHashMap::new();

    for (y, ln) in "789\n456\n123\n 0A".lines().enumerate() {
        for (x, ch) in ln.chars().enumerate().filter(|&(_, ch)| ch != ' ') {
            numpad.insert(ch, (x as i32, y as i32));
        }
    }

    let mut arrowpad = BiHashMap::new();

    for (y, ln) in " ^A\n<v>".lines().enumerate() {
        for (x, ch) in ln.chars().enumerate().filter(|&(_, ch)| ch != ' ') {
            arrowpad.insert(ch, (x as i32, y as i32));
        }
    }

    let dirs = HashMap::from([('^', (0, -1)), ('v', (0, 1)), ('>', (1, 0)), ('<', (-1, 0))]);

    let mut res = 0;

    let mut cache = HashMap::new();

    for line in input.lines() {
        let code: Vec<char> = line.chars().collect();

        let init = NumpadState {
            coord: *numpad.get_by_left(&'A').unwrap(),
            last_instr: 'A',
            input_len: 0,
        };

        let (_, cost) = dijkstra(
            &init,
            |s| numpad_successors(s, &code, inception, &numpad, &arrowpad, &dirs, &mut cache),
            |s| s.input_len == code.len(),
        )
        .unwrap();

        let code = line.strip_suffix("A").unwrap().parse::<usize>().unwrap();

        res += code * cost;
    }

    Ok(res.to_string())
}
