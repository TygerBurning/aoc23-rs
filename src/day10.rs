#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn solve_a(s: &str) -> usize {
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
    while current != start {
        path_length += 1;
        let pipe = grid[current.1][current.0];
        println!("Current pipe: {}", pipe);
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

    (path_length + 1) / 2
}

fn solve_b(s: &str) -> usize {
    0
}

pub fn day10() {
    let input = include_str!("../inputs/day10.txt");
    println!("Part A is: {}", solve_a(input));
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
        ),
        8
    );
}