use aoc23_rs::Grid;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone)]
struct GridSquare {
    from_north: bool, // We've already seen this light source coming *from* the north.
    from_east: bool,
    from_south: bool,
    from_west: bool,
    contents: char,
}

impl GridSquare {
    fn has_seen_from(&self, direction: &Direction) -> bool {
        match direction {
            Direction::North => self.from_north,
            Direction::South => self.from_south,
            Direction::East => self.from_east,
            Direction::West => self.from_west,
        }
    }

    fn seen_from(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.from_north = true,
            Direction::South => self.from_south = true,
            Direction::East => self.from_east = true,
            Direction::West => self.from_west = true,
        }
    }

    fn ever_seen(&self) -> u32 {
        if self.from_north || self.from_east || self.from_south || self.from_west {
            return 1;
        }
        0
    }
}

fn print_grid(grid: &Vec<Vec<GridSquare>>) {
    for row in grid {
        for elem in row {
            if elem.from_north {
                print!("v");
            } else if elem.from_east {
                print!("<");
            } else if elem.from_south {
                print!("^");
            } else if elem.from_west {
                print!(">");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn create_grid(input: &str) -> Vec<Vec<GridSquare>> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| GridSquare {
                    from_north: false,
                    from_east: false,
                    from_south: false,
                    from_west: false,
                    contents: c,
                })
                .collect::<Vec<GridSquare>>()
        })
        .collect::<Vec<Vec<GridSquare>>>()
}

fn solve(grid: &mut Vec<Vec<GridSquare>>, starting_square: (i32, i32, &Direction)) -> u32 {
    let mut elements_to_explore = vec![starting_square];
    while !elements_to_explore.is_empty() {
        let (x, y, from) = elements_to_explore.pop().unwrap();
        // Ignore this if the light has left the cavern
        if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
            continue;
        }

        if !grid[y as usize][x as usize].has_seen_from(&from) {
            // New grid square - process what to do.
            match grid[y as usize][x as usize].contents {
                '.' => {
                    // Carry on in the same direction
                    let mut new_x = x;
                    let mut new_y = y;
                    match from {
                        Direction::North => new_y = y + 1,
                        Direction::South => new_y = y - 1,
                        Direction::East => new_x = x - 1,
                        Direction::West => new_x = x + 1,
                    }
                    elements_to_explore.push((new_x, new_y, from));
                }
                '-' => match from {
                    Direction::North | Direction::South => {
                        elements_to_explore.push((x + 1, y, &Direction::West));
                        elements_to_explore.push((x - 1, y, &Direction::East));
                    }
                    Direction::East => {
                        elements_to_explore.push((x - 1, y, from));
                    }
                    Direction::West => {
                        elements_to_explore.push((x + 1, y, from));
                    }
                },
                '|' => match from {
                    Direction::East | Direction::West => {
                        elements_to_explore.push((x, y + 1, &Direction::North));
                        elements_to_explore.push((x, y - 1, &Direction::South));
                    }
                    Direction::North => {
                        elements_to_explore.push((x, y + 1, from));
                    }
                    Direction::South => {
                        elements_to_explore.push((x, y - 1, from));
                    }
                },
                '\\' => match from {
                    Direction::North => {
                        elements_to_explore.push((x + 1, y, &Direction::West));
                    }
                    Direction::South => {
                        elements_to_explore.push((x - 1, y, &Direction::East));
                    }
                    Direction::East => {
                        elements_to_explore.push((x, y - 1, &Direction::South));
                    }
                    Direction::West => {
                        elements_to_explore.push((x, y + 1, &Direction::North));
                    }
                },
                '/' => match from {
                    Direction::North => {
                        elements_to_explore.push((x - 1, y, &Direction::East));
                    }
                    Direction::South => {
                        elements_to_explore.push((x + 1, y, &Direction::West));
                    }
                    Direction::East => {
                        elements_to_explore.push((x, y + 1, &Direction::North));
                    }
                    Direction::West => {
                        elements_to_explore.push((x, y - 1, &Direction::South));
                    }
                },
                _ => panic!("Uh oh."),
            }
        }

        grid[y as usize][x as usize].seen_from(&from);
    }

    // print_grid(&grid);
    grid.iter()
        .map(|l| l.iter().map(|gs| gs.ever_seen()).sum::<u32>())
        .sum::<u32>()
}

fn solve_a(input: &str) -> u32 {
    solve(&mut create_grid(input), (0, 0, &Direction::West))
}

fn solve_b(input: &str) -> u32 {
    let mut max = 0;
    let grid = create_grid(input);
    let grid_height = grid.len() as i32;
    let grid_width = grid[0].len() as i32;
    for height in 0..grid_height {
        max = std::cmp::max(max, solve(&mut grid.clone(), (0, height, &Direction::West)));
        max = std::cmp::max(
            max,
            solve(&mut grid.clone(), (grid_width - 1, height, &Direction::East)),
        );
    }
    for width in 0..grid_width {
        max = std::cmp::max(max, solve(&mut grid.clone(), (width, 0, &Direction::North)));
        max = std::cmp::max(
            max,
            solve(&mut grid.clone(), (width, grid_height - 1, &Direction::South)),
        );
    }
    max
}

pub fn day16() {
    let input = include_str!("../inputs/day16.txt");

    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_1() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    assert_eq!(solve_a(input), 46)
}

#[test]
fn example_2() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    assert_eq!(solve_b(input), 51)
}
