use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::fmt;
use std::fmt::Formatter;
use crate::common::get_trimmed_lines;

#[derive(Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct NodeKey {
    key: (u8, u8, u8)
}

impl fmt::Debug for NodeKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeKey")
            .field("key", &format!(
                "{}{}{}", self.key.0 as char, self.key.1 as char, self.key.2 as char
            ))
            .finish()
    }
}

impl NodeKey {
    pub fn endswith(&self, b: u8) -> bool {
        self.key.2 == b
    }
}

struct Instructions {
    instructions: Vec<Instruction>,
    node_map: HashMap<NodeKey, (NodeKey, NodeKey)>,
}

fn _to_node_key(s: &str) -> NodeKey {
    if s.len() != 3 {
        panic!("unexpected");
    }
    let key_string_bytes: Vec<u8> = s.bytes().collect();
    NodeKey { key: (key_string_bytes[0], key_string_bytes[1], key_string_bytes[2]) }
}

fn parse_input(file_path: &Path) -> Instructions {
    let lines = get_trimmed_lines(file_path);
    let mut lines_iter = lines.iter();
    let instructions_chars = lines_iter.next().unwrap().chars();
    let instructions = instructions_chars.map(|c| {
        match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => panic!("unknown character")
        }
    }).collect();

    let mut node_map: HashMap<NodeKey, (NodeKey, NodeKey)> = HashMap::new();
    for line in lines_iter {
        let pieces: Vec<&str> = line.split(" = ").collect();
        let key = _to_node_key(pieces.get(0).unwrap());
        let value: Vec<&str> = pieces.get(1).unwrap().trim_matches(|c| { c == '(' || c == ')' }).split(", ").collect();
        node_map.insert(key, (_to_node_key(value[0]), _to_node_key(value[1])));
    }

    Instructions {
        instructions: instructions,
        node_map: node_map,
    }
}

pub fn part1(file_path: &Path) -> String {
    let instructions = parse_input(file_path);
    let mut current = NodeKey { key: (b'A', b'A', b'A') };
    let goal = NodeKey{ key: (b'Z', b'Z', b'Z') };

    let mut num_steps = 0;
    let mut instruction_index = 0;
    while current != goal {
        num_steps += 1;
        //println!("{:?} {:?}", instructions.node_map, current);
        let (left, right) = instructions.node_map.get(&current).unwrap();
        let instruction = instructions.instructions[instruction_index];
        let next = *match instruction {
            Instruction::Left => left,
            Instruction::Right => right,
        };
        current = next;
        instruction_index += 1;
        if instruction_index == instructions.instructions.len() {
            instruction_index = 0;
        }
    }

    num_steps.to_string()
}

#[derive(Copy, Clone, Debug)]
struct IteratorState {
    initial_num_steps: usize,
    current_num_steps: Option<usize>,
    inc: Option<usize>,
}

impl Iterator for IteratorState {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_num_steps;
        self.current_num_steps = match self.current_num_steps {
            Some(_current) => {
                match self.inc {
                    Some(_inc) => {
                        Some(_current + _inc)
                    },
                    None => {
                        None
                    }
                }
            },
            None => {
                None
            }
        };
        current
    }
}

struct IteratorDeque {
    iterator_state: IteratorState,
    deque: VecDeque<usize>,
}

impl IteratorDeque {
    pub fn peek(&mut self) -> usize {
        if self.deque.len() == 0 {
            self.deque.push_front(self.iterator_state.next().unwrap());
        }
        *self.deque.get(0).unwrap()
    }

    pub fn remove_first(&mut self) {
        self.deque.pop_front();
    }
}

fn calc_iterators(starts: &Vec<NodeKey>, instructions: &Instructions) -> HashMap<NodeKey, HashMap<(NodeKey, usize), IteratorState>> {
    let mut recurrence_first: HashMap<NodeKey, HashMap<(NodeKey, usize), usize>> = HashMap::new();
    let mut recurrence_second: HashMap<NodeKey, HashMap<(NodeKey, usize), usize>> = HashMap::new();

    for initial_key in starts.iter() {
        let mut first_map = HashMap::new();
        let mut second_map = HashMap::new();

        let mut instruction_index = 0;
        let mut current = *initial_key;
        let mut num_steps = 0;
        loop {
            let next = match instructions.instructions[instruction_index] {
                Instruction::Left => {
                    instructions.node_map[&current].0
                },
                Instruction::Right => {
                    instructions.node_map[&current].1
                }
            };

            if !first_map.contains_key(&(next, instruction_index)) {
                first_map.insert((next, instruction_index), num_steps + 1);
            } else if !second_map.contains_key(&(next, instruction_index)) {
                second_map.insert((next, instruction_index), num_steps + 1);
            } else {
                break;
            }

            num_steps += 1;
            current = next;
            instruction_index += 1;
            if instruction_index == instructions.instructions.len() {
                instruction_index = 0;
            }
        }

        recurrence_first.insert(*initial_key, first_map);
        recurrence_second.insert(*initial_key, second_map);
    }

    let iterators: HashMap<_, _> = starts.iter().map(|start| {
        let first = recurrence_first.get(start).unwrap();
        let second = recurrence_second.get(start).unwrap();

        let mut iters = HashMap::new();
        for (first_key, first_value) in first.iter() {
            let second_value = second.get(first_key);

            iters.insert(*first_key, IteratorState {
                initial_num_steps: *first_value,
                current_num_steps: Some(*first_value),
                inc: match second_value {
                    Some(_second_value) => {
                        Some(_second_value - first_value)
                    },
                    None => None
                }
            });
        }
        //println!("start {:?} {:?}", start, iters);
        (*start, iters)
    }).collect();

    iterators
}

fn _factorize(n: usize, factors: &mut HashSet<usize>) {
    for i in 2..(n - 1) {
        if n % i == 0 {
            factors.insert(i);
            return _factorize(n / i, factors);
        }
    }
    factors.insert(n);
}

fn factorize(n: usize) -> HashSet<usize> {
    let mut factors = HashSet::new();

    _factorize(n, &mut factors);

    factors
}

pub fn part2(file_path: &Path) -> String {
    let instructions = parse_input(file_path);

    let starts: Vec<NodeKey> = instructions.node_map.keys().filter_map(|s| {
        if s.endswith(b'A') {
            Some(*s)
        } else {
            None
        }
    }).collect();

    let iterator_map = calc_iterators(&starts, &instructions);

    let mut combined_factors = HashSet::new();
    for (start, end_map) in iterator_map.iter() {
        for (end_tup, end_iter) in end_map.iter() {
            if end_tup.0.endswith(b'Z') {
                println!("Starting from {:?} to {:?}, {:?} + {:?}x", start, end_tup, end_iter.initial_num_steps, end_iter.inc);

                if end_iter.inc.unwrap() != end_iter.initial_num_steps {
                    // the results showed all inc == initial for whatever reason, so just the case where it isn't for simplicity
                    panic!("unexpected");
                }

                let factors = factorize(end_iter.inc.unwrap());
                println!("factors {:?}", factors);
                combined_factors.extend(factors);
            }
        }
    }

    combined_factors.iter().product::<usize>().to_string()
}
