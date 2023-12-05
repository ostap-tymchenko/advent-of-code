use color_eyre::eyre::Result;
use itertools::Itertools;
use std::{any::type_name, fs, path::Path};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(data_path).expect("data parse fail")
}

fn line_to_range_bounds(line: &str) -> (i32, i32, i32, i32) {
    let (str_range_a, str_range_b) = line.split(',').collect_tuple().unwrap();

    let (start1, end1) = str_range_a
        .split('-')
        .collect_tuple::<(&str, &str)>()
        .unwrap();

    let (start2, end2) = str_range_b
        .split('-')
        .collect_tuple::<(&str, &str)>()
        .unwrap();

    let (start1, start2, end1, end2) = (
        start1.parse::<i32>().unwrap(),
        start2.parse::<i32>().unwrap(),
        end1.parse::<i32>().unwrap(),
        end2.parse::<i32>().unwrap(),
    );

    return (start1, start2, end1, end2);
}

fn part_1() -> i32 {
    let data = read_data(Path::new("./data.txt"));
    let mut fully_containded_pairs = 0;

    for line in data.lines() {
        if !line.is_empty() {
            let (start1, start2, end1, end2) = line_to_range_bounds(line);
            if (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1) {
                fully_containded_pairs += 1;
            }
        }
    }

    fully_containded_pairs
}

fn part_2() -> i32 {
    let data = read_data(Path::new("./data.txt"));
    let mut overlapping_pairs = 0;

    for line in data.lines() {
        if !line.is_empty() {
            let (start1, start2, end1, end2) = line_to_range_bounds(line);
            // if range1.end >= range2.start && range1.start <= range2.end {
            if end1 >= start2 && start1 <= end2 {
                overlapping_pairs += 1;
            }
        }
    }
    overlapping_pairs
}

pub fn main() -> Result<()> {
    color_eyre::install()?;
    // println!("{}", part_1());
    println!("{}", part_2());

    Ok(())
}
