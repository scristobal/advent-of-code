use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn validate(pages: &[usize], page_rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut forbidden_pages = HashSet::<usize>::new();

    for page in pages {
        if forbidden_pages.contains(page) {
            return false;
        }

        if let Some(pages) = page_rules.get(page) {
            for page in pages {
                forbidden_pages.insert(*page);
            }
        };
    }
    true
}

pub fn solve(input: &'static str) -> Result<String> {
    let mut sections = input.split("\n\n");

    let rule_list = sections.next().unwrap();
    let pages_lists = sections.next().unwrap();

    let mut pages_rules = HashMap::<usize, Vec<usize>>::new();

    for line in rule_list.lines() {
        let mut pages = line.split("|").map(|v| v.parse().unwrap());

        let previous = pages.next().unwrap();
        let page = pages.next().unwrap();

        pages_rules
            .entry(page)
            .and_modify(|rule| rule.push(previous))
            .or_insert(vec![previous]);
    }

    let mut sum = 0;

    for line in pages_lists.lines() {
        let pages: Vec<usize> = line.split(",").map(|v| v.parse().unwrap()).collect();

        if validate(&pages, &pages_rules) {
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
