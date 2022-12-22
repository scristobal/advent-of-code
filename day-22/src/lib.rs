use nom::{branch::alt, bytes::complete::tag, character::complete, multi::many1, IResult, *};
use std::collections::HashMap;

#[derive(Debug)]
enum Fill {
    Rock,
    Air,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orientation {
    North,
    East,
    South,
    West,
}

use Orientation::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: (i32, i32),
    facing: Orientation,
}

impl State {
    fn step(&self, mov: Move, ctx: &Map) -> Self {
        match mov {
            Move::Forward(steps) => {
                let mut next_state = *self;
                let mut state = *self;

                for _ in 1..=steps {
                    next_state = state.step_forward(ctx);
                    if state == next_state {
                        break;
                    }
                    state = next_state;
                }

                next_state
            }
            Move::Left => State {
                facing: match self.facing {
                    North => West,
                    East => North,
                    South => East,
                    West => South,
                },
                position: self.position,
            },
            Move::Right => State {
                facing: match self.facing {
                    North => East,
                    East => South,
                    South => West,
                    West => North,
                },
                position: self.position,
            },
        }
    }

    fn step_forward(&self, ctx: &Map) -> State {
        let wrapped = match ctx.is_flat {
            true => self.flat_wrapper(ctx),
            false => self.space_wrapper(ctx),
        };

        // println!("------");
        // dbg!(&self, &wrapped, ctx.fill.get(&wrapped.position));

        match ctx.fill.get(&wrapped.position) {
            Some(fill) => match fill {
                Fill::Rock => *self,
                Fill::Air => wrapped,
            },
            _ => unreachable!(),
        }
    }

    fn space_wrapper(&self, ctx: &Map) -> State {
        let test = State {
            position: match self.facing {
                North => (self.position.0, self.position.1 - 1),
                East => (self.position.0 + 1, self.position.1),
                South => (self.position.0, self.position.1 + 1),
                West => (self.position.0 - 1, self.position.1),
            },
            facing: self.facing,
        };

        if ctx.fill.get(&test.position).is_some() {
            return test;
        }

        let size = 50;

        match self.facing {
            North => {
                if self.position.0 <= size {
                    State {
                        position: (size + 1, self.position.0 + size),
                        facing: East,
                    }
                } else if self.position.0 <= 2 * size {
                    State {
                        position: (1, self.position.0 + 2 * size),
                        facing: East,
                    }
                } else {
                    State {
                        position: (self.position.0 - 2 * size, 4 * size),
                        facing: North,
                    }
                }
            }
            East => {
                if self.position.1 <= size {
                    State {
                        position: (2 * size, 3 * size - self.position.1 + 1),
                        facing: West,
                    }
                } else if self.position.1 <= 2 * size {
                    State {
                        position: (self.position.1 + size, size),
                        facing: North,
                    }
                } else if self.position.1 <= 3 * size {
                    State {
                        position: (3 * size, 3 * size - self.position.1 + 1),
                        facing: West,
                    }
                } else {
                    State {
                        position: (self.position.1 - 2 * size, 3 * size),
                        facing: North,
                    }
                }
            }
            South => {
                if self.position.0 <= size {
                    State {
                        position: (self.position.0 + 2 * size, 1),
                        facing: South,
                    }
                } else if self.position.0 <= 2 * size {
                    State {
                        position: (size, self.position.0 + 2 * size),
                        facing: West,
                    }
                } else {
                    State {
                        position: (2 * size, self.position.0 - size),
                        facing: West,
                    }
                }
            }
            West => {
                if self.position.1 <= size {
                    State {
                        position: (1, 3 * size - self.position.1 + 1),
                        facing: East,
                    }
                } else if self.position.1 <= 2 * size {
                    State {
                        position: (self.position.1 - size, 2 * size + 1),
                        facing: South,
                    }
                } else if self.position.1 <= 3 * size {
                    State {
                        position: (size + 1, 3 * size - self.position.1 + 1),
                        facing: East,
                    }
                } else {
                    State {
                        position: (self.position.1 - 2 * size, 1),
                        facing: South,
                    }
                }
            }
        }
    }

