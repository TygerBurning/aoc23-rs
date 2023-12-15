fn extrapolate_next_elem(v: Vec<isize>) -> isize {
    if v.iter().all(|i| i == &0) {
        return 0;
    }

    let mut diff_v = vec![];
    for sli in v.windows(2) {
        diff_v.push(sli[1] - sli[0]);
    }
    v[v.len() - 1] + extrapolate_next_elem(diff_v)
}

fn extrapolate_prev_elem(v: Vec<isize>) -> isize {
    println!("Extrapolating: {:?}", v);
    if v.iter().all(|i| i == &0) {
        return 0;
    }

    let mut diff_v = vec![];
    for sli in v.windows(2) {
        diff_v.push(sli[1] - sli[0]);
    }

    v[0] - extrapolate_prev_elem(diff_v)
}

fn solve_a(s: &str) -> isize {
    s.lines()
        .map(|x| {
            x.split(' ')
                .map(|i| i.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|x| extrapolate_next_elem(x))
        .sum()
}

fn solve_b(s: &str) -> isize {
    s.lines()
    .map(|x| {
        x.split(' ')
            .map(|i| i.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
    })
    .map(|x| extrapolate_prev_elem(x))
    .sum()
}

pub fn day09() {
    let input = include_str!("../inputs/day09.txt");
    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_a() {
    assert_eq!(
        solve_a(
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
        ),
        114
    );
}

#[test]
fn example_b() {
    assert_eq!(
        solve_b(
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
        ),
        2
    );
}