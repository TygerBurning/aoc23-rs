use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use regex::Regex;

#[derive(Eq, PartialEq, PartialOrd, Hash, Clone, Debug)]
enum Card {
    Jack = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
}

struct Player {
    hand: Vec<Card>,
    bid: usize,
}

fn to_card(c: &char) -> Card {
    match c {
        '1' => return Card::One,
        '2' => return Card::Two,
        '3' => return Card::Three,
        '4' => return Card::Four,
        '5' => return Card::Five,
        '6' => return Card::Six,
        '7' => return Card::Seven,
        '8' => return Card::Eight,
        '9' => return Card::Nine,
        'T' => return Card::Ten,
        'J' => return Card::Jack,
        'Q' => return Card::Queen,
        'K' => return Card::King,
        'A' => return Card::Ace,
        _ => panic!("Couldn't match: '{}'", c),
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
enum CamelCardTypes {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

fn determine_camel_card_type(hand: &Vec<Card>) -> CamelCardTypes {
    let mut hs: HashMap<Card, u32> = HashMap::new();
    for c in hand {
        *hs.entry(c.clone()).or_default() += 1;
    }

    match hs.len() {
        1 => CamelCardTypes::FiveOfAKind,
        2 => {
            // Either four of a kind, or full house.
            let x = hs.values().next().unwrap();
            if x == &1 || x == &4 {
                CamelCardTypes::FourOfAKind
            } else {
                CamelCardTypes::FullHouse
            }
        }
        3 => {
            // Either three of a kind, or two pair.
            for elem in hs.values() {
                if elem == &3 {
                    return CamelCardTypes::ThreeOfAKind;
                }
            }
            CamelCardTypes::TwoPair
        }
        4 => CamelCardTypes::OnePair,
        5 => CamelCardTypes::HighCard,
        _ => panic!("Impossible!"),
    }
}

fn determine_camel_card_type_with_joker(hand: &Vec<Card>) -> CamelCardTypes {
    let mut hs: HashMap<Card, usize> = HashMap::new();
    for c in hand {
        *hs.entry(c.clone()).or_default() += 1;
    }
    
    let max_seen = hs.iter().filter(|(c, i)| c != &&Card::Jack).map(|(_, i)| i).max().unwrap_or(&0);
    let num_jokers = hand.iter().filter(|c| c == &&Card::Jack).count();

    match max_seen {
        0 => match num_jokers {
            1 => CamelCardTypes::HighCard,
            2 => CamelCardTypes::OnePair,
            3 => CamelCardTypes::ThreeOfAKind,
            4 => CamelCardTypes::FourOfAKind,
            5 => CamelCardTypes::FiveOfAKind,
            _ => panic!("Ahh, {:?}", hand),
        },
        1 => match num_jokers {
            0 => CamelCardTypes::HighCard,
            1 => CamelCardTypes::OnePair,
            2 => CamelCardTypes::ThreeOfAKind,
            3 => CamelCardTypes::FourOfAKind,
            4 => CamelCardTypes::FiveOfAKind,
            _ => panic!("Ahh, {:?}", hand),
        }
        2 => match num_jokers {
            0 => {
                // One pair or two pair.
                if hs.len() == 3 {
                    // This must be 2 + 2 + 1
                    return CamelCardTypes::TwoPair
                }
                else {
                    return CamelCardTypes::OnePair
                }
            }
            1 => CamelCardTypes::ThreeOfAKind,
            2 => CamelCardTypes::FourOfAKind,
            3 => CamelCardTypes::FiveOfAKind,
            _ => panic!("Ahh, {:?}", hand),
        },
        3 => match num_jokers {
            0 => {
                if hs.len() == 2 {
                    // This must be a 3 + 2
                    return CamelCardTypes::FullHouse
                }
                else {
                    return CamelCardTypes::ThreeOfAKind
                }
            }                
            1 => CamelCardTypes::FourOfAKind,
            2 => CamelCardTypes::FiveOfAKind,
            _ => panic!("Ahh, {:?}", hand),
        },
        4 => match num_jokers {
                0 => CamelCardTypes::FourOfAKind,
                1 => CamelCardTypes::FiveOfAKind,
                _ => panic!("Ahh, {:?}", hand),
        },
        5 => CamelCardTypes::FiveOfAKind,
        _ => panic!("Impossible for hand: {:?}", hand),
    }
}

fn compare_hands(a: &Vec<Card>, b: &Vec<Card>) -> Ordering {
    let a_type = determine_camel_card_type_with_joker(a);
    let b_type = determine_camel_card_type_with_joker(b);
    if a_type == b_type {
        if a < b {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    } else {
        if a_type < b_type {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

pub fn day07() {
    let input = include_str!("../inputs/day07.txt");
    let handbet = Regex::new(r"(.{5}) ([0-9]*)").unwrap();

    let mut players = vec![];
    for p in input.lines() {
        let r = handbet.captures(p).unwrap();
        players.push(Player {
            hand: r[1].chars().map(|c| to_card(&c)).collect::<Vec<Card>>(),
            bid: r[2].parse::<usize>().unwrap(),
        });
    }
    println!("There are: {} hands", players.len());

    players.sort_by(|a, b| compare_hands(&a.hand, &b.hand));
    let mut res = 0;
    for (i, player) in players.iter().enumerate() {
        res += player.bid * (players.len() - i);
        println!(
            "Hand: {:?}, (classified as: {:?}) - res is now: {}",
            player.hand,
            determine_camel_card_type_with_joker(&player.hand),
            res
        );
    }

    println!("Part A is: {}", res);
}