    fn flat_wrapper(&self, ctx: &Map) -> State {
        let test = State {
            position: match self.facing {
                North => (self.position.0, self.position.1 - 1),
                East => (self.position.0 + 1, self.position.1),
                South => (self.position.0, self.position.1 + 1),
                West => (self.position.0 - 1, self.position.1),
            },
            facing: self.facing,
        };

        if ctx.fill.get(&test.position).is_some() {
            return test;
        }

        match self.facing {
            North => {
                let max_row = *ctx
                    .fill
                    .keys()
                    .filter_map(|(col, row)| {
                        if *col == test.position.0 {
                            Some(row)
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap();
                State {
                    position: (test.position.0, max_row),
                    facing: self.facing,
                }
            }
            East => {
                let min_col = *ctx
                    .fill
                    .keys()
                    .filter_map(|(col, row)| {
                        if *row == test.position.1 {
                            Some(col)
                        } else {
                            None
                        }
                    })
                    .min()
                    .unwrap();
                State {
                    position: (min_col, test.position.1),
                    facing: self.facing,
                }
            }
            South => {
                let min_row = *ctx
                    .fill
                    .keys()
                    .filter_map(|(col, row)| {
                        if *col == test.position.0 {
                            Some(row)
                        } else {
                            None
                        }
                    })
                    .min()
                    .unwrap();
                State {
                    position: (test.position.0, min_row),
                    facing: self.facing,
                }
            }
            West => {
                let max_col = *ctx
                    .fill
                    .keys()
                    .filter_map(|(col, row)| {
                        if *row == test.position.1 {
                            Some(col)
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap();
                State {
                    position: (max_col, test.position.1),
                    facing: self.facing,
                }
            }
        }
    }
}

#[derive(Debug)]
enum Move {
    Forward(i32),
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    fill: HashMap<(i32, i32), Fill>,
    is_flat: bool,
}

fn print(states: &[State], map: &Map) {
    let max_row = *map.fill.keys().map(|(_, row)| row).max().unwrap();
    let max_col = *map.fill.keys().map(|(col, _)| col).max().unwrap();

    let mut s = "".to_string();

    for row in 1..=max_row {
        for col in 1..=max_col {
            let state = states
                .iter()
                .filter(|state| state.position.0 == col && state.position.1 == row)
                .last();

            match state {
                Some(state) => match state.facing {
                    North => {
                        s += "^";
                    }
                    East => {
                        s += ">";
                    }
                    South => {
                        s += "v";
                    }
                    West => {
                        s += "<";
                    }
                },
                None => {
                    let fill = map.fill.get(&(col, row));

                    match fill {
                        Some(fill) => match fill {
                            Fill::Rock => {
                                s += "#";
                            }
                            Fill::Air => {
                                s += ".";
                            }
                        },
                        None => {
                            s += " ";
                        }
                    }
                }
            }
        }
        s += "\n";
    }

    println!("{s}");
}

fn moves_parser(s: &str) -> IResult<&str, Vec<Move>> {
    let (s, moves) = many1(alt((
        complete::i32.map(|steps| (1..=steps).map(|_| Move::Forward(1)).collect()),
        alt((tag("R"), tag("L"))).map(|c| match c {
            "R" => vec![Move::Right],
            "L" => vec![Move::Left],
            _ => unreachable!(),
        }),
    )))(s)?;

    Ok((s, moves.into_iter().flatten().collect()))

    //   let (s, moves) = many1(alt((
    //     complete::i32.map(Move::Forward),
    //     alt((tag("R"), tag("L"))).map(|c| match c {
    //         "R" => Move::Right,
    //         "L" => Move::Left,
    //         _ => unreachable!(),
    //     }),
    // )))(s)?;

    // Ok((s, moves))
}

fn map_parser(s: &str) -> HashMap<(i32, i32), Fill> {
    let mut col = 1;
    let mut row = 1;

    let mut fill = HashMap::new();

    for c in s.chars() {
        match c {
            '.' => {
                fill.insert((col, row), Fill::Air);
                col += 1;
            }
            '#' => {
                fill.insert((col, row), Fill::Rock);
                col += 1;
            }
            '\n' => {
                col = 1;
                row += 1;
            }
            ' ' => {
                col += 1;
            }
            _ => {}
        }
    }

    fill
}

fn parse(s: &str, is_flat: bool) -> (Vec<Move>, Map) {
    let s = s.split("\n\n").collect::<Vec<_>>();

    let map = s.first().unwrap();

    let map = map_parser(map);

    let map = Map { fill: map, is_flat };

    let moves = *s.last().unwrap();

    let moves = moves_parser(moves).unwrap().1;

    (moves, map)
}

pub fn solve_part1(input: &str) -> String {
    let (moves, map) = parse(input, true);

    let mut moves = moves.into_iter().rev().collect::<Vec<_>>();

    let min_col = *map
        .fill
        .keys()
        .filter_map(|(col, row)| if *row == 1 { Some(col) } else { None })
        .min()
        .unwrap();

    let mut states = vec![State {
        position: (min_col, 1),
        facing: East,
    }];

    while let Some(mov) = moves.pop() {
        let state = states.last().unwrap();
        states.push(state.step(mov, &map));
    }

    let last_state = states.last().unwrap();

    let password = last_state.position.1 * 1_000
        + last_state.position.0 * 4
        + match last_state.facing {
            North => 3,
            East => 0,
            South => 1,
            West => 2,
        };

    //  print(&states, &map);

    password.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (moves, map) = parse(input, false);

    let mut moves = moves.into_iter().rev().collect::<Vec<_>>();

    let mut states = vec![State {
        position: (51, 1),
        facing: East,
    }];

    while let Some(mov) = moves.pop() {
        let state = states.last().unwrap();
        states.push(state.step(mov, &map));
    }

    let last_state = states.last().unwrap();

    let password = last_state.position.1 * 1_000
        + last_state.position.0 * 4
        + match last_state.facing {
            North => 3,
            East => 0,
            South => 1,
            West => 2,
        };

    // print(&states, &map);

    password.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "6032");
    }

    const INPUT2: &str = include_str!("../sample2.txt");

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT2);
        assert_eq!(result, "5031");
    }

    const INPUT_COMPLETE: &str = include_str!("../input.txt");

    #[test]
    fn visual_1() {
        let (_, map) = parse(INPUT_COMPLETE, false);

        // North
        let states = (1..=4)
            .map(|col| State {
                position: (col, 101),
                facing: North,
            })
            .collect::<Vec<_>>();

        let states = (51..=54)
            .map(|col| State {
                position: (col, 1),
                facing: North,
            })
            .collect::<Vec<_>>();

        let states = (101..=104)
            .map(|col| State {
                position: (col, 1),
                facing: North,
            })
            .collect::<Vec<_>>();

        // //South
        let states = (1..=4)
            .map(|col| State {
                position: (col, 200),
                facing: South,
            })
            .collect::<Vec<_>>();

        let states = (51..=54)
            .map(|col| State {
                position: (col, 150),
                facing: South,
            })
            .collect::<Vec<_>>();

        let states = (101..=104)
            .map(|col| State {
                position: (col, 50),
                facing: South,
            })
            .collect::<Vec<_>>();

        //West
        let states = (1..=4)
            .map(|row| State {
                position: (51, row),
                facing: West,
            })
            .collect::<Vec<_>>();

        let states = (51..=54)
            .map(|row| State {
                position: (51, row),
                facing: West,
            })
            .collect::<Vec<_>>();

        let states = (101..=104)
            .map(|row| State {
                position: (1, row),
                facing: West,
            })
            .collect::<Vec<_>>();

        let states = (151..=154)
            .map(|row| State {
                position: (1, row),
                facing: West,
            })
            .collect::<Vec<_>>();

        //East
        let states = (1..=4)
            .map(|row| State {
                position: (150, row),
                facing: East,
            })
            .collect::<Vec<_>>();

        let states = (51..=54)
            .map(|row| State {
                position: (100, row),
                facing: East,
            })
            .collect::<Vec<_>>();

        let states = (101..=104)
            .map(|row| State {
                position: (100, row),
                facing: East,
            })
            .collect::<Vec<_>>();

        let states = (151..=154)
            .map(|row| State {
                position: (50, row),
                facing: East,
            })
            .collect::<Vec<_>>();

        let output_states = states
            .iter()
            .map(|state| state.space_wrapper(&map))
            .collect::<Vec<_>>();

        let states = states
            .iter()
            .chain(output_states.iter())
            .copied()
            .collect::<Vec<_>>();

        print(&states, &map);
    }
}
