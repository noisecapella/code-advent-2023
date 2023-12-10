use std::collections::HashMap;
use std::path::Path;
use crate::common::get_trimmed_lines;
use crate::day5::Element::Fertilizer;

#[derive(Copy, Clone)]
struct Range {
    source_start: u64,
    dest_start: u64,
    len: usize,
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Element {
    Seed,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
    Soil,
}
type MapType = (Element, Element);

struct SeedInfo {
    seeds: Vec<u64>,
    maps: HashMap<MapType, Vec<Range>>,
}


fn parse_input(file_path: &Path) -> SeedInfo {
    let lines = get_trimmed_lines(file_path);
    let mut seeds: Option<Vec<u64>> = None;
    let mut current_map_type: Option<MapType> = None;
    let mut current_ranges: Vec<Range> = Vec::new();
    let mut maps: HashMap<MapType, Vec<Range>> = HashMap::new();

    for line in lines {
        if line.starts_with("seeds:") {
            seeds = Some(line.split(" ").skip(1).map(|s| s.parse().unwrap()).collect());
        }
        else if line.ends_with(" map:") {
            match current_map_type {
                Some(_type) => {
                    maps.insert(current_map_type.unwrap(), current_ranges.clone());
                },
                None => {}
            }

            let typename = line.split(" ").next().unwrap();
            let pieces: Vec<Element> = typename.split("-").filter(|s| s != &"to").map(|s| {
                match s {
                    "seed" => Element::Seed,
                    "fertilizer" => Element::Fertilizer,
                    "water" => Element::Water,
                    "light" => Element::Light,
                    "temperature" => Element::Temperature,
                    "humidity" => Element::Humidity,
                    "location" => Element::Location,
                    "soil" => Element::Soil,
                    _ => {
                        panic!("unknown element {}", s);
                    }
                }
            }).collect();
            current_map_type = Some((*pieces.get(0).unwrap(), *pieces.get(1).unwrap()));
            current_ranges.clear();
        }
        else if !line.is_empty() {
            let pieces: Vec<u64> = line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
            let range = Range {
                dest_start: *pieces.get(0).unwrap(),
                source_start: *pieces.get(1).unwrap(),
                len: (*pieces.get(2).unwrap()) as usize,
            };
            current_ranges.push(range);
        }

    }

    maps.insert(current_map_type.unwrap(), current_ranges.clone());

    SeedInfo {
        seeds: seeds.unwrap(),
        maps: maps,
    }
}

pub fn part1(file_path: &Path) -> String {
    "".to_string()
}

pub fn part2(file_path: &Path) -> String {
    "".to_string()
}