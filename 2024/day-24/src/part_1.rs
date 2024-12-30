use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

enum Node<'a> {
    Value(bool),
    Result((&'a str, &'a str, &'a str)),
}

struct Logic<'a>(HashMap<&'a str, Node<'a>>);

impl<'a> Logic<'a> {
    fn eval(&self, n: &'a str, cache: &mut HashMap<&'a str, bool>) -> bool {
        match self.0.get(n).unwrap() {
            Node::Value(value) => *value,
            Node::Result((op, lhs, rhs)) => {
                if let Some(res) = cache.get(&n) {
                    return *res;
                }

                let rhs = self.eval(*rhs, cache);
                let lhs = self.eval(*lhs, cache);

                let res = match *op {
                    "AND" => rhs && lhs,
                    "OR" => rhs || lhs,
                    "XOR" => rhs ^ lhs,
                    _ => unreachable!(),
                };

                cache.insert(n, res);

                res
            }
        }
    }
}

pub fn solve(input: &'static str) -> Result<String> {
    let (constants, operations) = input.split_once("\n\n").unwrap();

    let mut logic = Logic(HashMap::new());

    for line in constants.lines() {
        let (name, value) = line.split_once(": ").unwrap();

        let value = match value {
            "1" => true,
            "0" => false,
            _ => unreachable!(),
        };

        let n = Node::Value(value);

        logic.0.insert(name, n);
    }

    for line in operations.lines() {
        let (operation, result) = line.split_once(" -> ").unwrap();
        let [lhs, op, rhs, ..] = operation.split(" ").collect::<Vec<_>>()[..] else {
            unreachable!()
        };

        logic.0.insert(&result, Node::Result((op, lhs, rhs)));
    }

    let mut cache = HashMap::new();

    let res: u64 = logic
        .0
        .keys()
        .filter(|k| k.starts_with("z"))
        .sorted()
        .map(|i| logic.eval(i, &mut cache))
        .enumerate()
        .filter(|(_, p)| *p)
        .map(|(d, _)| 2_u64.pow(d as u32))
        .sum();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample1() {
        #[rustfmt::skip]
        let result = solve(
"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
").unwrap();

        assert_eq!(result, "4");
    }

    #[test]
    fn solve_sample2() {
        #[rustfmt::skip]
        let result = solve(
"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
").unwrap();

        assert_eq!(result, "2024");
    }
}
