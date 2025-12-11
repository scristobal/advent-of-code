use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use pathfinding::prelude::count_paths;
use petgraph::{algo::all_simple_paths, graph::DiGraph};
use rayon::collections::vec_deque;
use std::{collections::VecDeque, hash::RandomState, ops::Deref};

fn main() {
    let s = include_str!("../../input/2025/day11.txt");

    println!("part 1: {}", solve_p1(s));
    println!("part 2: {}", solve_p2(s));
}

fn solve_p1(s: &str) -> usize {
    let mut graph = DiGraph::<&str, i32>::new();

    let mut names = HashMap::new();

    for line in s.lines() {
        let mut tokens = line.split_whitespace();

        let origin = tokens.next().unwrap().trim_matches(|c| c == ':');

        let origin = *names
            .entry(origin)
            .or_insert_with(|| graph.add_node(origin));

        for target in tokens {
            let target = names
                .entry(target)
                .or_insert_with(|| graph.add_node(target));
            graph.add_edge(origin, *target, 1);
        }
    }

    let you = names.get("you").unwrap();
    let out = names.get("out").unwrap();

    all_simple_paths::<Box<_>, _, RandomState>(&graph, *you, *out, 0, None).count()
}

fn solve_p2(s: &str) -> usize {
    let mut adj = HashMap::new();

    for line in s.lines() {
        let mut tokens = line.split_whitespace();

        let origin = encode(tokens.next().unwrap().trim_matches(|c| c == ':'));
        let targets = tokens.map(|s| encode(s)).collect::<Box<_>>();

        adj.insert(origin, targets);
    }

    let svr = encode("svr");
    let dac = encode("dac");
    let fft = encode("fft");
    let out = encode("out");

    let empty = vec![].into_boxed_slice();

    let svr_dac = count_paths(&svr, |&n| adj.get(n).unwrap_or(&empty), |&n| *n == dac);
    let dac_fft = count_paths(&dac, |&n| adj.get(n).unwrap_or(&empty), |&n| *n == fft);
    let fft_out = count_paths(&fft, |&n| adj.get(n).unwrap_or(&empty), |&n| *n == out);

    let svr_fft = count_paths(&svr, |&n| adj.get(n).unwrap_or(&empty), |&n| *n == fft);
    let fft_dac = count_paths(&fft, |&n| adj.get(n).unwrap_or(&empty), |&n| *n == dac);
    let dac_out = count_paths(&dac, |&n| adj.get(n).unwrap_or(&empty), |&n| *n == out);

    svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out
}

fn encode(str: &str) -> u16 {
    str.as_bytes().iter().fold(0, |acc, b| {
        acc * (b'z' - b'a' + 1) as u16 + (*b as u16 - b'a' as u16)
    })
}

fn solve_p2_(s: &str) -> usize {
    let mut graph = DiGraph::<&str, i32>::new();

    let mut names = HashMap::new();

    for line in s.lines() {
        let mut segs = line.split_whitespace();

        let origin = segs.next().unwrap().trim_matches(|c| c == ':');

        let origin = *names
            .entry(origin)
            .or_insert_with(|| graph.add_node(origin));

        for target in segs {
            let target = names
                .entry(target)
                .or_insert_with(|| graph.add_node(target));
            graph.add_edge(origin, *target, 1);
        }
    }

    let svr = names.get("svr").unwrap();
    let dac = names.get("dac").unwrap();
    let fft = names.get("fft").unwrap();
    let out = names.get("out").unwrap();

    let svr_dac = all_simple_paths::<Box<_>, _, RandomState>(&graph, *svr, *dac, 0, None).count();
    let dac_fft = all_simple_paths::<Box<_>, _, RandomState>(&graph, *dac, *fft, 0, None).count();
    let fft_out = all_simple_paths::<Box<_>, _, RandomState>(&graph, *fft, *out, 0, None).count();

    let svr_fft = all_simple_paths::<Box<_>, _, RandomState>(&graph, *svr, *fft, 0, None).count();
    let fft_dac = all_simple_paths::<Box<_>, _, RandomState>(&graph, *fft, *dac, 0, None).count();
    let dac_out = all_simple_paths::<Box<_>, _, RandomState>(&graph, *dac, *out, 0, None).count();

    svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out
}

fn solve_p2__(s: &str) -> usize {
    let mut adj = HashMap::new();

    for line in s.lines() {
        let mut tokens = line.split_whitespace();

        let origin = encode(tokens.next().unwrap().trim_matches(|c| c == ':'));
        let targets = tokens.map(|s| encode(s)).collect::<Box<_>>();

        adj.insert(origin, targets);
    }

    let svr = encode("svr");
    let dac = encode("dac");
    let fft = encode("fft");
    let out = encode("out");

    let mut queue = VecDeque::from([(svr, false, false)]);

    let mut res = 0;
    while let Some((n, dac_flag, fft_flag)) = queue.pop_back() {
        match adj.get(&n) {
            Some(adjs) => adjs
                .iter()
                .for_each(|m| queue.push_back((*m, dac_flag || (n == dac), fft_flag || n == fft))),
            None if n == out && dac_flag && fft_flag => res += 1,
            _ => continue,
        };
    }
    res
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn sample_p1() {
        const SAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
        assert_eq!(solve_p1(SAMPLE), 5)
    }

    #[test]
    fn sample_p2() {
        const SAMPLE: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
        assert_eq!(solve_p2(SAMPLE), 2)
    }
}
