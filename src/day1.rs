use std::collections::HashMap;
use std::path::Path;
use regex::Regex;
use crate::common::{ get_trimmed_lines };

pub fn part1(file_path: &Path) -> String {
    let values: Vec<u64> = get_trimmed_lines(file_path).iter().map(|line| {
        let first = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
        format!("{first}{last}").parse::<u64>().unwrap()
    }).collect();

    let sum: u64 = values.iter().sum();
    format!("{}", sum)
}


pub fn part2(file_path: &Path) -> String {
    let number_words = [
        ("one", 1u8),
        ("two", 2u8),
        ("three", 3u8),
        ("four", 4u8),
        ("five", 5u8),
        ("six", 6u8),
        ("seven", 7u8),
        ("eight", 8u8),
        ("nine", 9u8),
    ];
    let mut pattern_map: HashMap<String, u8> = HashMap::new();
    for n in 1..=9 {
        pattern_map.insert(n.to_string(), n);
    }
    for (word, n) in number_words.iter() {
        pattern_map.insert(word.to_string(), *n);
        let reversed: String = word.chars().rev().collect();
        pattern_map.insert(reversed, *n);
    }
    let mut pattern_keys: Vec<&str> = pattern_map.keys().into_iter().map(|s| s.as_str()).collect();

    let regex = Regex::new(pattern_keys.join("|").as_str()).unwrap();

    let values: Vec<u64> = get_trimmed_lines(file_path).iter().map(|line| {
        // find returns byte index, not char index, but that's fine for this purpose
        let first = regex.find_iter(line).map(|_match| {
            let pos = _match.start();
            let count = pattern_map.get(_match.as_str()).unwrap();
            (pos, count)
        }).min_by_key(|tup| tup.0).unwrap().1;

        let line_reversed: String = line.chars().into_iter().rev().collect();
        let last = regex.find_iter(line_reversed.as_str()).map(|_match| {
            let pos = _match.start();
            let count = pattern_map.get(_match.as_str()).unwrap();
            (pos, count)
        }).min_by_key(|tup| tup.0).unwrap().1;
        let ret = format!("{first}{last}").parse::<u64>().unwrap();
        println!("ret: {}", ret);
        ret
    }).collect();

    let sum: u64 = values.iter().sum();
    format!("{}", sum)
}