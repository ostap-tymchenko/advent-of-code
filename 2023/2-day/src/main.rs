use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const DATA_FOLDER: &str = "data";
const FOLDER_SPLIT: &str = "/";

fn read_data_from_name(file_name: &str) -> String {
    let path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    fs::read_to_string(Path::new(&path)).expect("data parse fail")
}

fn part_one(data: String) -> i32 {
    let mut valid_ids = 0;
    for line in data.lines() {
        let id: i32 = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(':')
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();
        // println!("game {id} has started with: {line}");
        let replaced = line.replace(";", ",").replace(": ", ", ");

        let mut line_valid = true;

        'line_check: for entry in replaced.split(',').skip(1) {
            // println!("{entry}");

            if entry.contains("red")
                && entry
                    .split_whitespace()
                    .nth(0)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
                    > 12
            {
                // println!("game {id} has more than 12 red cubes");
                line_valid = false;
                break 'line_check;
            } else if entry.contains("green")
                && entry
                    .split_whitespace()
                    .nth(0)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
                    > 13
            {
                // println!("game {id} has more than 13 green cubes");
                line_valid = false;
                break 'line_check;
            } else if entry.contains("blue")
                && entry
                    .split_whitespace()
                    .nth(0)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
                    > 14
            {
                // println!("game {id} has more than 14 blue recubes");
                line_valid = false;
                break 'line_check;
            }
        }

        if line_valid {
            // println!("game {id} is valid. adding id {id} to valid id's {valid_ids}");
            valid_ids += id;
        }
    }

    valid_ids
}

fn part_two(data: String) -> i32 {
    let mut total = 0;
    for line in data.lines() {
        let id: i32 = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(':')
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();

        let mut color_max = HashMap::from([("blue", 1), ("red", 1), ("green", 1)]);

        let replaced = line.replace(";", ",").replace(": ", ", ");
        for entry in replaced.split(',').skip(1) {

            for color in ["blue", "red", "green"] {
                if entry.contains(color) {
                    let color_number = entry
                        .split_whitespace()
                        .nth(0)
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();

                    // if color_number > *color_max.get(color).unwrap() {
                    //     color_max.get_mut(color).unwrap() = color_number;
                    // }
                    if let Some(current_top) = color_max.get_mut(color) {
                        if color_number > *current_top {
                            *current_top = color_number;
                        }
                    }
                }
            }
        }

        // dbg!(&color_max);
        let power = color_max.get("red").unwrap() * color_max.get("green").unwrap() * color_max.get("blue").unwrap();
        total += power;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(read_data_from_name("dummy-data.txt")), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(read_data_from_name("dummy-data.txt")), 2286);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    // println!("{}", part_one(read_data_from_name("data.txt")));
    println!("{}", part_two(read_data_from_name("data.txt")));

    Ok(())
}
