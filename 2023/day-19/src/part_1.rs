/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::str::FromStr;
use std::{cmp, collections::HashMap};

#[derive(PartialEq, Eq, Debug, Hash)]
enum Field {
    X,
    M,
    A,
    S,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "x" => Ok(X),
            "m" => Ok(M),
            "a" => Ok(A),
            "s" => Ok(S),
            _ => Err(()),
        }
    }
}

use Field::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Classification {
    Accepted,
    Rejected,
    Continue(String),
}

impl FromStr for Classification {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(Accepted),
            "R" => Ok(Rejected),
            s => Ok(Continue(s.to_string())),
        }
    }
}

use Classification::*;

#[derive(PartialEq, Eq, Debug, Hash)]
struct Rule {
    field: Field,
    op: cmp::Ordering,
    value: usize,
    success: Classification,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Workflow {
    rules: Vec<Rule>,
    fall_back: Classification,
}

#[derive(PartialEq, Eq, Debug)]
struct ProcessingPlant {
    workflows: HashMap<String, Workflow>,
}

#[derive(PartialEq, Eq, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

// eg. "s<537:gd"
fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, ((field, op, value), success)) = separated_pair(
        tuple((alpha1, alt((tag(">"), tag("<"))), complete::u32)),
        tag(":"),
        alpha1,
    )(input)?;

    let rule = Rule {
        field: field.parse().unwrap(),
        op: match op {
            "<" => cmp::Ordering::Less,
            ">" => cmp::Ordering::Greater,
            _ => panic!(),
        },
        value: value as usize,
        success: success.parse().unwrap(),
    };

    Ok((input, rule))
}

// eg. "rfg{s<537:gd,x>2440:R,A}"
fn parse_workflow(input: &str) -> IResult<&str, (String, Workflow)> {
    let (input, (name, (rules, fall_back))) = tuple((
        alpha1,
        delimited(
            tag("{"),
            tuple((
                separated_list1(tag(","), parse_rule),
                preceded(tag(","), alt((alpha1, tag("A"), tag("R")))),
            )),
            tag("}"),
        ),
    ))(input)?;

    let workflow = Workflow {
        rules,
        fall_back: fall_back.parse().unwrap(),
    };

    let name = name.to_string();

    Ok((input, (name, workflow)))
}

// eg. "x=787"
fn parse_field(input: &str) -> IResult<&str, (Field, usize)> {
    let (input, (field, value)) = separated_pair(alpha1, tag("="), complete::u32)(input)?;

    let field = field.parse().unwrap();

    Ok((input, (field, value as usize)))
}

// eg. "{x=787,m=2655,a=1222,s=2876}"
fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, rules) =
        delimited(tag("{"), separated_list1(tag(","), parse_field), tag("}"))(input)?;

    let part = Part {
        x: rules.iter().find(|(f, _)| f == &X).unwrap().1,
        m: rules.iter().find(|(f, _)| f == &M).unwrap().1,
        a: rules.iter().find(|(f, _)| f == &A).unwrap().1,
        s: rules.iter().find(|(f, _)| f == &S).unwrap().1,
    };

    Ok((input, part))
}

// eg. "gd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}"
fn parse_input(input: &str) -> IResult<&str, (ProcessingPlant, Vec<Part>)> {
    let (input, (workflows, parts)) = separated_pair(
        separated_list1(tag("\n"), parse_workflow),
        tag("\n\n"),
        separated_list1(tag("\n"), parse_part),
    )(input)?;

    let plant = ProcessingPlant {
        workflows: workflows.into_iter().collect(),
    };

    Ok((input, (plant, parts)))
}

impl Rule {
    // return Some if rule check was successful (eg. true) or None otherwise
    fn process(&self, part: &Part) -> Option<Classification> {
        match self.field {
            X if part.x.cmp(&self.value) == self.op => Some(self.success.clone()),
            M if part.m.cmp(&self.value) == self.op => Some(self.success.clone()),
            A if part.a.cmp(&self.value) == self.op => Some(self.success.clone()),
            S if part.s.cmp(&self.value) == self.op => Some(self.success.clone()),
            _ => None,
        }
    }
}

impl Workflow {
    // chains a set of rules defined by the workflow, if a rule fails, it continues, otherwise returns Continue(next_rule_id), Accept or Reject
    fn process(&self, part: &Part) -> Classification {
        for rule in self.rules.iter() {
            match rule.process(part) {
                Some(next) => return next,
                None => continue,
            }
        }

        self.fall_back.clone()
    }
}

impl ProcessingPlant {
    // returns true if accepted, false otherwise
    fn process(&self, part: &Part) -> bool {
        let Some(mut workflow) = self.workflows.get("in") else {
            panic!("Failed to find `in` workflow, check parsing/input data");
        };

        loop {
            match workflow.process(part) {
                Accepted => return true,
                Rejected => return false,
                Continue(next) => {
                    workflow = self.workflows.get(&next).unwrap_or_else(|| {
                        panic!("workflow {} found, check parsing/input data", &next)
                    });
                }
            }
        }
    }
}

