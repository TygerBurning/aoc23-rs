use std::collections::HashMap;

use num::complex::ComplexFloat;
use regex::Regex;

enum PartType {
    X,
    M,
    A,
    S,
}

enum Comparison {
    GT,
    LT,
    Continue,
}

fn comparison_from_str(comp: &str) -> Comparison {
    match comp {
        "<" => Comparison::LT,
        ">" => Comparison::GT,
        "=" => Comparison::Continue,
        _ => panic!("Jeepers"),
    }
}

fn part_type_from_str(part_str: &str) -> PartType {
    match part_str {
        "x" => PartType::X,
        "m" => PartType::M,
        "a" => PartType::A,
        "s" => PartType::S,
        _ => panic!("Yikes"),
    }
}

#[derive(Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn create_greater_than(pt: PartType, val: u32, s: String) -> Box<dyn Fn(&Part) -> Option<String>> {
    Box::new(move |part: &Part| match pt {
        PartType::X => {
            if part.x > val {
                Some(s.to_owned())
            } else {
                None
            }
        }
        PartType::M => {
            if part.m > val {
                Some(s.to_owned())
            } else {
                None
            }
        }
        PartType::A => {
            if part.a > val {
                Some(s.to_owned())
            } else {
                None
            }
        }
        PartType::S => {
            if part.s > val {
                Some(s.to_owned())
            } else {
                None
            }
        }
    })
}

fn create_less_than(pt: PartType, val: u32, s: String) -> Box<dyn Fn(&Part) -> Option<String>> {
    Box::new(move |part: &Part| match pt {
        PartType::X => {
            if part.x < val {
                Some(s.to_owned())
            } else {
                None
            }
        }
        PartType::M => {
            if part.m < val {
                Some(s.to_owned())
            } else {
                None
            }
        }
        PartType::A => {
            if part.a < val {
                Some(s.to_owned())
            } else {
                None
            }
        }
        PartType::S => {
            if part.s < val {
                Some(s.to_owned())
            } else {
                None
            }
        }
    })
}

fn return_str(s: String) -> Box<dyn Fn(&Part) -> Option<String>> {
    Box::new(move |_| Some(s.to_owned()))
}

fn create_workflow_parts(
    input: &str,
) -> (
    HashMap<String, Vec<Box<dyn Fn(&Part) -> Option<String>>>>,
    Vec<Part>,
) {
    let workflows_parts = input.split("\n\n").collect::<Vec<&str>>();

    let mut hm = HashMap::new();
    let workflow_regex = Regex::new(r"(.*)\{(.*)\}").unwrap();
    let rules_regex = Regex::new(r"(.*)([<>])(.*):(.*)").unwrap();
    for workflow_str in workflows_parts[0].lines() {
        let r = workflow_regex.captures(workflow_str).unwrap();
        let name = r[1].to_string();
        let mut rules = vec![];
        for rule_str in r[2].split(",") {
            let rules_c_opt = rules_regex.captures(rule_str);
            if rules_c_opt.is_some() {
                let rules_c = rules_c_opt.unwrap();
                if rules_c[2].to_string() == "<" {
                    rules.push(create_less_than(
                        part_type_from_str(&rules_c[1]),
                        rules_c[3].parse().unwrap(),
                        rules_c[4].to_string(),
                    ));
                } else if rules_c[2].to_string() == ">" {
                    rules.push(create_greater_than(
                        part_type_from_str(&rules_c[1]),
                        rules_c[3].parse().unwrap(),
                        rules_c[4].to_string(),
                    ));
                } else {
                    panic!("Uh oh");
                }
            } else {
                rules.push(Box::new(return_str(rule_str.to_string())));
            }
        }
        hm.insert(name, rules);
    }

    let parts_regex = Regex::new(r"x=([0-9]*),m=([0-9]*),a=([0-9]*),s=([0-9]*)").unwrap();
    let parts = workflows_parts[1]
        .lines()
        .map(|part_str| {
            let r = parts_regex.captures(part_str).unwrap();
            Part {
                x: r[1].parse().unwrap(),
                m: r[2].parse().unwrap(),
                a: r[3].parse().unwrap(),
                s: r[4].parse().unwrap(),
            }
        })
        .collect::<Vec<Part>>();

    (hm, parts)
}

