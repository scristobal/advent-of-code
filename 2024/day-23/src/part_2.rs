use anyhow::Result;
use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};

fn encode(a: &str) -> usize {
    a.chars().nth(0).unwrap().as_ascii().unwrap().to_u8() as usize * u8::MAX as usize
        + a.chars().nth(1).unwrap().as_ascii().unwrap().to_u8() as usize
}

fn decode(n: &usize) -> String {
    let a = (n % u8::MAX as usize) as u8;
    let b = (n / u8::MAX as usize) as u8;

    format!("{}{}", b.as_ascii().unwrap(), a.as_ascii().unwrap())
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut edges = HashSet::new();
    let mut nodes = HashSet::new();

    for (a, b) in input.lines().filter_map(|line| line.split_once("-")) {
        let a = encode(a);
        let b = encode(b);

        edges.insert((a, b));
        edges.insert((b, a));
        nodes.insert(a);
        nodes.insert(b);
    }

    let mut cliques: Vec<_> = nodes
        .iter()
        .combinations(3)
        .filter(|v| {
            edges.contains(&(*v[0], *v[1]))
                && edges.contains(&(*v[1], *v[2]))
                && edges.contains(&(*v[2], *v[0]))
        })
        .map(|v| v.into_iter().collect::<BTreeSet<_>>())
        .collect();

    let mut visited: HashSet<BTreeSet<&usize>> = HashSet::new();

    let mut party: BTreeSet<&usize> = BTreeSet::new();

    while let Some(clique) = cliques.pop() {
        if visited.contains(&clique) {
            continue;
        }
        'outher: for n in nodes.iter() {
            let mut maybe_clique = clique.clone();

            if !clique.contains(&n) {
                maybe_clique.insert(n);
            }

            if visited.contains(&maybe_clique) {
                continue;
            }

            for w in maybe_clique.iter().combinations(2) {
                if !edges.contains(&(**w[0], **w[1])) {
                    continue 'outher;
                }
            }

            if maybe_clique.len() > party.len() {
                party = maybe_clique.clone();
            }

            cliques.push(maybe_clique);
        }

        visited.insert(clique);
    }

    let admins = party.into_iter().map(|n| decode(&n)).sorted().join(",");

    Ok(admins)
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

        assert_eq!(result, "co,de,ka,ta");
    }

    #[test]
    fn encode_decode() {
        assert_eq!(decode(&encode(&"kh")), "kh")
    }
}
