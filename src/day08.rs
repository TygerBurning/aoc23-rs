use std::{collections::HashMap, ops::Index};

use regex::Regex;

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

    {
    // let mut node = "AAA".to_string();
    // let mut steps = 0;
    // while node != "ZZZ" {
    //     let d = directions[steps % directions.len()];
    //     match d {
    //         'L' => node = graph[&node].0.clone(),
    //         'R' => node = graph[&node].1.clone(),
    //         _ => panic!("What the deuce"),
    //     }
    //     steps += 1;
    // }

    // println!("Part A is: {}", steps);
    }

    let mut nodes = graph
        .clone()
        .into_keys()
        .filter(|p| p.chars().nth(2).unwrap() == 'A')
        .collect::<Vec<String>>();
    let mut steps = 0;
    while !reached_end(&nodes) {
        let d = directions[steps % directions.len()];
        let mut new_nodes = vec![];
        for node in nodes {
            match d {
                'L' => {
                    new_nodes.push(graph[&node].0.clone());
                    // println!(
                    //     "Node: {} went left and is now: {}",
                    //     node,
                    //     graph[&node].0.clone()
                    // )
                }
                'R' => {
                    new_nodes.push(graph[&node].1.clone());
                    // println!(
                    //     "Node: {} went right and is now: {}",
                    //     node,
                    //     graph[&node].0.clone()
                    // )
                }
                _ => panic!("What the deuce"),
            }
        }
        nodes = new_nodes;
        steps += 1;
    }
    println!("Part B is: {}", steps);
}

fn reached_end(v: &Vec<String>) -> bool {
    v.iter().all(|p| p.chars().nth(2).unwrap() == 'Z')
}
