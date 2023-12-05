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
    let mut total = 0;
    for line in data.lines() {
        let a: Vec<char> = line.chars().filter(|x| x.is_digit(10)).collect();
        let first = a.first().unwrap();
        let last = a.last().unwrap();
        let line_total: i32 = format!("{first}{last}").parse().unwrap();
        dbg!(line_total);
        total += line_total;
    }

    total
}

fn part_two(data: String) -> i32 {
    let mut total = 0;
    for mut line in data.lines() {
        let replaced = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

        // dbg!(&replaced);

        let filtered: Vec<char> = replaced.chars().filter(|x| x.is_digit(10)).collect();
        dbg!(&filtered);
        let first = filtered.first().unwrap();
        let last = filtered.last().unwrap();
        let line_total: i32 = format!("{first}{last}").parse().unwrap();
        dbg!(line_total);
        total += line_total;    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(read_data_from_name("dummy-data.txt")), 142);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(read_data_from_name("dummy-data-2.txt")), 281);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    // println!("{}", part_one(read_data_from_name("data.txt")));
    println!("{}", part_two(read_data_from_name("data.txt")));

    Ok(())
}
