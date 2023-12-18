use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use array2d::Array2D;


pub fn get_trimmed_lines(file_path: &Path) -> Vec<String> {
    let file_contents = fs::read_to_string(file_path).unwrap();

    file_contents.split("\n").map(|line| line.trim()).filter(|line| !line.is_empty()).map(|line| line.to_string()).collect()
}

type CoordType = (usize, usize);
type ValueType = u8;
type CostType = f64;

