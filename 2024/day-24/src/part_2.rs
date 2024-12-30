use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const BITS: usize = 44;

#[derive(Debug)]
struct Gate<'a> {
    op: &'a str,
    lhs: &'a str,
    rhs: &'a str,
}

struct Board<'a>(HashMap<&'a str, Gate<'a>>);

// Ripple carry adder
//
//  Each block is a function
//      (x_in, y_in, c_in) -> (z_out, c_out)
//  defined by:
//      (1) x_in XOR y_in -> s
//      (2) s XOR c_in -> z_out
//      (3) x_in AND y_in -> a
//      (4) s AND c_in -> b
//      (5) a OR b -> c_out
//
// blocks are connected by carrier bit
//      (prev) c_out -> (next) c_in

impl<'a> Board<'a> {
    // z_out should be the result of (1)
    fn get_z(&self) -> Vec<&'a str> {
        self.0
            .iter()
            .filter(|(_, g)| g.op == "XOR" && (!g.rhs.starts_with("x") && !g.lhs.starts_with("x")))
            .map(|(k, _)| *k)
            .collect()
    }

    // z_out has no implicit equations, but it is determined by name
    fn check_z(&self, n: &'a str) -> bool {
        n.starts_with("z")
    }

    // c_out should be the result of (5)
    fn get_c(&self) -> Vec<&'a str> {
        self.0
            .iter()
            .filter(|(_, v)| v.op == "OR")
            .map(|(k, _)| *k)
            .collect()
    }

    // c_out should be implicit in (4) and (2), except the very last
    fn check_c(&self, n: &'a str) -> bool {
        let a = self
            .0
            .values()
            .find(|g| g.op == "AND" && (g.lhs == n || g.rhs == n))
            .map(|g| if g.lhs == n { g.rhs } else { g.lhs });

        let b = self
            .0
            .values()
            .find(|g| g.op == "XOR" && (g.lhs == n || g.rhs == n))
            .map(|g| if g.lhs == n { g.rhs } else { g.lhs });

        b.is_some() && a.is_some() && b == a || n.ends_with(&(BITS + 1).to_string())
    }

    // b should be the result of (4)
    fn get_b(&self) -> Vec<&'a str> {
        self.0
            .iter()
            .filter(|(_, g)| g.op == "AND" && (!g.lhs.starts_with("x") && !g.rhs.starts_with("x")))
            .map(|(k, _)| *k)
            .collect()
    }

    // b should be implicit in (5)
    fn check_b(&self, n: &'a str) -> bool {
        self.0
            .values()
            .find(|g| g.op == "OR" && (g.lhs == n || g.rhs == n))
            .is_some()
    }

    // a should be result of (3)
    fn get_a(&self) -> Vec<&'a str> {
        self.0
            .iter()
            .filter(|(_, g)| g.op == "AND" && (g.lhs.starts_with("x") || g.rhs.starts_with("x")))
            .map(|(k, _)| *k)
            .collect()
    }

    // a should be implitic of (5), unless is the first adder
    fn check_a(&self, n: &'a str) -> bool {
        self.0
            .values()
            .find(|g| {
                (g.op == "OR" && (g.lhs == n || g.rhs == n))
                    || (self.0.get(n).unwrap().lhs.ends_with("00"))
            })
            .is_some()
    }

    // s should be result of (1)
    fn get_s(&self) -> Vec<&'a str> {
        self.0
            .iter()
            .filter(|(_, g)| g.op == "XOR" && (g.rhs.starts_with("x") || g.lhs.starts_with("x")))
            .map(|(k, _)| *k)
            .collect()
    }

    //  s should be implicit in (2) and (4), except for first adder
    fn check_s(&self, n: &'a str) -> bool {
        let a = self
            .0
            .values()
            .find(|g| g.op == "AND" && (g.lhs == n || g.rhs == n))
            .map(|g| if g.lhs == n { g.rhs } else { g.lhs });

        let b = self
            .0
            .values()
            .find(|g| g.op == "XOR" && (g.lhs == n || g.rhs == n))
            .map(|g| if g.lhs == n { g.rhs } else { g.lhs });

        b.is_some() && a.is_some() && b == a || n.ends_with("00")
    }
}

pub fn solve(input: &'static str) -> Result<String> {
    let (_, operations) = input.split_once("\n\n").unwrap();

    let mut board = Board(HashMap::new());

    for line in operations.lines() {
        let (operation, out) = line.split_once(" -> ").unwrap();
        let [lhs, op, rhs, ..] = operation.split(" ").collect::<Vec<_>>()[..] else {
            unreachable!()
        };

        board.0.insert(&out, Gate { op, lhs, rhs });
    }

    let mut swaps: HashSet<&str> = HashSet::new();

    board
        .get_z()
        .iter()
        .filter(|g| !board.check_z(g))
        .for_each(|g| {
            swaps.insert(g);
        });

    board
        .get_c()
        .iter()
        .filter(|g| !board.check_c(g))
        .for_each(|g| {
            swaps.insert(g);
        });

    board
        .get_b()
        .iter()
        .filter(|g| !board.check_b(g))
        .for_each(|g| {
            swaps.insert(g);
        });

    board
        .get_a()
        .iter()
        .filter(|g| !board.check_a(g))
        .for_each(|g| {
            swaps.insert(g);
        });

    board
        .get_s()
        .iter()
        .filter(|g| !board.check_s(g))
        .for_each(|g| {
            swaps.insert(g);
        });

    Ok(swaps.iter().sorted().join(",").to_string())
}
