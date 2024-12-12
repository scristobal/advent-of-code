use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn validate(pages: &[usize], prevs: &HashMap<usize, Vec<usize>>) -> Option<usize> {
    let mut reqs = HashSet::<usize>::new();

    for (ind, page) in pages.iter().enumerate() {
        if reqs.contains(page) {
            return Some(ind);
        }

        if let Some(new_reqs) = prevs.get(page) {
            for req in new_reqs {
                reqs.insert(*req);
            }
        };
    }
    None
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut sections = input.split("\n\n");

    let rules = sections.next().unwrap();
    let pages = sections.next().unwrap();

    let mut prevs = HashMap::<usize, Vec<usize>>::new();

    for line in rules.lines() {
        let mut pages = line.split("|").map(|v| v.parse().unwrap());

        let prev = pages.next().unwrap();
        let page = pages.next().unwrap();

        prevs
            .entry(page)
            .and_modify(|prevs| prevs.push(prev))
            .or_insert(vec![prev]);
    }

    let mut sum = 0;

    for line in pages.lines() {
        let mut pages: Vec<usize> = line.split(",").map(|v| v.parse().unwrap()).collect();

        let was_invalid = validate(&pages, &prevs).is_some();

        while let Some(ind) = validate(&pages, &prevs) {
            let value = pages.remove(ind);
            pages.insert(ind - 1, value);
        }

        if was_invalid {
            sum += &pages[pages.len() / 2];
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE).unwrap();
        assert_eq!(result, "123");
    }
}
