mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::Path;

fn main() {
    let matches = command!()
        .arg(arg!(-d --day <day>).required(true).value_parser(clap::value_parser!(u16).range(1..=25)))
        .arg(arg!(-p --part <part>).required(true).value_parser(clap::value_parser!(u16).range(1..=2)))
        .arg(arg!(-i --input <input>).required(true))
        .get_matches();
    let file_path: &Path = Path::new(matches.get_one::<String>("input").unwrap());
    let day: u16 = *matches.get_one::<u16>("day").unwrap();
    let part: u16 = *matches.get_one::<u16>("part").unwrap();

    println!("Day {}, part {}: ", day, part);

    let result: String = match day {
        1 => match part {
            1 => day1::part1(file_path),
            2 => day1::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        2 => match part {
            1 => day2::part1(file_path),
            2 => day2::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        3 => match part {
            1 => day3::part1(file_path),
            2 => day3::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        4 => match part {
            1 => day4::part1(file_path),
            2 => day4::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        5 => match part {
            1 => day5::part1(file_path),
            2 => day5::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        6 => match part {
            1 => day6::part1(file_path),
            2 => day6::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        7 => match part {
            1 => day7::part1(file_path),
            2 => day7::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        8 => match part {
            1 => day8::part1(file_path),
            2 => day8::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        9 => match part {
            1 => day9::part1(file_path),
            2 => day9::part2(file_path),
            _ => panic!("Unknown part {}", part),
        },
        _ => panic!("unknown day {}", day)
    };
    println!("Result: {}", result);
}
