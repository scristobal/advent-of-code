use std::collections::{HashSet, VecDeque};

use anyhow::Result;

#[derive(Eq, Hash, PartialEq)]
struct State {
    coords: (i32, i32),
    direction: (i32, i32),
    score: usize,
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut walls = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    walls.insert((x as i32, y as i32));
                }
                'S' => start = (x as i32, y as i32),
                'E' => end = (x as i32, y as i32),
                '.' => continue,
                _ => unreachable!(),
            }
        }
    }

    let mut min_score: usize = usize::MAX;

    let mut queue: VecDeque<State> = VecDeque::from([State {
        coords: start,
        direction: (1, 0),
        score: 0,
    }]);

    let mut visited: HashSet<State> = HashSet::new();

    while let Some(state) = queue.pop_back() {
        if let Some(prev_state) = visited
            .iter()
            .find(|s| s.coords == state.coords && s.direction == state.direction)
        {
            if prev_state.score <= state.score {
                continue;
            }
        }

        if state.coords == end && state.score < min_score {
            min_score = state.score;
            continue;
        };

        if state.score > min_score {
            continue;
        }

        let next_coords = (
            state.coords.0 + state.direction.0,
            state.coords.1 + state.direction.1,
        );

        if !walls.contains(&next_coords) {
            queue.push_back(State {
                coords: next_coords,
                direction: state.direction,
                score: state.score + 1,
            });
        }

        queue.push_front(State {
            coords: state.coords,
            direction: match state.direction {
                (1, 0) => (0, 1),
                (-1, 0) => (0, -1),
                (0, 1) => (-1, 0),
                (0, -1) => (1, 0),
                _ => unreachable!(),
            },
            score: state.score + 1_000,
        });

        queue.push_front(State {
            coords: state.coords,
            direction: match state.direction {
                (1, 0) => (0, -1),
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            },
            score: state.score + 1_000,
        });

        visited.insert(state);
    }

    Ok(min_score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        #[rustfmt::skip]
        let result = solve(
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
").unwrap();

        assert_eq!(result, "7036");
    }

    #[test]
    fn solve_sample2() {
        #[rustfmt::skip]
        let result = solve(
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
").unwrap();

        assert_eq!(result, "11048");
    }
}
