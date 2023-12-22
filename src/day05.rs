// Search through all possible maps
fn find_next_val(val: u64, maps: &Vec<Vec<u64>>) -> u64 {
    // Look through each map in turn and see if it's present. Failing all this, we just return val.
    for map in maps {
        let dst = map[0];
        let src = map[1];
        let len = map[2];
        if src <= val && val < src + len {
            return dst + (val - src)
        }
    }

    val
}

fn solve_a(input: &str) -> u64 {
    let breakdown = input.split("\n\n").collect::<Vec<&str>>();

    let seeds = breakdown[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
    let mut maps_set = vec![];
    for almanac_map in &breakdown[1..] {
        maps_set.push(almanac_map.lines().skip(1).map(|s| s.split(" ").map(|c| c.parse::<u64>().unwrap()).collect::<Vec<u64>>()).collect::<Vec<Vec<u64>>>());

    }

    let mut locations = vec![];
    for seed in seeds {
        let mut val = seed;
        for maps in &maps_set {
            val = find_next_val(val, &maps)
        }
        locations.push(val);
    }

    println!("Maps: {:?}", maps_set);
    locations.iter().min().unwrap().to_owned()
}

fn solve_b(input: &str) -> usize {
    0
}

pub fn day05() {
    let input = include_str!("../inputs/day05.txt");

    println!("Part A is: {}", solve_a(input));
    println!("Part B is: {}", solve_b(input));
}

#[test]
fn example_1() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(solve_a(input), 35);
}