fn solve_a(input: &str) -> u32 {
    let (workflows, parts) = create_workflow_parts(input);

    let mut accepted_parts = vec![];
    for part in parts {
        let mut workflow = "in".to_string();
        let mut undecided = true;
        while (undecided) {
            let rules = workflows.get(&workflow).unwrap();
            // println!("Rules are: {:?}", rules);
            for rule in rules {
                let rule_result = rule(&part);
                match rule_result {
                    Some(x) => {
                        if x == "A" {
                            accepted_parts.push(part.clone());
                            undecided = false;
                            break;
                        }
                        if x == "R" {
                            // Rejected - move onto the next part.
                            undecided = false;
                            break;
                        }
                        // Jump to different workflow.
                        workflow = x.clone();
                        break;
                    }
                    None => {
                        // Try the next rule to see if it matches.
                    }
                }
            }
        }
    }

    let mut total = 0;
    for part in accepted_parts {
        total += part.x;
        total += part.m;
        total += part.a;
        total += part.s
    }
    total
}


pub fn day19() {
    let input = include_str!("../inputs/day19.txt");

    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));

    // "rfg{s<537:gd,x>2440:R,A}"
}

#[test]
fn example_1() {
    let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    assert_eq!(solve_a(input), 19114)
}


#[test]
fn example_2a() {
    let input = r#"in{a<1000:A,R}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    assert_eq!(solve_b(input), 63936000000000)
}

#[test]
fn example_2b() {
    let input = r#"in{a<1001:A,m>2000:R,A}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    assert_eq!(solve_b(input), 64000000000000 + 96000000000000)
}


#[test]
fn example_2() {
    let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    assert_eq!(solve_b(input), 167409079868000)
}


/******************************************************
 * Don't look down here - there be some scary things...
 *****************************************************/



