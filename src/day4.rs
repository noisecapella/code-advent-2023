use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::Map;
use std::path::Path;
use std::vec::IntoIter;

use crate::common::{ get_trimmed_lines};

#[derive(Clone, Debug)]
struct Card {
    card_number: u64,
    winning_numbers: Vec<u64>,
    your_numbers: Vec<u64>,
}

fn calc_winning_card_count(card: &Card) -> usize {
    let winning_set: HashSet<u64> = HashSet::from_iter(card.winning_numbers.iter().map(|n| *n));
    card.your_numbers.iter().filter(|n| winning_set.contains(n)).count()
}

fn parse_cards(lines: Vec<String>) -> Vec<Card> {
    lines.into_iter().map(|line| {
        let pieces: Vec<&str> = line.split(": ").collect();
        let card_pieces: Vec<&str> = pieces[0].split_whitespace().collect();
        let card_number = card_pieces[1].parse::<u64>().unwrap();

        let number_pieces: Vec<Vec<u64>> = pieces[1].split("|").map(|part| {
            part.split_whitespace().map(|n| {
                n.trim().parse::<u64>().unwrap()
            }).collect()
        }).collect();

        Card {
            card_number: card_number,
            winning_numbers: number_pieces[0].clone(),
            your_numbers: number_pieces[1].clone(),
        }
    }).collect()
}

pub fn part1(file_path: &Path) -> String {
    let lines = get_trimmed_lines(file_path);

    let cards = parse_cards(lines);

    let count = cards.into_iter().map(|card| {
        let winning_num_count = calc_winning_card_count(&card);
        if winning_num_count == 0 {
            0
        } else {
            2u32.pow(winning_num_count as u32 - 1)
        }
    });

    let s = count.sum::<u32>();
    s.to_string()
}


pub fn part2(file_path: &Path) -> String {
    let lines = get_trimmed_lines(file_path);

    let original_cards: Vec<Card> = parse_cards(lines);

    let card_lookup: HashMap<u64, usize> = HashMap::from_iter(original_cards.iter().map(|card| {
        (card.card_number, calc_winning_card_count(card))
    }));

    let mut current_cards: VecDeque<u64> = original_cards.iter().map(|card| card.card_number).collect();
    let mut total_count = current_cards.len();

    loop {
        if current_cards.is_empty() {
            break;
        }

        let card_number = current_cards.pop_front().unwrap();
        let card_count = *card_lookup.get(&card_number).unwrap();
        for n in 0..card_count {
            current_cards.push_back(card_number + (n as u64) + 1);
            total_count += 1;
        }
    }

    total_count.to_string()
}