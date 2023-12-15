use aoc23_rs::create_grid;

pub fn day03() {
    let input = include_str!("../inputs/day03.txt");
    let g = create_grid(input);
    
    println!("Part A answer is: {:?}", 0);
    println!("Part B answer is: {:?}", g.index(1, 0));
}