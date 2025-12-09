use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use petgraph::graph::UnGraph;
use std::collections::BinaryHeap;

fn main() {
    let s = include_str!("../../input/2025/day8.txt");

    println!("part 1: {}", solve_p1(s));
    println!("part 2: {}", solve_p2(s));
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Default)]
struct Coords(i64, i64, i64);

#[derive(Hash, PartialEq, Eq, Clone)]
struct Pair(Coords, Coords);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        distance(&other.0, &other.1).cmp(&distance(&self.0, &self.1))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_p1(s: &str) -> usize {
    let mut points: Vec<Coords> = vec![];

    for line in s.lines() {
        let mut numbers = line.split(",");

        let x = numbers.next().unwrap().parse().unwrap();
        let y = numbers.next().unwrap().parse().unwrap();
        let z = numbers.next().unwrap().parse().unwrap();

        points.push(Coords(x, y, z));
    }

    let mut distances = BinaryHeap::<Pair>::with_capacity(points.len() * points.len());

    for i in 0..points.len() {
        for j in 0..i {
            distances.push(Pair(points[i], points[j]));
        }
    }

    let mut edges = HashSet::with_capacity(points.len() * points.len());

    for _ in 0..1_000 {
        let p = distances.pop().unwrap();
        edges.insert(p);
    }

    let mut sizes: Vec<usize> = vec![];

    let mut remaining = points.clone();

    while let Some(point) = remaining.pop() {
        let mut visited: HashSet<Coords> = HashSet::new();
        let mut queue: Vec<Coords> = vec![point];

        while let Some(point) = queue.pop() {
            if !visited.contains(&point) {
                let e = edges.iter().filter(|p| p.0 == point || p.1 == point);

                for p in e {
                    queue.push(p.0);
                    queue.push(p.1);
                }
            }

            visited.insert(point);
        }

        sizes.push(visited.len());

        remaining.retain(|c| !visited.contains(c))
    }

    // dbg!(&sizes);

    sizes.sort();

    sizes.iter().rev().take(3).product()
}

fn distance(rhs: &Coords, lhs: &Coords) -> i64 {
    (rhs.0 - lhs.0).pow(2) + (rhs.1 - lhs.1).pow(2) + (rhs.2 - lhs.2).pow(2)
}

fn solve_p2(s: &str) -> usize {
    let mut points: Vec<Coords> = vec![];

    for line in s.lines() {
        let mut numbers = line.split(",");

        let x = numbers.next().unwrap().parse().unwrap();
        let y = numbers.next().unwrap().parse().unwrap();
        let z = numbers.next().unwrap().parse().unwrap();

        points.push(Coords(x, y, z));
    }

    let mut distances = BinaryHeap::<Pair>::with_capacity(points.len() * points.len());

    for i in 0..points.len() {
        for j in 0..i {
            distances.push(Pair(points[i], points[j]));
        }
    }

    let mut edges = HashSet::with_capacity(points.len() * points.len());

    let point = points.first().unwrap();

    while let Some(p) = distances.pop() {
        let q = p.clone();
        edges.insert(p);

        let mut visited: HashSet<Coords> = HashSet::new();
        let mut queue = vec![*point];

        while let Some(point) = queue.pop() {
            if !visited.contains(&point) {
                let e = edges.iter().filter(|p| p.0 == point || p.1 == point);

                for p in e {
                    queue.push(p.0);
                    queue.push(p.1);
                }
            }

            visited.insert(point);
        }

        if visited.len() == points.len() {
            return (q.0.0 * q.1.0) as usize;
        } else {
            // println!("{}", points.len() - visited.len())
        }
    }

    unreachable!();
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn sample_p1() {
        assert_eq!(solve_p1(SAMPLE), 40)
    }

    #[test]
    #[ignore]
    fn sample_p2() {
        assert_eq!(solve_p2(SAMPLE), 25272)
    }
}
