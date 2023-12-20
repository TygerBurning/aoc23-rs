use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::Debug,
    thread::{current, sleep},
    time::Duration,
};

#[derive(Eq, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
    Teleported,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::North => write!(f, "↑"),
            Self::East => write!(f, "→"),
            Self::South => write!(f, "↓"),
            Self::West => write!(f, "←"),
            Self::Teleported => write!(f, "o"),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Step {
    position: Coord,
    direction_travelled: Direction,
}

impl Debug for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.position, self.direction_travelled)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Journey {
    current_steps: Vec<Step>,
    total_heat_loss: u32,
    expected_heat_loss: u32, // Heuristic based on heat_loss + distance_from_end * 9
    straight_line_distance: u32,
}

#[derive(Clone, PartialEq, Eq)]
struct CityBlock {
    position: Coord,
    heat_loss: u32,
}

impl PartialOrd for Journey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.expected_heat_loss
            .partial_cmp(&other.expected_heat_loss)
    }
}

impl Ord for Journey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.expected_heat_loss.cmp(&other.expected_heat_loss)
    }
}

fn have_been_here_before(
    previous_visits: &Vec<Step>,
    current_position: &Coord,
    proposed_direction: Direction,
) -> bool {
    // println!("Searching previous visits: {:?}", previous_visits);
    let res = previous_visits.iter().any(|step| {
        &step.position == current_position && step.direction_travelled == proposed_direction
    });
    res
}

fn worst_case_heat_loss(x1: usize, x2: usize, y1: usize, y2: usize) -> u32 {
    x1.abs_diff(x2) as u32 + y1.abs_diff(y2) as u32
}

