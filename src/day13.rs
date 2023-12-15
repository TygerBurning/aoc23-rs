use std::iter::zip;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut v2 = vec![];
    for i in 0..v[0].len() {
        let pos_v = v
            .iter()
            .map(|inner_v| inner_v[i].clone())
            .collect::<Vec<T>>();
        v2.push(pos_v);
    }
    v2
}

fn find_symmetry(v: &Vec<Vec<char>>) -> Option<usize> {
    for guess_symmetry_line in (0 as isize)..(v.len() - 1) as isize {
        let mut symmetry_line_above = guess_symmetry_line;
        let mut symmetry_line_below = guess_symmetry_line + 1;
        let mut valid = true;
        while symmetry_line_above >= 0 && symmetry_line_below < v.len() as isize {
            // Check all lines on the grid are the same. Can break as soon as we find a mismatch.
            if v[symmetry_line_above as usize] != v[symmetry_line_below as usize] {
                valid = false;
                break;
            }
            symmetry_line_above -= 1;
            symmetry_line_below += 1;
        }
        if valid {
            return Some((guess_symmetry_line + 1) as usize);
        }
    }
    None
}

fn vec_distance(v1: &Vec<char>, v2: &Vec<char>) -> usize {
    zip(v1, v2).map(|(v1, v2)| if v1 == v2 { 0 } else { 1 }).sum()
}

fn find_symmetry_b(v: &Vec<Vec<char>>) -> Option<usize> {
    for guess_symmetry_line in (0 as isize)..(v.len() - 1) as isize {
        let mut symmetry_line_above = guess_symmetry_line;
        let mut symmetry_line_below = guess_symmetry_line + 1;
        let mut distance = 0;
        while symmetry_line_above >= 0 && symmetry_line_below < v.len() as isize {
            // Check all lines on the grid are the same. Can break as soon as we find a mismatch.
            distance += vec_distance(&v[symmetry_line_above as usize], &v[symmetry_line_below as usize]);
            if distance > 1 {
                break;
            }
            symmetry_line_above -= 1;
            symmetry_line_below += 1;
        }
        if distance == 1 {
            return Some((guess_symmetry_line + 1) as usize);
        }
    }
    None
}


fn find_any_symmetry(s: &str) -> usize {
    let grid = s
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let res = find_symmetry(&grid);
    if res.is_some() {
        // Horiztonal symmetry is worth 100* points.
        return res.unwrap() * 100;
    }
    else {
        return find_symmetry(&transpose(grid)).unwrap()
    }
}

fn find_any_symmetry_b(s: &str) -> usize {
    let grid = s
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("Considering the following grid:");
    println!("{}", s);

    let res = find_symmetry_b(&grid);
    if res.is_some() {
        // Horiztonal symmetry is worth 100* points.
        return res.unwrap() * 100;
    }
    else {
        return find_symmetry_b(&transpose(grid)).unwrap()
    }
}

fn solve_a(input: &str) -> usize {
    let grids = input.split("\n\n").collect::<Vec<&str>>();

    grids.iter().map(|s| find_any_symmetry(s)).sum()
}

fn solve_b(input: &str) -> usize {
    let grids = input.split("\n\n").collect::<Vec<&str>>();

    grids.iter().map(|s| find_any_symmetry_b(s)).sum()
}

pub fn day13() {
    let input = include_str!("../inputs/day13.txt");
    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));

}

#[test]
fn example_a() {
    let input = assert_eq!(
        solve_a(
            r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
        ),
        405
    );
}

#[test]
fn example_b() {
    let input = assert_eq!(
        solve_b(
            r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
        ),
        400
    );
}