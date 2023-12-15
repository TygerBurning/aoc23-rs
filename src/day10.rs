#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn solve_a(s: &str) -> (usize, Vec<(usize, usize)>) {
    let grid = s
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut start = (0, 0);
    'outer: for (j, row) in grid.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if c == &'S' {
                start = (i, j);
                break 'outer;
            }
        }
    }

    // Spoilers, but we know it's sensible to go up first.
    // Quick check to see where we should start (thankfully the grid allows this)
    let mut current;
    let mut have_come_from;
    if "|7F".contains(grid[start.1 - 1][start.0]) {
        // We can go up.
        current = (start.0, start.1 - 1);
        have_come_from = Direction::South;
    } else if "|JL".contains(grid[start.1 + 1][start.0]) {
        // We can go down.
        current = (start.0, start.1 + 1);
        have_come_from = Direction::North;
    } else if "-LF".contains(grid[start.1 - 1][start.0]) {
        // We can go left.
        current = (start.0 - 1, start.1);
        have_come_from = Direction::East;
    } else {
        // We can go right - hopefully!
        current = (start.0 + 1, start.1);
        have_come_from = Direction::West;
    }

    let mut path_length = 0;
    let mut path = vec![];
    while current != start {
        path_length += 1;
        path.push(current.clone());
        let pipe = grid[current.1][current.0];
        match pipe {
            '7' => {
                if have_come_from == Direction::South {
                    current = (current.0 - 1, current.1);
                    have_come_from = Direction::East;
                } else {
                    current = (current.0, current.1 + 1);
                    have_come_from = Direction::North;
                }
            }
            'J' => {
                if have_come_from == Direction::West {
                    current = (current.0, current.1 - 1);
                    have_come_from = Direction::South;
                } else {
                    current = (current.0 - 1, current.1);
                    have_come_from = Direction::East;
                }
            }
            'F' => {
                if have_come_from == Direction::South {
                    current = (current.0 + 1, current.1);
                    have_come_from = Direction::West;
                } else {
                    current = (current.0, current.1 + 1);
                    have_come_from = Direction::North;
                }
            }
            'L' => {
                if have_come_from == Direction::East {
                    current = (current.0, current.1 - 1);
                    have_come_from = Direction::South;
                } else {
                    current = (current.0 + 1, current.1);
                    have_come_from = Direction::West;
                }
            }
            '|' => {
                if have_come_from == Direction::North {
                    current = (current.0, current.1 + 1);
                } else {
                    current = (current.0, current.1 - 1);
                }
            }
            '-' => {
                if have_come_from == Direction::West {
                    current = (current.0 + 1, current.1);
                } else {
                    current = (current.0 - 1, current.1);
                }
            }
            _ => {
                panic!("Unexpected pipe: {}", pipe);
            }
        }
    }

    ((path_length + 1) / 2, path)
}

fn solve_b(s: &str) -> usize {
    let (_, path) = solve_a(s);

    let grid = s
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut internal = 0;
    for (j, row) in grid.iter().enumerate() {
        let mut inside = false;
        for (i, c) in row.iter().enumerate() {
            if path.contains(&(i, j)) {
                print!("X");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    internal
}

pub fn day10() {
    let input = include_str!("../inputs/day10.txt");
    println!("Part A is: {}", solve_a(input).0);
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_a() {
    assert_eq!(
        solve_a(
            r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#
        )
        .0,
        8
    );
}

// #[test]
// fn example_b() {
//     assert_eq!(
//         solve_b(
//             r#".F----7F7F7F7F-7....
// .|F--7||||||||FJ....
// .||.FJ||||||||L7....
// FJL7L7LJLJ||LJ.L-7..
// L--J.L7...LJS7F-7L7.
// ....F-J..F7FJ|L7L7L7
// ....L7.F7||L7|.L7L7|
// .....|FJLJ|FJ|F7|.LJ
// ....FJL-7.||.||||...
// ....L---J.LJ.LJLJ..."#
//         ),
//         8
//     );
// }
