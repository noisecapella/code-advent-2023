use std::path::Path;
use crate::common::get_trimmed_lines;
use itertools::Itertools;

struct Race {
    time: u64,
    distance: u64,
}


fn _parse_part1(file_path: &Path) -> (Vec<u64>, Vec<u64>) {
    let mut time = None;
    let mut distance = None;

    for line in get_trimmed_lines(file_path) {
        let mut pieces = line.split_whitespace();
        let first = pieces.next().unwrap();
        let numbers: Vec<u64> = pieces.into_iter().map(|s| s.parse().unwrap()).collect();

        if first == "Time:" {
            time = Some(numbers);
        } else if first == "Distance:" {
            distance = Some(numbers);
        } else {
            panic!("unexpected");
        }
    }

    (time.unwrap(), distance.unwrap())
}
fn parse_input_part1(file_path: &Path) -> Vec<Race> {
    let (time, distance) = _parse_part1(file_path);

    let combined = (0..time.len()).into_iter().map(|idx| {
        Race {
            time: *time.get(idx).unwrap(),
            distance: *distance.get(idx).unwrap(),
        }
    }).collect();
    combined
}

fn calc_time(seconds_held_down: u64, seconds_total: u64) -> u64 {
    let seconds_released = seconds_total - seconds_held_down;
    seconds_held_down * seconds_released
}

pub fn part1(file_path: &Path) -> String {
    let races = parse_input_part1(file_path);

    let result: u64 = races.iter().map(|race| {
        (0..=race.time).filter(|ms| {
            calc_time(*ms, race.time) > race.distance
        }).count()
    }).product::<usize>() as u64;

    result.to_string()
}

fn _parse_part2(file_path: &Path) -> (Vec<u64>, Vec<u64>) {
    let mut time = None;
    let mut distance = None;

    for line in get_trimmed_lines(file_path) {
        let mut pieces = line.split_whitespace();
        let first = pieces.next().unwrap();
        let numbers: Vec<u64> = Vec::from(&[pieces.into_iter().join("").parse().unwrap()]);

        if first == "Time:" {
            time = Some(numbers);
        } else if first == "Distance:" {
            distance = Some(numbers);
        } else {
            panic!("unexpected");
        }
    }

    (time.unwrap(), distance.unwrap())
}

fn parse_input_part2(file_path: &Path) -> Vec<Race> {
    let (time, distance) = _parse_part2(file_path);

    let combined = (0..time.len()).into_iter().map(|idx| {
        Race {
            time: *time.get(idx).unwrap(),
            distance: *distance.get(idx).unwrap(),
        }
    }).collect();
    combined
}


pub fn part2(file_path: &Path) -> String {
    let races = parse_input_part2(file_path);

    let result: u64 = races.iter().map(|race| {
        (0..=race.time).filter(|ms| {
            calc_time(*ms, race.time) > race.distance
        }).count()
    }).product::<usize>() as u64;

    result.to_string()
}