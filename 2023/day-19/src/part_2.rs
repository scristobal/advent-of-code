/*
 * Advent of code solutions
 * https://www.github.com/scristobal/advent-of-code
 * Licensed under MIT, 2023 Samuel Cristobal
 */

use nom::character::complete;
use nom::multi::{separated_list0, separated_list1};
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

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
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

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Rule {
    field: Field,
    op: cmp::Ordering,
    value: u128,
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
    x: u128,
    m: u128,
    a: u128,
    s: u128,
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
        value: value as u128,
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
                separated_list0(tag(","), parse_rule),
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

// eg. "gd{a>3333:R,R}\nhdj{m>838:A,pv}"
fn parse_input(input: &str) -> IResult<&str, ProcessingPlant> {
    let (input, workflows) = separated_list1(tag("\n"), parse_workflow)(input)?;

    let plant = ProcessingPlant {
        workflows: workflows.into_iter().collect(),
    };

    Ok((input, plant))
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Range {
    min: u128,
    max: u128,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Restriction {
    range_x: Range,
    range_m: Range,
    range_a: Range,
    range_s: Range,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct State {
    classification: Classification,
    workflow_history: Vec<String>,
    restriction: Restriction,
}

pub fn solve(input: &str) -> String {
    let (_, plant) = parse_input(input).unwrap();

    let mut queue = vec![State {
        classification: Continue("in".to_string()),
        workflow_history: vec![],
        restriction: Restriction {
            range_x: Range { min: 1, max: 4000 },
            range_m: Range { min: 1, max: 4000 },
            range_a: Range { min: 1, max: 4000 },
            range_s: Range { min: 1, max: 4000 },
        },
    }];

    let mut accepted: Vec<State> = vec![];
    let mut rejected: Vec<State> = vec![];

    while let Some(state) = queue.pop() {
        match state.classification {
            Accepted => {
                accepted.push(state);
            }
            Rejected => {
                rejected.push(state);
            }
            Continue(workflow_id) => {
                let workflow = plant
                    .workflows
                    .get(&workflow_id)
                    .unwrap_or_else(|| panic!("Failed to find workflow {}", workflow_id));

                let mut restriction = state.restriction.clone();
                let mut negated_restriction = state.restriction.clone();

                for rule in &workflow.rules {
                    match (rule.field, rule.op) {
                        (X, cmp::Ordering::Greater) => {
                            restriction.range_x.min =
                                cmp::max(restriction.range_x.min, rule.value + 1);
                            negated_restriction.range_x.max =
                                cmp::min(negated_restriction.range_x.max, rule.value);
                        }
                        (X, cmp::Ordering::Less) => {
                            restriction.range_x.max =
                                cmp::min(restriction.range_x.max, rule.value - 1);
                            negated_restriction.range_x.min =
                                cmp::max(negated_restriction.range_x.min, rule.value);
                        }
                        (M, cmp::Ordering::Greater) => {
                            restriction.range_m.min =
                                cmp::max(restriction.range_m.min, rule.value + 1);
                            negated_restriction.range_m.max =
                                cmp::min(negated_restriction.range_m.max, rule.value);
                        }
                        (M, cmp::Ordering::Less) => {
                            restriction.range_m.max =
                                cmp::min(restriction.range_m.max, rule.value - 1);
                            negated_restriction.range_m.min =
                                cmp::max(negated_restriction.range_m.min, rule.value);
                        }
                        (A, cmp::Ordering::Greater) => {
                            restriction.range_a.min =
                                cmp::max(restriction.range_a.min, rule.value + 1);
                            negated_restriction.range_a.max =
                                cmp::min(negated_restriction.range_a.max, rule.value);
                        }
                        (A, cmp::Ordering::Less) => {
                            restriction.range_a.max =
                                cmp::min(restriction.range_a.max, rule.value - 1);
                            negated_restriction.range_a.min =
                                cmp::max(negated_restriction.range_a.min, rule.value);
                        }
                        (S, cmp::Ordering::Greater) => {
                            restriction.range_s.min =
                                cmp::max(restriction.range_s.min, rule.value + 1);
                            negated_restriction.range_s.max =
                                cmp::min(negated_restriction.range_s.max, rule.value);
                        }
                        (S, cmp::Ordering::Less) => {
                            restriction.range_s.max =
                                cmp::min(restriction.range_s.max, rule.value - 1);
                            negated_restriction.range_s.min =
                                cmp::max(negated_restriction.range_s.min, rule.value);
                        }
                        _ => panic!("Invalid rule {:?}", rule),
                    }

                    if restriction.range_x.min <= restriction.range_x.max
                        && restriction.range_m.min <= restriction.range_m.max
                        && restriction.range_a.min <= restriction.range_a.max
                        && restriction.range_s.min <= restriction.range_s.max
                    {
                        queue.push(State {
                            classification: rule.success.clone(),
                            restriction: restriction.clone(),
                            workflow_history: state
                                .workflow_history
                                .iter()
                                .cloned()
                                .chain(vec![workflow_id.clone()])
                                .collect(),
                        });
                    }

                    restriction = negated_restriction.clone()
                }

                if negated_restriction.range_x.min <= negated_restriction.range_x.max
                    && negated_restriction.range_m.min <= negated_restriction.range_m.max
                    && negated_restriction.range_a.min <= negated_restriction.range_a.max
                    && negated_restriction.range_s.min <= negated_restriction.range_s.max
                {
                    queue.push(State {
                        classification: workflow.fall_back.clone(),
                        restriction: negated_restriction,
                        workflow_history: state
                            .workflow_history
                            .iter()
                            .cloned()
                            .chain(vec![workflow_id])
                            .collect(),
                    });
                }
            }
        }
    }

    let num_accepted = accepted
        .iter()
        .map(|State { restriction: r, .. }| {
            (1 + r.range_x.max - r.range_x.min)
                * (1 + r.range_m.max - r.range_m.min)
                * (1 + r.range_a.max - r.range_a.min)
                * (1 + r.range_s.max - r.range_s.min)
        })
        .sum::<u128>();

    let num_rejected = rejected
        .iter()
        .map(|State { restriction: r, .. }| {
            (1 + r.range_x.max - r.range_x.min)
                * (1 + r.range_m.max - r.range_m.min)
                * (1 + r.range_a.max - r.range_a.min)
                * (1 + r.range_s.max - r.range_s.min)
        })
        .sum::<u128>();

    assert_eq!(4000 * 4000 * 4000 * 4000, num_accepted + num_rejected);

    num_accepted.to_string()
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
    #[case("rfg{s<537:gd,x>2440:R,A}\nqqz{s>2770:qs,m<1801:hdj,R}\n\n{x=2461,m=1339,a=466,s=291}", ProcessingPlant {
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
         })]
    #[case("rfg{s<537:gd,x>2440:R,A}\nqqz{s>2770:qs,m<1801:hdj,R}\n\n{x=787,m=2655,a=1222,s=2876}", ProcessingPlant {
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
         })]
    fn parse_input_test(#[case] input: &str, #[case] output: ProcessingPlant) {
        let (input, parsed) = parse_input(input).unwrap();
        assert_eq!(input.chars().take(2).collect::<String>(), "\n\n");
        assert_eq!(parsed, output);
    }

    #[rstest]
    #[case("in{s<1001:A,R}", &(4000_u128*4000*4000*1000).to_string())]
    #[case("in{s<1001:R,A}", &(4000_u128*4000*4000*3000).to_string())]
    #[case("in{s<1001:A,A}", &(4000_u128*4000*4000*4000).to_string())]
    #[case("in{s<2001:a,A}\na{s<1001:A,R}", &(4000_u128*4000*4000*3000).to_string())]
    #[case("in{s<2001:A,a}\na{s>3000:A,R}", &(2000_u128*4000*4000*4000 + (1000_u128*4000*4000*4000 )).to_string())]
    #[case("in{s>2000:A,R}", &(4000_u128*4000*4000*2000).to_string())]
    #[case("in{s>2000:a,R}\na{s<3001:A,R}", &(4000_u128*4000*4000*1000).to_string())]
    fn solve_unit_test(#[case] input: &str, #[case] output: &str) {
        let result = solve(input);
        assert_eq!(result, output);
    }

    // #[ignore = "reason"]
    #[test]
    fn solve_sample_test() {
        let result = solve(SAMPLE);
        assert_eq!(result, "167409079868000");
    }
}
