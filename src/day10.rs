use std::collections::{HashMap, VecDeque};
use std::path::Path;
use array2d::Array2D;
use crate::common::get_trimmed_lines;

fn parse_input(file_path: &Path) -> Array2D<u8> {
    let lines = get_trimmed_lines(file_path);
    let byterows: Vec<Vec<u8>> = lines.iter().map(|x| {
        x.bytes().collect()
    }).collect();

    Array2D::from_rows(byterows.as_slice()).unwrap()
}

fn _find_loop(board: &Array2D<u8>, start: (usize, usize), path: &mut Vec<(usize, usize)>) -> bool {
    // println!("_find_loop {:?}", path);
    let current = *path.last().unwrap_or(&start);

    if path.len() > 2 && start == current {
        return true;
    }

    let to_left = (current.0 as i64, current.1 as i64 + 1);
    let to_right = (current.0 as i64, current.1 as i64 - 1);
    let to_up = (current.0 as i64 - 1, current.1 as i64);
    let to_down = (current.0 as i64 + 1, current.1 as i64);

    for next_move in [to_left, to_right, to_up, to_down] {
        if !is_valid_move(&board, current, next_move) {
            continue;
        }
        let next_move_usize = (next_move.0 as usize, next_move.1 as usize);

        if path.contains(&next_move_usize) {
            continue;
        }

        path.push(next_move_usize);

        if _find_loop(board, start, path) {
            return true;
        }
        path.pop();
    }

    return false;
}

fn is_valid_move(board: &Array2D<u8>, current: (usize, usize), next: (i64, i64)) -> bool {
    if next.0 < 0 || next.0 >= (board.num_rows() as i64) || next.1 < 0 || next.1 >= (board.num_columns() as i64) {
        return false;
    }

    #[derive(Eq, PartialEq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let next_usize = (next.0 as usize, next.1 as usize);
    let direction =
        if next_usize.0 == current.0 {
            if next_usize.1 == current.1 + 1 {
                Direction::Right
            } else if next_usize.1 == current.1 - 1 {
                Direction::Left
            } else {
                return false;
            }
        } else if next_usize.1 == current.1 {
            if next_usize.0 == current.0 + 1 {
                Direction::Down
            } else if next_usize.0 == current.0 - 1 {
                Direction::Up
            } else {
                return false;
            }
        } else {
            return false;
        };

    match board[next_usize] {
        b'|' => {
            if direction == Direction::Left || direction == Direction::Right {
                return false;
            }
        }
        b'-' => {
            if direction == Direction::Up || direction == Direction::Down {
                return false;
            }
        },
        b'L' => {
            if direction == Direction::Up || direction == Direction::Right {
                return false;
            }
        },
        b'J' => {
            if direction == Direction::Up || direction == Direction::Left {
                return false;

            }
        },
        b'7' => {
            if direction == Direction::Down || direction == Direction::Left {
                return false;

            }
        },
        b'F' => {
            if direction == Direction::Down || direction == Direction::Right {
                return false;

            }
        },
        b'S' => {

        },
        _ => {

        }
    };

    match board[current] {
        b'|' => {
            direction == Direction::Up || direction == Direction::Down
        },
        b'-' => {
            direction == Direction::Left || direction == Direction::Right
        },
        b'L' => {
            direction == Direction::Up || direction == Direction::Right
        },
        b'J' => {
            direction == Direction::Up || direction == Direction::Left
        },
        b'7' => {
            direction == Direction::Down || direction == Direction::Left
        },
        b'F' => {
            direction == Direction::Down || direction == Direction::Right
        },
        b'S' => {
            true
        },
        _ => {
            false
        }
    }
}

fn find_loop(board: &Array2D<u8>, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut loop_path = Vec::new();
    if !_find_loop(board, start_pos, &mut loop_path) {
        panic!("unable to find loop");
    }

    let mut ret = Vec::new();
    ret.push(start_pos);
    ret.extend(loop_path);
    println!("loop is {:?}", ret);
    ret
}

pub fn part1(file_path: &Path) -> String {
    let board = parse_input(file_path);

    let start_pos = board.enumerate_row_major().find_map(|(pos, item)| {
        if *item == b'S' {
            Some(pos)
        } else {
            None
        }
    }).unwrap();

    let loop_path = find_loop(&board, start_pos);

    (loop_path.len() / 2).to_string()
}

pub fn part2(file_path: &Path) -> String {
    "".to_string()
}
