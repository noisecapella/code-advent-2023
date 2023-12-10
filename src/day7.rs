use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::TrySendError::Full;
use crate::common::get_trimmed_lines;
use crate::day7::HandType::{FiveofaKind, FourofaKind, FullHouse, HighCard, OnePair, ThreeofaKind, TwoPair};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveofaKind = 7,
    FourofaKind = 6,
    FullHouse = 5,
    ThreeofaKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CardPart1 {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    _9 = 9,
    _8 = 8,
    _7 = 7,
    _6 = 6,
    _5 = 5,
    _4 = 4,
    _3 = 3,
    _2 = 2,
}

struct HandPart1 {
    hand: Vec<CardPart1>,
    handtype: HandType,
    bid: u64,
}

fn get_hand_type_part1(hand: &Vec<CardPart1>) -> HandType {
    let mut counter: HashMap<CardPart1, usize> = HashMap::new();
    for card in hand.iter() {
        counter.entry(*card).and_modify(|v| *v += 1 ).or_insert(1);
    }

    let mut sizes: Vec<usize> = counter.values().map(|v| *v).collect();
    sizes.sort();
    sizes.reverse();

    if sizes[0] == 5 {
        FiveofaKind
    } else if sizes[0] == 4 {
        FourofaKind
    } else if sizes[0] == 3 {
        if sizes[1] == 2 {
            FullHouse
        } else {
            ThreeofaKind
        }
    } else if sizes[0] == 2 {
        if sizes[1] == 2 {
            TwoPair
        } else {
            OnePair
        }
    } else {
        HighCard
    }
}

fn comparator_part1(a: &HandPart1, b: &HandPart1) -> Ordering {
    a.handtype.cmp(&b.handtype).then(a.hand.cmp(&b.hand))
}

fn parse_input_part1(file_path: &Path) -> Vec<HandPart1> {
    get_trimmed_lines(file_path).iter().map(|line| {
        let mut pieces = line.split_whitespace();
        let cards: Vec<CardPart1> = pieces.next().unwrap().chars().map(|c| {
            match c {
                'A' => CardPart1::A,
                'K' => CardPart1::K,
                'Q' => CardPart1::Q,
                'J' => CardPart1::J,
                'T' => CardPart1::T,
                '9' => CardPart1::_9,
                '8' => CardPart1::_8,
                '7' => CardPart1::_7,
                '6' => CardPart1::_6,
                '5' => CardPart1::_5,
                '4' => CardPart1::_4,
                '3' => CardPart1::_3,
                '2' => CardPart1::_2,
                _ => panic!("unexpected character")
            }
        }).collect();
        let bid = pieces.next().unwrap().parse().unwrap();
        let handtype = get_hand_type_part1(&cards);
        HandPart1 {
            hand: cards,
            handtype: handtype,
            bid,
        }
    }).collect()
}

pub fn part1(file_path: &Path) -> String {
    let mut hands = parse_input_part1(file_path);
    hands.sort_by(comparator_part1);
    let winnings: usize = hands.iter().enumerate().map(|(i, v)| {
        (i + 1) * (v.bid as usize)
    }).sum();
    winnings.to_string()
}


#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CardPart2 {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    _9 = 9,
    _8 = 8,
    _7 = 7,
    _6 = 6,
    _5 = 5,
    _4 = 4,
    _3 = 3,
    _2 = 2,
    J = 1,
}

struct HandPart2 {
    hand: Vec<CardPart2>,
    handtype: HandType,
    bid: u64,
}

fn get_hand_type_part2(hand: &Vec<CardPart2>) -> HandType {
    let mut counter: HashMap<CardPart2, usize> = HashMap::new();
    for card in hand.iter() {
        counter.entry(*card).and_modify(|v| *v += 1 ).or_insert(1);
    }
    let num_jokers = *counter.get(&CardPart2::J).unwrap_or(&0);
    counter.remove(&CardPart2::J);

    let mut sizes: Vec<usize> = counter.values().map(|v| *v).collect();
    sizes.sort();
    sizes.reverse();

    if sizes.len() > 0 {
        sizes[0] += num_jokers;
    } else {
        sizes.push(num_jokers);
    }

    if sizes[0] == 5 {
        FiveofaKind
    } else if sizes[0] == 4 {
        FourofaKind
    } else if sizes[0] == 3 {
        if sizes[1] == 2 {
            FullHouse
        } else {
            ThreeofaKind
        }
    } else if sizes[0] == 2 {
        if sizes[1] == 2 {
            TwoPair
        } else {
            OnePair
        }
    } else {
        HighCard
    }
}

fn comparator_part2(a: &HandPart2, b: &HandPart2) -> Ordering {
    a.handtype.cmp(&b.handtype).then(a.hand.cmp(&b.hand))
}

fn parse_input_part2(file_path: &Path) -> Vec<HandPart2> {
    get_trimmed_lines(file_path).iter().map(|line| {
        let mut pieces = line.split_whitespace();
        let cards: Vec<CardPart2> = pieces.next().unwrap().chars().map(|c| {
            match c {
                'A' => CardPart2::A,
                'K' => CardPart2::K,
                'Q' => CardPart2::Q,
                'J' => CardPart2::J,
                'T' => CardPart2::T,
                '9' => CardPart2::_9,
                '8' => CardPart2::_8,
                '7' => CardPart2::_7,
                '6' => CardPart2::_6,
                '5' => CardPart2::_5,
                '4' => CardPart2::_4,
                '3' => CardPart2::_3,
                '2' => CardPart2::_2,
                _ => panic!("unexpected character")
            }
        }).collect();
        let bid = pieces.next().unwrap().parse().unwrap();
        let handtype = get_hand_type_part2(&cards);
        HandPart2 {
            hand: cards,
            handtype: handtype,
            bid,
        }
    }).collect()
}

pub fn part2(file_path: &Path) -> String {
    let mut hands = parse_input_part2(file_path);
    hands.sort_by(comparator_part2);
    let winnings: usize = hands.iter().enumerate().map(|(i, v)| {
        (i + 1) * (v.bid as usize)
    }).sum();
    winnings.to_string()
}

