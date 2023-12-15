fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
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

// Is this row/column between the two given coords?
fn between(c1: &usize, c2: &usize, v: &usize) -> bool {
    if c1 < c2 {
        c1 < v && v < c2
    } else {
        c2 < v && v < c1
    }
}

fn distance_between_galaxies(
    g1: &(usize, usize),
    g2: &(usize, usize),
    expanding_rows: &Vec<usize>,
    expanding_columns: &Vec<usize>,
    multiplier: &usize,
) -> usize {
    let (x1, y1) = g1;
    let (x2, y2) = g2;

    let added_columns = expanding_columns
        .iter()
        .filter(|i| between(x1, x2, *i))
        .count();
    let x_dist = if x1 > x2 { x1 - x2 } else { x2 - x1 };

    let added_rows = expanding_rows
        .iter()
        .filter(|i| between(y1, y2, *i))
        .count();
    let y_dist = if y1 > y2 { y1 - y2 } else { y2 - y1 };

    let total_dist = x_dist + y_dist + (added_columns + added_rows) * (multiplier - 1);

    // println!(
    //     "Distance between {:?} and {:?} is: {}",
    //     g1,
    //     g2,
    //     total_dist
    // );

    total_dist
}

fn solve(s: &str, multiplier: usize) -> usize {
    let grid: Vec<Vec<char>> = s.lines().map(|s| s.chars().collect()).collect();

    let expanding_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, s)| !s.contains(&'#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let expanding_columns = transpose(&grid)
        .iter()
        .enumerate()
        .filter(|(_, s)| !s.contains(&'#'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    println!(
        "Expanding rows are: {:?} and expanding columns are {:?}",
        expanding_rows, expanding_columns
    );

    let mut galaxy_locations = vec![];
    for (j, row) in grid.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxy_locations.push((i, j));
            }
        }
    }

    let mut total = 0;
    for galaxy_1 in &galaxy_locations {
        for galaxy_2 in &galaxy_locations {
            if galaxy_1 == galaxy_2 {
                continue;
            }

            total += distance_between_galaxies(
                galaxy_1,
                galaxy_2,
                &expanding_rows,
                &expanding_columns,
                &multiplier,
            );
        }
    }
    total / 2
}

fn solve_a(s: &str) -> usize {
    // 1 space has gone to 2 spaces
    solve(s, 2)
}

fn solve_b(s: &str) -> usize {
    // 1 space has gone to 1000000
    solve(s, 1000000)
}

pub fn day11() {
    let input = include_str!("../inputs/day11.txt");
    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_a() {
    assert_eq!(
        solve_a(
            r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
        ),
        374
    );
}

#[test]
fn example_b() {
    assert_eq!(
        solve(
            r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#,
            10
        ),
        1030
    );
}