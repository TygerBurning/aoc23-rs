use std::{collections::HashMap, ops::Index};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct FlipFlop {
    active: bool,
    downstream: Vec<String>,
    just_emitted: Pulse,
}

#[derive(Clone, Debug)]
struct Conjunction {
    downstream: Vec<String>,
    upstream: Vec<String>,
    just_emitted: Pulse,
}

#[derive(Clone, Debug)]
struct Broadcaster {
    downstream: Vec<String>,
}

fn solve_a(input: &str) -> usize {
    let mut flip_flops = HashMap::new();
    let mut conjunctions = HashMap::new();
    let mut broadcaster = Broadcaster { downstream: vec![] };
    for module in input.lines() {
        if module.chars().next().unwrap() == '%' {
            let module_parts = module[1..].split(" -> ").collect::<Vec<&str>>();
            let module_name = module_parts[0];
            let downstream = module_parts[1]
                .split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            flip_flops.insert(
                module_name,
                FlipFlop {
                    active: false,
                    downstream: downstream,
                    just_emitted: Pulse::Low,
                },
            );
        } else if module.chars().next().unwrap() == '&' {
            let module_parts = module[1..].split(" -> ").collect::<Vec<&str>>();
            let module_name = module_parts[0];
            let downstream = module_parts[1]
                .split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            conjunctions.insert(
                module_name,
                Conjunction {
                    downstream: downstream,
                    upstream: vec![],
                    just_emitted: Pulse::Low,
                },
            );
        } else {
            let module_parts = module[0..].split(" -> ").collect::<Vec<&str>>();
            let downstream = module_parts[1]
                .split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            broadcaster = Broadcaster {
                downstream: downstream,
            };
        }
    }
    for (name, module) in &flip_flops {
        for (conjunction_name, conjunction_module) in &mut conjunctions {
            if module.downstream.contains(&conjunction_name.to_string()) {
                conjunction_module.upstream.push(name.to_string());
            }
        }
    }
    for (name, module) in &conjunctions.clone() {
        for (conjunction_name, conjunction_module) in &mut conjunctions {
            if module.downstream.contains(&conjunction_name.to_string()) {
                conjunction_module.upstream.push(name.to_string());
            }
        }
    }

    let mut low_pulses_count = 0;
    let mut high_pulses_count = 0;

    for i in 0..10000000 {
        // println!("================= {} ================", i);
        let mut current_pulses: Vec<(String, Pulse)> =
            vec![("broadcaster".to_string(), Pulse::Low)];
        let mut round = 0;
        while !current_pulses.is_empty() {
            let mut new_pulses = vec![];
            // println!("Examining current_pulses: {:?}", current_pulses);
            for (current_pulse, pulse_type) in &current_pulses {
                // We go through all the active pulses, and determine what other pulses will fire.
                // Count what we're seeing.
                match pulse_type {
                    Pulse::High => high_pulses_count += 1,
                    Pulse::Low => low_pulses_count += 1,
                }

                if pulse_type == &Pulse::Low && current_pulse == "fv" {
                    println!("Sent Low to 'fv' on: button press {} on round: {}", i, round);
                }
                if pulse_type == &Pulse::Low && current_pulse == "kk" {
                    println!("Sent Low to 'kk' on: button press {} on round: {}", i, round);
                }
                if pulse_type == &Pulse::Low && current_pulse == "vt" {
                    println!("Sent Low to 'vt' on: button press {} on round: {}", i, round);
                }
                if pulse_type == &Pulse::Low && current_pulse == "xr" {
                    println!("Sent Low to 'xr' on: button press {} on round: {}", i, round);
                }

                // Special case for the first go...
                if current_pulse == "broadcaster" {
                    for module_name in &broadcaster.downstream {
                        new_pulses.push((module_name.clone(), Pulse::Low));
                    }
                }
                // Only do something with flips flops if the Pulse is low.
                else if pulse_type == &Pulse::Low
                    && flip_flops.contains_key(current_pulse.as_str())
                {
                    let ff = flip_flops.get_mut(current_pulse.as_str()).unwrap();
                    ff.active = !ff.active;
                    let next_pulse = if ff.active { Pulse::High } else { Pulse::Low };
                    ff.just_emitted = next_pulse.clone();
                    for module_name in &ff.downstream {
                        new_pulses.push((module_name.clone(), next_pulse.clone()));
                    }
                } else if conjunctions.contains_key(current_pulse.as_str()) {
                    let c_clone  = conjunctions.clone(); // Bleugh.
                    let c = conjunctions.get_mut(current_pulse.as_str()).unwrap();
                    let mut all_high = true;
                    for ups in &c.upstream {
                        all_high = all_high
                            && (!flip_flops.contains_key(ups.as_str())
                                || flip_flops.get(ups.as_str()).unwrap().just_emitted
                                    == Pulse::High)
                            && (!c_clone.contains_key(ups.as_str())
                                || c_clone.get(ups.as_str()).unwrap().just_emitted
                                    == Pulse::High)
                    }
                    for module_name in &c.downstream {
                        if all_high {
                            c.just_emitted = Pulse::Low;
                            new_pulses.push((module_name.clone(), Pulse::Low))
                        } else {
                            c.just_emitted = Pulse::High;
                            new_pulses.push((module_name.clone(), Pulse::High))
                        }
                    }
                }
            }
            current_pulses = new_pulses;
            round += 1;
        }
    }

    println!("Low: {}, High: {}", low_pulses_count, high_pulses_count);

    low_pulses_count * high_pulses_count
}

fn solve_b(input: &str) -> usize {
    0
}

pub fn day20() {
    let input = include_str!("../inputs/day20.txt");

    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_1() {
    let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
    assert_eq!(solve_a(input), 32000000)
}

#[test]
fn example_1b() {
    let input = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;
    assert_eq!(solve_a(input), 11687500);
    assert!(false);
}