fn search(grid: &Vec<Vec<CityBlock>>) -> (Vec<Step>, u32) {
    let mut heap = BinaryHeap::new();

    heap.push(Reverse(Journey {
        current_steps: vec![Step {
            position: Coord { x: 0, y: 0 },
            direction_travelled: Direction::Teleported,
        }],
        total_heat_loss: 0,
        expected_heat_loss: 10000,
        straight_line_distance: 0,
    }));

    println!(
        "Searching - limits of the grid are: {}, {}",
        grid[0].len(),
        grid.len()
    );

    // TODO - don't explore the same direction if we've already gone the same way 3 times.
    // for (current_path, current_total) in &mut options_to_check {
    while !heap.is_empty() {
        let current_journey = heap.pop().unwrap().0;
        let mut previous_path = current_journey.current_steps.clone();
        let current = previous_path.pop().unwrap();

        println!("Exploring: {:?}!", current_journey);
        // We're done!
        if current.position
            == (Coord {
                x: grid[0].len() - 1,
                y: grid.len() - 1,
            })
        {
            println!("Found solution!");
            return (
                current_journey.current_steps.to_owned(),
                current_journey.total_heat_loss.to_owned(),
            );
        }

        println!(
            "The current path is: {:?} steps",
            current_journey.current_steps.len()
        );

        // Can we explore north?
        if current.direction_travelled != Direction::South
            && (current.direction_travelled != Direction::North
                || current_journey.straight_line_distance < 3)
            && current.position.y > 0
            && !have_been_here_before(&previous_path, &current.position, Direction::North)
        {
            println!("Explore north!");
            let new_x = current.position.x;
            let new_y = current.position.y - 1;
            let new_direction = Direction::North;

            let new_straight_line_distance = if current.direction_travelled == new_direction {
                current_journey.straight_line_distance + 1
            } else {
                1
            };
            let mut new_path = current_journey.current_steps.clone();
            new_path.push(Step {
                position: Coord { x: new_x, y: new_y },
                direction_travelled: new_direction,
            });
            let new_heat_loss = current_journey.total_heat_loss + grid[new_y][new_x].heat_loss;

            heap.push(Reverse(Journey {
                current_steps: new_path,
                total_heat_loss: new_heat_loss,
                expected_heat_loss: new_heat_loss
                    + worst_case_heat_loss(new_x, grid[0].len() - 1, new_y, grid.len() - 1),
                straight_line_distance: new_straight_line_distance,
            }));
        }

        // Can we explore east?
        if current.direction_travelled != Direction::West
            && (current.direction_travelled != Direction::East
                || current_journey.straight_line_distance < 3)
            && current.position.x < grid[0].len() - 1
            && !have_been_here_before(&previous_path, &current.position, Direction::East)
        {
            println!("Explore east!");
            let new_x = current.position.x + 1;
            let new_y = current.position.y;
            let new_direction = Direction::East;

            let new_straight_line_distance = if current.direction_travelled == new_direction {
                current_journey.straight_line_distance + 1
            } else {
                1
            };
            let mut new_path = current_journey.current_steps.clone();
            new_path.push(Step {
                position: Coord { x: new_x, y: new_y },
                direction_travelled: new_direction,
            });
            let new_heat_loss = current_journey.total_heat_loss + grid[new_y][new_x].heat_loss;

            heap.push(Reverse(Journey {
                current_steps: new_path,
                total_heat_loss: new_heat_loss,
                expected_heat_loss: new_heat_loss
                    + worst_case_heat_loss(new_x, grid[0].len() - 1, new_y, grid.len() - 1),
                straight_line_distance: new_straight_line_distance,
            }));
        }

        // Can we explore south?
        if current.direction_travelled != Direction::North
            && (current.direction_travelled != Direction::South
                || current_journey.straight_line_distance < 3)
            && current.position.y < grid.len() - 1
            && !have_been_here_before(&previous_path, &current.position, Direction::South)
        {
            println!("Explore south!");
            let new_x = current.position.x;
            let new_y = current.position.y + 1;
            let new_direction = Direction::South;

            let new_straight_line_distance = if current.direction_travelled == new_direction {
                current_journey.straight_line_distance + 1
            } else {
                1
            };
            let mut new_path = current_journey.current_steps.clone();
            new_path.push(Step {
                position: Coord { x: new_x, y: new_y },
                direction_travelled: new_direction,
            });
            let new_heat_loss = current_journey.total_heat_loss + grid[new_y][new_x].heat_loss;

            heap.push(Reverse(Journey {
                current_steps: new_path,
                total_heat_loss: new_heat_loss,
                expected_heat_loss: new_heat_loss
                    + worst_case_heat_loss(new_x, grid[0].len() - 1, new_y, grid.len() - 1),
                straight_line_distance: new_straight_line_distance,
            }));
        }

        // Can we explore west?
        if current.direction_travelled != Direction::East
            && (current.direction_travelled != Direction::West
                || current_journey.straight_line_distance < 3)
            && current.position.x > 0
            && !have_been_here_before(&previous_path, &current.position, Direction::West)
        {
            println!("Explore west!");
            let new_x = current.position.x - 1;
            let new_y = current.position.y;
            let new_direction = Direction::West;

            let new_straight_line_distance = if current.direction_travelled == new_direction {
                current_journey.straight_line_distance + 1
            } else {
                1
            };
            let mut new_path = current_journey.current_steps.clone();
            new_path.push(Step {
                position: Coord { x: new_x, y: new_y },
                direction_travelled: new_direction,
            });
            let new_heat_loss = current_journey.total_heat_loss + grid[new_y][new_x].heat_loss;

            heap.push(Reverse(Journey {
                current_steps: new_path,
                total_heat_loss: new_heat_loss,
                expected_heat_loss: new_heat_loss
                    + worst_case_heat_loss(new_x, grid[0].len() - 1, new_y, grid.len() - 1),
                straight_line_distance: new_straight_line_distance,
            }));
        }
    }

    (vec![], 0)
}

fn solve_a(input: &str) -> u32 {
    let city_blocks = input
        .lines()
        .enumerate()
        .map(|(j, s)| {
            s.chars()
                .enumerate()
                .map(|(i, c)| CityBlock {
                    position: Coord { x: i, y: j },
                    heat_loss: c.to_digit(10).unwrap(),
                })
                .collect::<Vec<CityBlock>>()
        })
        .collect::<Vec<Vec<CityBlock>>>();

    search(&city_blocks).1
}

fn solve_b(input: &str) -> u32 {
    0
}

pub fn day17() {
    let input = include_str!("../inputs/day17.txt");

    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_0() {
    let input = r#"11111
11111
11111"#;
    assert_eq!(solve_a(input), 6);
}

// #[test]
// fn example_1() {
//     let input = r#"2413432311323
// 3215453535623
// 3255245654254
// 3446585845452
// 4546657867536
// 1438598798454
// 4457876987766
// 3637877979653
// 4654967986887
// 4564679986453
// 1224686865563
// 2546548887735
// 4322674655533"#;
//     assert_eq!(solve_a(input), 102)
// }
