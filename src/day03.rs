use std::collections::HashMap;

use aoc23_rs::create_grid;

static DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn search_for_symbol(
    numbers: &Vec<char>,
    grid: &Vec<Vec<char>>,
    row: &Vec<char>,
    i: usize,
    j: usize,
) -> (u32, Vec<(usize, usize)>) {
    // We've finished a number, search around it to see if there was any symbols.
    let number_start: isize =
        (i as isize - numbers.len() as isize - 1).clamp(0, row.len() as isize - 1);
    let number_stop: isize = (i as isize).clamp(0, row.len() as isize - 1);

    let vertical_start = (j as isize - 1).clamp(0, grid.len() as isize - 1);
    let vertical_stop = (j as isize + 1).clamp(0, grid.len() as isize - 1);

    let mut number_value = 0;
    let mut gear_locations = vec![];
    for vertical in vertical_start..vertical_stop + 1 {
        for horizontal in number_start..number_stop + 1 {
            if !DIGITS.contains(&grid[vertical as usize][horizontal as usize])
                && grid[vertical as usize][horizontal as usize] != '.'
            {
                number_value = numbers.iter().collect::<String>().parse::<u32>().unwrap();
                if (grid[vertical as usize][horizontal as usize] == '*') {
                    gear_locations.push((horizontal as usize, vertical as usize));
                }
            }
        }
    }
    (number_value, gear_locations)
}

fn solve_a(s: &str) -> (usize, HashMap<(usize, usize), Vec<u32>>) {
    let grid = s
        .lines()
        .map(|c| c.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;

    // Anytime a number is next to a gear, log it against the gear-coordinate in this hash map.
    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (j, row) in grid.iter().enumerate() {
        let mut numbers = vec![];
        for (i, c) in row.iter().enumerate() {
            if DIGITS.contains(&c) {
                numbers.push(c.clone());
            } else {
                if !numbers.is_empty() {
                    let (n, gear_locations) = search_for_symbol(&numbers, &grid, row, i, j);
                    sum += n;
                    for gear_location in gear_locations {
                        let mut existing_parts = gear_map.get(&gear_location).unwrap_or(&vec![]).clone();
                        existing_parts.push(n);
                        gear_map.insert(gear_location, existing_parts);
                    }
                    numbers.clear();
                }
            }
        }
        if !numbers.is_empty() {
            let (n, gear_locations) = search_for_symbol(&numbers, &grid, row, row.len(), j);
            sum += n;
            for gear_location in gear_locations {
                let mut existing_parts = gear_map.get(&gear_location).unwrap_or(&vec![]).clone();
                existing_parts.push(n);
                gear_map.insert(gear_location, existing_parts);
            }
            numbers.clear();
        }
    }
    (sum as usize, gear_map)
}

fn solve_b(s: &str) -> u32 {
    let (_, gear_map) = solve_a(s);
    let mut sum = 0;
    for elem in gear_map.values() {
        if elem.len() == 2 {
            sum += elem[0] * elem[1]
        }
    }
    sum
}

pub fn day03() {
    let input = include_str!("../inputs/day03.txt");

    println!("Part A answer is: {:?}", solve_a(&input).0);
    println!("Part B answer is: {:?}", solve_b(&input));
}

#[test]
fn example_a() {
    assert_eq!(
        solve_a(
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
        )
        .0,
        4361
    );
}

#[test]
fn example_b() {
    assert_eq!(
        solve_b(
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
        ),
        467835
    );
}
