use crate::common::{ get_trimmed_lines };
use std::path::Path;

struct Draw {
    blue: u64,
    red: u64,
    green: u64,
}

struct Game {
    gameid: u64,
    draws: Vec<Draw>
}

fn parse_games(file_path: &Path) -> Vec<Game> {
    get_trimmed_lines(file_path).iter().map(|line| {
        let parts: Vec<&str> = line.split(": ").collect();
        let header: Vec<&str> = parts[0].split(" ").collect();
        let gameid = header[1].parse::<u64>().unwrap();

        let draws = parts[1].split("; ").into_iter().map(|drawline| {
            let cube_words: Vec<&str> = drawline.split(", ").collect();
            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;

            for word in cube_words.iter() {
                let pieces: Vec<&str> = word.split(" ").collect();
                match pieces[1] {
                    "blue" => {
                        blue = pieces[0].parse().unwrap()
                    },
                    "green" => {
                        green = pieces[0].parse().unwrap()
                    },
                    "red" => {
                        red = pieces[0].parse().unwrap()
                    },
                    _ => panic!("unknown color")
                };
            }

            Draw {
                blue: blue,
                green: green,
                red: red,
            }
        }).collect();

        Game {
            gameid: gameid,
            draws: draws
        }
    }).collect()
}

pub fn part1(file_path: &Path) -> String {

    let games = parse_games(file_path);

    let maxred = 12;
    let maxgreen = 13;
    let maxblue = 14;

    let validgames = games.iter().filter(|game| {
        game.draws.iter().all(|draw| {
            draw.blue <= maxblue && draw.green <= maxgreen && draw.red <= maxred
        })
    });

    validgames.map(|game| game.gameid).sum::<u64>().to_string()
}

pub fn part2(file_path: &Path) -> String {
    let games = parse_games(file_path);

    let powers: Vec<u64> = games.iter().map(|game| {
        let emptydraw = Draw {
            red: 0,
            green: 0,
            blue: 0
        };
        let mindraw = game.draws.iter().fold(emptydraw, |acc, x| {
           Draw {
               red: u64::max(acc.red, x.red),
               green: u64::max(acc.green, x.green),
               blue: u64::max(acc.blue, x.blue),
           }
        });
        mindraw.red * mindraw.green * mindraw.blue
    }).collect();
    let sum = powers.iter().sum::<u64>();

    sum.to_string()
}

