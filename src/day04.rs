use std::collections::HashMap;

fn build_map(input: &str) -> HashMap<usize, u32> {
    let mut hm = HashMap::new();
    for (index, card) in input.lines().enumerate() {
        let numbers = card.split(':').collect::<Vec<&str>>()[1];
        let winning_numbers = numbers.split("|").collect::<Vec<&str>>()[0]
            .split(" ")
            .filter(|c| c != &"")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let my_numbers = numbers.split("|").collect::<Vec<&str>>()[1]
            .split(" ")
            .filter(|c| c != &"")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let won = my_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();
        hm.insert(index, won as u32);
    }
    hm
}

fn solve_a(input: &str) -> usize {
    let hm = build_map(input);

    hm.values()
        .map(|won| if won > &0 { 2_usize.pow(won - 1) } else { 0 })
        .sum()
}

fn solve_b(input: &str) -> i32 {
    let hm = build_map(input);
    let mut new_hm: HashMap<usize, i32> = HashMap::new();
    for i in 0..hm.len() {
        let mut cards_owned = new_hm.get(&i).unwrap_or(&0).clone();
        cards_owned += 1; // You always get one card for free.

        for following_cards in 0..hm.get(&i).unwrap().clone() {
            let following_card_index = i + following_cards as usize + 1 as usize;
            let mut subsequent_cards = new_hm.get(&following_card_index).unwrap_or(&0).clone();
            subsequent_cards += cards_owned;
            println!(
                "Because card: {} has {} copies, adding {} to {} (so it's now: {})",
                i, cards_owned, cards_owned, following_card_index, subsequent_cards
            );
            new_hm.insert(following_card_index, subsequent_cards);
        }
    }

    // *waves hands* we forgot to add one to our count when we reached it, so give ourselves a bonus card for each now.
    new_hm.values().sum::<i32>() + hm.len() as i32
}

pub fn day04() {
    let input = include_str!("../inputs/day04.txt");
    println!("Part A is: {}", solve_a(&input));
    println!("Part B is: {}", solve_b(&input));
}

#[test]
fn example_a() {
    assert_eq!(
        solve_a(
            r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
        ),
        13
    );
}

#[test]
fn example_b() {
    assert_eq!(
        solve_b(
            r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
        ),
        30
    );
}