pub fn solve(input: &str) -> String {
    let (input, (plant, parts)) = parse_input(input).unwrap();

    assert!(input.is_empty());

    let result = parts
        .into_iter()
        .filter(|part| plant.process(part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[rstest]
    #[case("x>2440:R", Rule { field: X, op: cmp::Ordering::Greater, value: 2440, success: Rejected })]
    #[case("m<2655:A", Rule { field: M, op: cmp::Ordering::Less, value: 2655, success: Accepted })]
    fn parse_rule_test(#[case] input: &str, #[case] output: Rule) {
        let (input, parsed) = parse_rule(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(parsed, output);
    }

    #[rstest]
    #[case("rfg{s<537:gd,x>2440:R,A}", ("rfg", Workflow {
        rules: vec![
            Rule { field: S, op: cmp::Ordering::Less, value: 537, success: Continue("gd".to_string()) },
            Rule { field: X, op: cmp::Ordering::Greater, value: 2440, success: Rejected }],
            fall_back: Accepted
        }))]
    #[case("qqz{s>2770:qs,m<1801:hdj,R}", ("qqz", Workflow {
        rules: vec![
            Rule { field: S, op: cmp::Ordering::Greater, value: 2770, success: Continue("qs".to_string()) },
            Rule { field: M, op: cmp::Ordering::Less, value: 1801, success: Continue("hdj".to_string())}],
            fall_back: Rejected
        }))]
    fn parse_workflow_test(#[case] input: &str, #[case] output: (&str, Workflow)) {
        let (input, parsed) = parse_workflow(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(parsed.0, output.0);
        assert_eq!(parsed.1, output.1);
    }

    #[rstest]
    #[case("{x=787,m=2655,a=1222,s=2876}", Part { x: 787, m: 2655, a: 1222, s: 2876 })]
    #[case("{x=2036,a=79,s=2244,m=264}", Part { x: 2036, m: 264, a: 79, s: 2244 })]
    fn parse_part_test(#[case] input: &str, #[case] output: Part) {
        let (input, parsed) = parse_part(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(parsed, output);
    }

    #[rstest]
    #[case("rfg{s<537:gd,x>2440:R,A}\nqqz{s>2770:qs,m<1801:hdj,R}\n\n{x=2461,m=1339,a=466,s=291}", (ProcessingPlant {
        workflows: vec![
            ("rfg".to_string(), Workflow {
                rules: vec![
                    Rule { field: S, op: cmp::Ordering::Less, value: 537, success: Continue("gd".to_string()) },
                    Rule { field: X, op: cmp::Ordering::Greater, value: 2440, success: Rejected }],
                    fall_back: Accepted
                }),
            ("qqz".to_string(), Workflow {
                rules: vec![
                    Rule { field: S, op: cmp::Ordering::Greater, value: 2770, success: Continue("qs".to_string()) },
                    Rule { field: M, op: cmp::Ordering::Less, value: 1801, success: Continue("hdj".to_string()) }],
                    fall_back: Rejected
                })
            ].into_iter().collect()
        }, vec![
            Part { x: 2461, m: 1339, a: 466, s: 291 }
        ]))]
    #[case("rfg{s<537:gd,x>2440:R,A}\nqqz{s>2770:qs,m<1801:hdj,R}\n\n{x=787,m=2655,a=1222,s=2876}", (ProcessingPlant {
        workflows: vec![
            ("rfg".to_string(), Workflow {
                rules: vec![
                    Rule { field: S, op: cmp::Ordering::Less, value: 537, success: Continue("gd".to_string()) },
                    Rule { field: X, op: cmp::Ordering::Greater, value: 2440, success: Rejected }],
                    fall_back: Accepted
                }),
            ("qqz".to_string(), Workflow {
                rules: vec![
                    Rule { field: S, op: cmp::Ordering::Greater, value: 2770, success: Continue("qs".to_string()) },
                    Rule { field: M, op: cmp::Ordering::Less, value: 1801, success: Continue("hdj".to_string()) }],
                    fall_back: Rejected
                })
            ].into_iter().collect()
        }, vec![
            Part { x: 787, m: 2655, a: 1222, s: 2876 }
        ]))]
    fn parse_input_test(#[case] input: &str, #[case] output: (ProcessingPlant, Vec<Part>)) {
        let (input, parsed) = parse_input(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(parsed.0, output.0);
        assert_eq!(parsed.1, output.1);
    }

    #[test]
    fn solve_sample() {
        let result = solve(SAMPLE);
        assert_eq!(result, "19114");
    }
}
