use std::collections::HashMap;

fn hash(code: &str) -> usize {
    let mut res = 0;
    for elem in code.chars() {
        res += elem as usize;
        res *= 17;
        res %= 256;
    }
    res
}

fn solve_a(s: &str) -> usize {
    s.split(",").map(|s| hash(s)).sum()
}

// This is a horrible amount of nasty cloning code... Am I going to improve it? eh.
fn solve_b(s: &str) -> usize {
    let mut hm: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    for instruction in s.split(",") {
        if instruction.contains('-') {
            let label = instruction.split('-').collect::<Vec<&str>>()[0];
            let box_id = hash(label);
            let v = vec![];
            let box_values = hm.get(&box_id).unwrap_or(&v);
            let new_box_values = box_values
                .iter()
                .filter(|(s, _)| s != &label)
                .map(|s| s.to_owned())
                .collect::<Vec<(&str, usize)>>();
            hm.insert(box_id, new_box_values);
        }
        if instruction.contains('=') {
            let label = instruction.split('=').collect::<Vec<&str>>()[0];
            let box_id = hash(label);
            let focal_length = instruction.split('=').collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let v = vec![];
            let box_values = hm.get(&box_id).unwrap_or(&v).clone();

            if (box_values.iter().find(|(s, _)| s == &label)).is_some() {
                let new_box_values = box_values
                    .iter()
                    .map(|(s, f)| {
                        if s == &label {
                            (label, focal_length)
                        } else {
                            (s.to_owned(), f.to_owned())
                        }
                    })
                    .map(|s| s.to_owned())
                    .collect::<Vec<(&str, usize)>>();
                hm.insert(box_id, new_box_values);
            }
            else {
                let mut new_box_values = box_values.clone();
                new_box_values.push((label, focal_length));
                hm.insert(box_id, new_box_values);
            }
        }
    }

    let mut total = 0;
    for (box_id, lenss) in hm {
        for (slot_id, (_, focal_length)) in lenss.iter().enumerate() {
            total += (box_id + 1) * (slot_id + 1) * focal_length;
        }
    }
    total
}

pub fn day15() {
    let input = include_str!("../inputs/day15.txt");
    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_a() {
    assert_eq!(
        solve_a("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        1320
    )
}

#[test]
fn example_b_1() {
    assert_eq!(hash("rn"), 0);
    assert_eq!(hash("cm"), 0);
    assert_eq!(hash("qp"), 1);
}

#[test]
fn example_b() {
    assert_eq!(
        solve_b("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        145
    );
}