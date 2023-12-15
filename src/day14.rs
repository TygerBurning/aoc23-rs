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

fn solve_a(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let rock_columns = transpose(lines);
    let rock_rows = rock_columns.len();
    let mut total = 0;

    for rock_column in rock_columns {
        // Default the blocked row to one off the end.
        let mut blocked_row = rock_rows + 1;
        for (index, rock) in rock_column.iter().enumerate() {
            match rock {
                '#' => blocked_row = rock_rows - index,
                'O' => {
                    blocked_row -= 1;
                    total += blocked_row;
                }
                _ => {}
            }
        }
    }
    total
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for rock in row {
            print!("{}", rock);
        }
        println!();
    }
    println!();
}

fn move_rocks(rotation: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next_rotation = vec![];

    for rock_row in rotation {
        // We'll count the number of circle rocks until we hit a blocked rock.
        let mut circle_rocks_seen = 0;
        let mut gaps_seen = 0;
        let mut new_row = vec![];
        for (_index, rock) in rock_row.iter().enumerate() {
            match rock {
                'O' => {
                    circle_rocks_seen += 1;
                }
                '.' => {
                    gaps_seen += 1;
                }
                '#' => {
                    // We've hit a blocked rock, so we need to move all the circle rocks
                    // we've seen so far.
                    for _i in 0..circle_rocks_seen {
                        new_row.push('O');
                    }
                    // Then add the spaces
                    for _i in 0..gaps_seen {
                        new_row.push('.');
                    }
                    // Finally add the blocked rock
                    new_row.push('#');
                    circle_rocks_seen = 0;
                    gaps_seen = 0;
                }
                _ => {}
            }
        }
        for _i in 0..circle_rocks_seen {
            new_row.push('O');
        }
        // Then add the spaces
        for _i in 0..gaps_seen {
            new_row.push('.');
        }
        next_rotation.push(new_row);
    }

    next_rotation
}

fn move_north(rotation: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(move_rocks(transpose(rotation)))
}

fn move_west(rotation: Vec<Vec<char>>) -> Vec<Vec<char>> {
    move_rocks(rotation)
}

fn move_east(rotation: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut eastward = rotation;
    eastward.iter_mut().for_each(|v| v.reverse());
    let mut res = move_rocks(eastward);
    res.iter_mut().for_each(|v| v.reverse());
    res
}

fn move_south(rotation: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut southward = transpose(rotation);
    southward.iter_mut().for_each(|v| v.reverse());
    let mut res = move_rocks(southward);
    res.iter_mut().for_each(|v| v.reverse());
    transpose(res)
}

fn rotate(rotation: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next_rotation = rotation;

    next_rotation = move_north(next_rotation);
    next_rotation = move_west(next_rotation);
    next_rotation = move_south(next_rotation);
    next_rotation = move_east(next_rotation);

    next_rotation
}

fn calculate_cost(rotation: &Vec<Vec<char>>) -> usize {
    // print_grid(rotation);

    let mut total = 0;
    let max = rotation[0].len();
    for (cost, row) in rotation.iter().enumerate() {
        for rock in row {
            if rock == &'O' {
                total += max - cost;
            }
        }
    }
    total
}

fn solve_b(input: &str) -> usize {
    let mut previous_rotations = vec![];

    let mut rotation = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    while !previous_rotations.contains(&rotation) {
        previous_rotations.push(rotation.clone());
        rotation = rotate(rotation);
    }

    let total_cycles = 1000000000;
    let first_match = previous_rotations
        .iter()
        .position(|r| r == &rotation)
        .unwrap();
    let cycle_length = previous_rotations.len() - first_match;
    let last_match = ((total_cycles - first_match) / cycle_length) * cycle_length + first_match;
    calculate_cost(&previous_rotations[total_cycles - last_match + first_match])
}

pub fn day14() {
    let input = include_str!("../inputs/day14.txt");
    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_a() {
    assert_eq!(
        solve_a(
            r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#,
        ),
        136
    );
}

#[test]
fn example_a_2() {
    let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    let rotation = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let north_rotation = move_north(rotation);
    assert_eq!(calculate_cost(&north_rotation), 136);
}

#[test]
fn example_b() {
    assert_eq!(
        solve_b(
            r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#,
        ),
        64
    );
}