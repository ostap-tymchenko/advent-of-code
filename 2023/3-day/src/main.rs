use color_eyre::eyre::Result;
use core::fmt;
use std::any::type_name;
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

struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map(Vec<Vec<char>>);

enum PointType {
    Number,
    BlankSpace,
    SpecialChar,
}

impl Map {
    fn get_type(&self, c: Coordinate) -> PointType {
        let point = self.0[c.x][c.y];

        if point.is_numeric() {
            return PointType::Number;
        } else if point == '.' {
            return PointType::BlankSpace;
        } else {
            return PointType::SpecialChar;
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for x in self.0.iter() {
            for y in x {
                write!(f, "[{y}]")?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

fn get_full_number(line: Vec<char>, y: usize) -> Result<i32> {
    let mut number_chars = String::new(); 
    for previous_y in (0..y).rev() {
        let previous_y = line[previous_y];

        if previous_y.is_numeric() {
            number_chars.insert(0, previous_y);
        } else {
            break;
        } 
    }

    for next_y in y.. {
        let next_y = line[next_y];

        if next_y.is_numeric() {
            number_chars.push(next_y);
        } else {
            break;
        }
    }

    Ok(number_chars.parse::<i32>()?)
}

fn scan_surroundings(map: &Map, c:Coordinate) -> Option<Coordinate> {
    // needs to return a n>=0 list of number points. Or alternativley just return the numbers
    // from here
}

fn part_one(data: String) -> i32 {
    let data = Map(data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect());

    println!("{data}");

    // for (x, line) in data.0.iter().enumerate() {
    //     let mut number_is_valid = false;
    //
    //     for (y, char) in line.iter().enumerate() {
    //         if char.is_numeric() {
    //             let mut left_hand_bound = y;
    //             let mut right_thand_bound = y;
    //
    //             'left_hand_search: for potential_y in (0..y).rev() {
    //                 let potential_y_point_type = data.get_type(Coordinate { x, y: potential_y });
    //
    //                 match potential_y_point_type {
    //                     PointType::Number => left_hand_bound = potential_y,
    //                     PointType::BlankSpace => break 'left_hand_search, // break loop, check valid surounding
    //                     PointType::SpecialChar => {
    //                         number_is_valid = true;
    //                         break 'left_hand_search;
    //                     } // break loop, number valid
    //                 }
    //             }
    //
    //             'right_hand_search: for potential_y in y.. {
    //                 let potential_y_point_type = data.get_type(Coordinate { x, y: potential_y });
    //
    //                 match potential_y_point_type {
    //                     PointType::Number => right_hand_bound = potential_y,
    //                     PointType::BlankSpace => break 'right_hand_search, // break loop, check valid surounding
    //                     PointType::SpecialChar => {
    //                         number_is_valid = true;
    //                         break 'right_hand_search;
    //                     } // break loop, number valid
    //                 }
    //             }
    //         }
    //     }
    // }

    todo!();
}

fn part_two(data: String) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(read_data_from_name("dummy-data.txt")), 4361);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(read_data_from_name("dummy-data.txt")), TODO);
    // }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let part = 1;
    println!(
        "have you made sure you are printing the right solution? you are printing part {part}"
    );
    println!();
    if part == 1 {
        println!("{}", part_one(read_data_from_name("data.txt")));
    } else if part == 2 {
        println!("{}", part_two(read_data_from_name("data.txt")));
    }

    Ok(())
}
