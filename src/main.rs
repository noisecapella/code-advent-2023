mod common;
mod day1;
mod day2;
mod day3;

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
        _ => panic!("unknown day {}", day)
    };
    println!("Result: {}", result);
}
