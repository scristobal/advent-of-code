use std::collections::HashMap;

use petgraph::{algo::dijkstra, prelude::*};

fn parse(input: &str) -> (GraphMap<usize, (), Directed>, usize, usize, Vec<usize>) {
    let width = input.split_once('\n').unwrap().0.len();

    let input = &input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    let mut elevation = ('a'..='z')
        .enumerate()
        .map(|(v, c)| (c, v as u32))
        .collect::<HashMap<_, _>>();

    elevation.insert('S', *elevation.get(&'a').unwrap());
    elevation.insert('E', *elevation.get(&'z').unwrap());

    let map = input
        .chars()
        .map(|c| *elevation.get(&c).unwrap())
        .enumerate()
        .collect::<HashMap<_, _>>();

    let mut edges = Vec::new();

    for i in 0..map.len() {
        let e = *map.get(&i).unwrap();

        if i >= width && (e + 1) >= *map.get(&(i - width)).unwrap() {
            edges.push((i, i - width))
        }

        if (i + width) < map.len() && (e + 1) >= *map.get(&(i + width)).unwrap() {
            edges.push((i, i + width))
        }

        if i % width != 0 && (e + 1) >= *map.get(&(i - 1)).unwrap() {
            edges.push((i, i - 1))
        }

        if i % width != width - 1 && (e + 1) >= *map.get(&(i + 1)).unwrap() {
            edges.push((i, i + 1))
        }
    }

    let paths = DiGraphMap::<_, ()>::from_edges(edges.iter());

    let start = input.find(|c| c == 'S').unwrap();
    let end = input.find(|c| c == 'E').unwrap();

    let starts = map
        .iter()
        .filter_map(|(&i, &e)| if e == 0 { Some(i) } else { None })
        .collect::<Vec<_>>();

    (paths, start, end, starts)
}

pub fn solve_part1(input: &str) -> String {
    let (paths, start, end, _) = parse(input);

    let min_path = dijkstra(&paths, start, end.into(), |_| 1);

    min_path.get(&end).unwrap().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (paths, _, end, starts) = parse(input);

    starts
        .iter()
        .filter_map(|&s| {
            let min_path = dijkstra(&paths, s, end.into(), |_| 1);
            min_path.get(&end).cloned()
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "29");
    }
}
