use itertools::Itertools;
use std::{collections::BTreeSet, fs, path::Path};

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

fn priority_check(letter: char) -> i32 {
    let priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    priority.find(letter).expect("invalid letter input at priority check").try_into().expect("this should never panic!")
}

pub fn part_one() -> i32 {
    let data = read_data(Path::new("./data.txt"));

    let mut added_priority = 0;
    for line in data.lines() {
        // for split in line.split_at(line.len() /2) {
        let line = line.split_at(line.len() / 2);

        let first = line.0;
        let second = line.1;

        let all_in_first = BTreeSet::from_iter(first.chars());

        let mut common_char = '\n';
        for var_char in second.chars() {
            if all_in_first.contains(&var_char) {
                common_char = var_char
            }
        }

        let priority_buffer = priority_check(common_char);
        added_priority += priority_buffer + 1; // +1 for 0-index in priority_check
    }
    added_priority
}

pub fn part_two() -> i32 {
    let data = read_data(Path::new("./data.txt"));

    let lines = data.lines();
    let mut total_priority = 0;
    for (a, b, c) in lines.tuples() {
        let mut common_char = '🦀';
        for a_char in a.chars() {
            for b_char in b.chars() {
                for c_char in c.chars() {
                    if a_char == b_char && b_char == c_char {
                        common_char = a_char
                    }
                }
            }
        }
        
        if common_char == '🦀' {
            panic!("Placeholder crab activated")
        }
        total_priority += priority_check(common_char) + 1 // +1 for 0-index in priority_check
    }
    total_priority
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
