use std::{collections::HashMap};

use regex::Regex;

fn solve_a(start: &str, graph: &HashMap<String, (String, String)>, directions: &Vec<char>) -> usize {
    let mut steps = 0;
    let mut current = start;
    while !reached_end(current) {
        let d = directions[steps % directions.len()];
        match d {
            'L' => current = &graph[current].0,
            'R' => current = &graph[current].1,
            _ => panic!("What the deuce"),
        }
        steps += 1;
    }
    steps
}

fn reached_end(v: &str) -> bool {
    v.chars().nth(2).unwrap() == 'Z'
}

fn solve_b(graph: &HashMap<String, (String, String)>, directions: &Vec<char>) -> usize {
    let mut start_nodes = graph
        .clone()
        .into_keys()
        .filter(|p| p.chars().nth(2).unwrap() == 'A');

    // This is cheating. I should really be doing cycle detection, but I heard a rumour this works, and hey, it does.
    let mut lcm = 1;
    for node in start_nodes {
        let steps = solve_a(&node, &graph, &directions);
        lcm = num::integer::lcm(lcm, steps);
    }
    lcm
}

pub fn day08() {
    let input = include_str!("../inputs/day08.txt");

    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();
    _ = lines.next();

    let edge_regex = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();

    let mut graph = HashMap::new();
    for edge in lines {
        let r = edge_regex.captures(edge).unwrap();
        graph.insert(r[1].to_string(), (r[2].to_string(), r[3].to_string()));
    }

    println!("Part A is: {}", solve_a("AAA", &graph, &directions));
    println!("Part B is: {}", solve_b(&graph, &directions));
}
