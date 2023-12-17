use std::path::Path;
use crate::common::get_trimmed_lines;

fn parse_input(file_path: &Path) -> Vec<Vec<i64>> {
    let lines = get_trimmed_lines(file_path);

    lines.iter().map(|line| {
        line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect()
    }).collect()
}

fn calc_history_next(nums: &Vec<i64>) -> i64 {
    let mut triangle: Vec<Vec<i64>> = Vec::new();
    triangle.push(nums.clone());

    while let Some(last) = triangle.last() {
        if last.iter().all(|n| *n == 0) {
            break;
        }

        let mut next = Vec::with_capacity(last.len() - 1);
        for idx in 1..last.len() {
            next.push(last[idx] - last[idx - 1]);
        }
        triangle.push(next.clone());
    }

    let mut history = 0;
    for row in triangle.iter().rev().skip(1) {
        history = history + row.last().unwrap();
    }
    history
}

pub fn part1(file_path: &Path) -> String {
    let mut nums = parse_input(file_path);

    let histories: Vec<_> = nums.iter().map(|n| {
        calc_history_next(n)
    }).collect();

    histories.iter().sum::<i64>().to_string()
}

fn calc_history_prev(nums: &Vec<i64>) -> i64 {
    let mut triangle: Vec<Vec<i64>> = Vec::new();
    triangle.push(nums.clone());

    while let Some(last) = triangle.last() {
        if last.iter().all(|n| *n == 0) {
            break;
        }

        let mut next = Vec::with_capacity(last.len() - 1);
        for idx in 1..last.len() {
            next.push(last[idx] - last[idx - 1]);
        }
        triangle.push(next.clone());
    }

    triangle.reverse();
    let mut history = 0;
    for row in triangle.iter().skip(1) {
        // new-history + old-history = row.first().unwrap()

        println!("{:?} - {:?} = {:?}", row.first().unwrap(), history, row.first().unwrap() - history);
        history = row.first().unwrap() - history;
    }
    println!("---");
    history
}

pub fn part2(file_path: &Path) -> String {
    let mut nums = parse_input(file_path);

    let histories: Vec<_> = nums.iter().map(|n| {
        calc_history_prev(n)
    }).collect();

    histories.iter().sum::<i64>().to_string()
}
