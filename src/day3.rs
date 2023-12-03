use std::collections::HashMap;
use std::fs::read;
use std::path::Path;
use crate::common::{ get_trimmed_lines };
use array2d::Array2D;

#[derive(Copy, Clone, Debug)]
struct NumberLocation {
    number: u32,
    length: usize,
    row: usize,
    col: usize,
}

fn read_numbered_locations(board: &Array2D<char>) -> Vec<NumberLocation> {
    let mut number_locations: Vec<NumberLocation> = Vec::new();
    let mut current: Option<NumberLocation> = None;

    for ((row, col), item) in board.enumerate_row_major() {
        match item.to_digit(10) {
            Some(digit) => {
                match &mut current {
                    None => {
                        current = Some(NumberLocation {
                            row: row,
                            col: col,
                            number: digit,
                            length: 1,
                        });
                    },
                    Some(_current) => {
                        _current.number = (&_current.number * 10) + digit;
                        _current.length += 1;
                    }
                }
            },
            None => {
                match current {
                    None => {

                    },
                    Some(_current) => {
                        number_locations.push(_current);
                        current = None
                    }
                }
            }
        }
    }
    number_locations
}

pub fn part1(file_path: &Path) -> String {
    let lines = get_trimmed_lines(file_path);
    let bytes: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().collect()
    }).collect();

    let board = Array2D::from_rows(&bytes).unwrap();

    let numbered_locations = read_numbered_locations(&board);

    let filtered = numbered_locations.into_iter().filter(|loc| {
        let locrow = loc.row as i32;
        let loccol = loc.col as i32;
        let loclength = loc.length as i32;
        let mut coords_to_check = Vec::new();
        coords_to_check.push((locrow - 1, loccol - 1));
        coords_to_check.push((locrow, loccol - 1));
        coords_to_check.push((locrow + 1, loccol - 1));
        coords_to_check.push((locrow - 1, loccol + loclength));
        coords_to_check.push((locrow, loccol + loclength));
        coords_to_check.push((locrow + 1, loccol + loclength));
        for col in loccol..loccol+loclength {
            coords_to_check.push((locrow - 1, col));
            coords_to_check.push((locrow + 1, col));
        }

        let filtered_coords = coords_to_check.into_iter().filter_map(|coord| {
            if coord.0 < 0 || coord.1 < 0 {
                None
            } else {
                Some((coord.0 as usize, coord.1 as usize))
            }
        });

        for (_row, _col) in filtered_coords {
            let c = board.get(_row, _col);
            match c {
                Some('.') => {
                },
                Some('0'..='9') => {
                },
                None => {
                },
                Some(_) => {
                    return true;
                }
            }
        }

        false
    });

    filtered.map(|loc| {
        println!("loc: {:?}", loc);
        loc.number
    }).sum::<u32>().to_string()
}

pub fn part2(file_path: &Path) -> String {
    let lines = get_trimmed_lines(file_path);
    let bytes: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().collect()
    }).collect();

    let board = Array2D::from_rows(&bytes).unwrap();

    let numbered_locations = read_numbered_locations(&board);

    let nearby_gears = numbered_locations.into_iter().enumerate().filter_map(|(idx, loc)| {
        let locrow = loc.row as i32;
        let loccol = loc.col as i32;
        let loclength = loc.length as i32;
        let mut coords_to_check = Vec::new();
        coords_to_check.push((locrow - 1, loccol - 1));
        coords_to_check.push((locrow, loccol - 1));
        coords_to_check.push((locrow + 1, loccol - 1));
        coords_to_check.push((locrow - 1, loccol + loclength));
        coords_to_check.push((locrow, loccol + loclength));
        coords_to_check.push((locrow + 1, loccol + loclength));
        for col in loccol..loccol+loclength {
            coords_to_check.push((locrow - 1, col));
            coords_to_check.push((locrow + 1, col));
        }

        let filtered_coords = coords_to_check.into_iter().filter_map(|coord| {
            if coord.0 < 0 || coord.1 < 0 {
                None
            } else {
                Some((coord.0 as usize, coord.1 as usize))
            }
        });

        for (_row, _col) in filtered_coords {
            let c = board.get(_row, _col);
            match c {
                Some('*') => {
                    return Some((loc, (_row, _col)));
                },
                _ => {

                }
            }
        }

        None
    });

    let mut gear_map = HashMap::new();
    for (loc, gear_coord) in nearby_gears {
        let locs = gear_map.entry(gear_coord).or_insert(Vec::new());
        locs.push(loc);
    }

    let gear_ratios = gear_map.iter().filter_map(|(gear_coord, locs)| {
        if locs.len() == 2 {
            Some(locs[0].number * locs[1].number)
        } else {
            None
        }
    });

    gear_ratios.sum::<u32>().to_string()
}