use color_eyre::eyre::Result;
use std::any::type_name;
use std::path::Path;
use std::fs;
use nom::*;

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const DATA_FOLDER: &str = "data";
const FOLDER_SPLIT: &str = "/";
const ALPHABET_START: u8 = b'a';
const ALPHABET_LEN: u8 = b'z' - b'a';

fn read_data_from_name(file_name: &str) -> String {
    let path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    fs::read_to_string(Path::new(&path)).expect("data parse fail")
}

#[derive(Debug, Default)]
struct Point {
    y: i32,
    x: i32,
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    use nom::combinator::map;

    let (input, number) = map(digit, i32::from_str)(input)?;
    Ok((input, number))
}

fn part_one(data: String) -> i32 {
    // This solution wont be so clean. In the future im planing to make functions to
    // to generate a node graph and traverse it via a*. atm Im its just goint to be
    // brute force. Im really very sorry about what your about to read :/

    let input_str = "aabqponm\nabcryxxl\naccszExk\naccSuvwj\nabdefghi";
    println!("{input_str}");

    // let mut input_vec: Vec<Vec<u8>> = vec![];
    // let mut start;

    // let start = Point {x:input_str.lines().enimerate().contains("")}
    // let start = Point { y: input_str.lines().enumerate()[0].contains(""), x: 0};
    

    // for (iter, line) in input_str.lines().enumerate() {
    //     // the filter is here because for some reason there are 4 extra spaces in each but the
    //     // first vec at the front. I do not know why.
    //     if let Some(a) = line.find('S') {
    //         start = Point {
    //             x: a as i32,
    //             y: iter as i32,
    //         };
    //     }
    //
    //     input_vec.push(
    //         line.to_lowercase().chars()
    //             .filter(|&c| c != ' ')
    //             .map(|c| c as u8 - ALPHABET_START)
    //             .collect(),
    //     );
    // }
    //
    // dbg!(input_vec, start);
    //
    todo!();
}

// fn part_two(data: String) -> i32 {
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(read_data_from_name("dummy-data.txt")), 31);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(read_data_from_name("dummy-data.txt")), TODO);
    // }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_one(read_data_from_name("data.txt")));

    Ok(())
}
