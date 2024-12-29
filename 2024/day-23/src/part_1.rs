use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: &'static str) -> Result<String> {
    let mut edges = HashSet::new();
    let mut nodes = HashSet::new();

    for (a, b) in input.lines().filter_map(|line| line.split_once("-")) {
        edges.insert((a, b));
        edges.insert((b, a));
        nodes.insert(a);
        nodes.insert(b);
    }

    let count = nodes
        .into_iter()
        .combinations(3)
        .filter(|v| {
            edges.contains(&(v[0], v[1]))
                && edges.contains(&(v[1], v[2]))
                && edges.contains(&(v[2], v[0]))
                && (v[0].starts_with("t") || v[1].starts_with("t") || v[2].starts_with("t"))
        })
        .count();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        #[rustfmt::skip]
        let result = solve(
"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
").unwrap();

        assert_eq!(result, "7");
    }
}
