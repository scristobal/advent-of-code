/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet, VecDeque};

struct Garden {
    rocks: HashSet<(isize, isize)>,
}

fn parse(input: &str) -> (Garden, (isize, isize)) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let input = input.replace('\n', "");

    let start = input
        .chars()
        .enumerate()
        .position(|(_, c)| c == 'S')
        .unwrap();

    let start: (isize, isize) = ((start % width) as isize, (start / width) as isize);

    let rocks: HashSet<_> = input
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i)
        .collect();

    let rocks: HashSet<(isize, isize)> = rocks
        .iter()
        .map(|i| ((i % width) as isize, (i / width) as isize))
        .map(|p| (p.0 - start.0, p.1 - start.1))
        .collect();

    (Garden { rocks }, (width as isize, height as isize))
}

fn generate_neighbors(position: (isize, isize)) -> Vec<(isize, isize)> {
    vec![
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ]
}

pub fn solve(input: &'static str) -> String {
    let (garden, (width, height)) = parse(input);

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let mut visited = HashMap::new();

    while let Some((coords, dist)) = queue.pop_front() {
        if let Vacant(e) = visited.entry(coords) {
            let neighbors = generate_neighbors(coords);

            for neighbor in neighbors {
                if !garden.rocks.contains(&neighbor)
                    && neighbor.0 >= -width / 2
                    && neighbor.1 >= -height / 2
                    && neighbor.0 <= width / 2
                    && neighbor.1 <= height / 2
                {
                    queue.push_back((neighbor, dist + 1));
                }
            }

            e.insert(dist);
        }
    }

    let visited_even_outside = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();

    let visited_even = visited.values().filter(|v| **v % 2 == 1).count();

    let visited_odd_outside = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let visited_odd = visited.values().filter(|v| **v % 2 == 0).count();

    let n = 26501365 / width as usize;

    let visited_inside = visited_even * (n + 1) * (n + 1) + visited_odd * n * n;
    let visited_outside = n * visited_even_outside - (n + 1) * visited_odd_outside;

    (visited_inside + visited_outside).to_string()
}