fn search_for_an_a(
    x_min: u64,
    x_max: u64,
    m_min: u64,
    m_max: u64,
    a_min: u64,
    a_max: u64,
    s_min: u64,
    s_max: u64,
    workflow: &str,
    workflow_rules: &HashMap<String, Vec<(PartType, Comparison, u64, String)>>,
) -> u64 {
    if x_min > x_max || m_min > m_max || a_min > a_max || s_min > s_max {
        return 0;
    }
    if workflow == "A" {
        return (x_max - x_min + 1) * (m_max - m_min + 1) * (a_max - a_min + 1) * (s_max - s_min + 1);
    }
    if workflow == "R" {
        return 0;
    }

    let rules = workflow_rules.get(workflow).unwrap();
    let mut total = 0;
    let (
        mut new_x_min,
        mut new_x_max,
        mut new_m_min,
        mut new_m_max,
        mut new_a_min,
        mut new_a_max,
        mut new_s_min,
        mut new_s_max,
    ) = (x_min, x_max, m_min, m_max, a_min, a_max, s_min, s_max);
    for (pt, comparison, val, next_step) in rules {
        match comparison {
            Comparison::Continue => {
                total += search_for_an_a(
                    new_x_min,
                    new_x_max,
                    new_m_min,
                    new_m_max,
                    new_a_min,
                    new_a_max,
                    new_s_min,
                    new_s_max,
                    next_step,
                    &workflow_rules,
                );
            },
            Comparison::GT => match pt {
                PartType::X => {
                    // Success is when X > val
                    total += search_for_an_a(
                        *val + 1,
                        new_x_max,
                        new_m_min,
                        new_m_max,
                        new_a_min,
                        new_a_max,
                        new_s_min,
                        new_s_max,
                        next_step,
                        &workflow_rules,
                    );
                    new_x_max = *val
                }
                PartType::M => {
                    total += search_for_an_a(
                        new_x_min,
                        new_x_max,
                        *val + 1,
                        new_m_max,
                        new_a_min,
                        new_a_max,
                        new_s_min,
                        new_s_max,
                        next_step,
                        &workflow_rules,
                    );
                    new_m_max = *val;
                }
                PartType::A => {
                    total += search_for_an_a(
                        new_x_min,
                        new_x_max,
                        new_m_min,
                        new_m_max,
                        *val + 1,
                        new_a_max,
                        new_s_min,
                        new_s_max,
                        next_step,
                        &workflow_rules,
                    );
                    new_a_max = *val;
                }
                PartType::S => {
                    total += search_for_an_a(
                        new_x_min,
                        new_x_max,
                        new_m_min,
                        new_m_max,
                        new_a_min,
                        new_a_max,
                        *val + 1,
                        new_s_max,
                        next_step,
                        &workflow_rules,
                    );
                    new_s_max = *val;
                }
            },
            Comparison::LT => match pt {
                PartType::X => {
                    // Success is when X < val
                    total += search_for_an_a(
                        new_x_min,
                        std::cmp::min(*val - 1, 4000),
                        new_m_min,
                        new_m_max,
                        new_a_min,
                        new_a_max,
                        new_s_min,
                        new_s_max,
                        next_step,
                        &workflow_rules,
                    );
                    new_x_min = *val
                }
                PartType::M => {
                    total += search_for_an_a(
                        new_x_min,
                        new_x_max,
                        new_m_min,
                        *val - 1,
                        new_a_min,
                        new_a_max,
                        new_s_min,
                        new_s_max,
                        next_step,
                        &workflow_rules,
                    );
                    new_m_min = *val;
                }
                PartType::A => {
                    total += search_for_an_a(
                        new_x_min,
                        new_x_max,
                        new_m_min,
                        new_m_max,
                        new_a_min,
                        *val - 1,
                        new_s_min,
                        new_s_max,
                        next_step,
                        &workflow_rules,
                    );
                    new_a_min = *val;
                }
                PartType::S => {
                    total += search_for_an_a(
                        new_x_min,
                        new_x_max,
                        new_m_min,
                        new_m_max,
                        new_a_min,
                        new_a_max,
                        new_s_min,
                        *val - 1,
                        next_step,
                        &workflow_rules,
                    );
                    new_s_min = *val;
                }
            },
        }
    }

    // For rule in rules:
    // Clamp relevant letter for true:
    // - If "A", calculate how many possibilities and return
    // - If "R", return 0
    // - Otherwise - recurse down that workflow
    // Clamp relevant letter for false - continue rules processing.
    total
}

fn solve_b(input: &str) -> u64 {
    let workflows_parts = input.split("\n\n").collect::<Vec<&str>>();

    let mut hm = HashMap::new();
    let workflow_regex = Regex::new(r"(.*)\{(.*)\}").unwrap();
    let rules_regex = Regex::new(r"(.*)([<>])(.*):(.*)").unwrap();
    for workflow_str in workflows_parts[0].lines() {
        let r = workflow_regex.captures(workflow_str).unwrap();
        let name = r[1].to_string();
        let mut rules = vec![];
        for rule_str in r[2].split(",") {
            let rules_c_opt = rules_regex.captures(rule_str);
            if rules_c_opt.is_some() {
                let rules_c = rules_c_opt.unwrap();
                    rules.push((part_type_from_str(&rules_c[1]),
                        comparison_from_str(&rules_c[2]),
                        rules_c[3].parse().unwrap(),
                        rules_c[4].to_string(),
                    ));
            } else {
                // Set Comparison to be as passthrough.
                rules.push((PartType::X, Comparison::Continue, 5000, rule_str.to_string()));
            }
        }
        hm.insert(name, rules);
    }

    search_for_an_a(1, 4000, 1, 4000, 1, 4000, 1, 4000, "in", &hm)
}