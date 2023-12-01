use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};

#[derive(Debug)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(n), Self::Int(m)) => n == m,
            (Self::List(l), Self::List(r)) => l == r,
            (Self::Int(n), Self::List(l)) => *l == vec![Self::Int(*n)],
            (Self::List(l), Self::Int(n)) => *l == vec![Self::Int(*n)],
        }
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(n), Self::Int(m)) => n.cmp(m),
            (Self::List(l), Self::List(r)) => l.cmp(r),
            (Self::Int(n), Self::List(l)) => vec![Self::Int(*n)].cmp(l),
            (Self::List(l), Self::Int(n)) => l.cmp(&vec![Self::Int(*n)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn packet(s: &str) -> IResult<&str, Packet> {
    alt((
        complete::u8.map(Packet::Int),
        delimited(tag("["), separated_list0(tag(","), packet), tag("]")).map(Packet::List),
    ))(s)
}

fn packet_pair(s: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(packet, newline, packet)(s)
}

fn packet_pairs_list(s: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list1(tag("\n\n"), packet_pair)(s)
}

pub fn solve_part1(input: &str) -> String {
    packet_pairs_list(input)
        .unwrap()
        .1
        .iter()
        .enumerate()
        .map(|(p, v)| (p + 1, v))
        .filter_map(|(p, (left, right))| match left.cmp(right) {
            std::cmp::Ordering::Less => Some(p),
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Greater => None,
        })
        .sum::<usize>()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let packet_paris = packet_pairs_list(input).unwrap().1;

    let mut packet_list = packet_paris
        .iter()
        .flat_map(|(left, right)| [left, right])
        .collect::<Vec<_>>();

    let (_, divider_1) = packet("[[2]]").unwrap();
    let (_, divider_2) = packet("[[6]]").unwrap();

    packet_list.push(&divider_1);
    packet_list.push(&divider_2);

    packet_list.sort();

    let (index_1, _) = packet_list
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .find(|(_, &p)| p == &divider_1)
        .unwrap();

    let (index_2, _) = packet_list
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .find(|(_, &p)| p == &divider_2)
        .unwrap();

    (index_1 * index_2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_works() {
        let result = solve_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = solve_part2(INPUT);
        assert_eq!(result, "140");
    }
}
