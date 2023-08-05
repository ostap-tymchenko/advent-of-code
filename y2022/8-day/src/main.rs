use color_eyre::eyre::Result;
use core::fmt;
use std::any::type_name;
use std::fs;
use std::path::Path;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

const DATA_FOLDER: &str = "data";
const FOLDER_SPLIT: &str = "/";

// fn part_one(file_name: &str) -> i32 {
//     let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
//     let data = read_data(Path::new(&data_path));
//
//     todo!()
// }

#[derive(Debug, Default, Clone)]
struct Forrest(Vec<Vec<u32>>);

impl Forrest {
    fn get(&self, y: usize, x: usize) -> u32 {
        (*self
            .0
            .iter()
            .nth(y)
            .expect("y out of range")
            .iter()
            .nth(x)
            .expect("x out of range"))
        .try_into()
        .expect("couldnt convery i32 to u32")
    }

    fn push(&mut self, v: Vec<u32>) {
        self.0.push(v);
    }
}

impl fmt::Display for Forrest {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for v in self.0.iter() {
            for c in v {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}

// fn parse(input_data: String) -> Vec<Vec<i8>> {
//     let mut parsed_forrest: Vec<Vec<i8>> = Vec::new();
//
//     for line in input_data.split('\n') {
//         let mut parsed_line: Vec<i8> = Vec::new();
//         for number in line.chars() {
//             parsed_line.push(number.to_digit(10).unwrap() as i8);
//         }
//         if !parsed_line.is_empty() {
//             parsed_forrest.push(parsed_line);
//         }
//     }
//     parsed_forrest
// }

// fn parse_forrest(data: String) -> Forrest {
//     let mut forrest = Forrest(vec![vec![]]);
//     for line in data.lines() {
//         forrest.push(
//             line.chars()
//                 .map(|c| c.to_digit(10).expect("char should be pos <10"))
//                 .collect::<Vec<u32>>(),
//         );
//     }
//
//     forrest
// }


fn parse_forrest(data: String) -> Forrest {
    Forrest(
        data.lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("char should be pos <10"))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>(),
    )
}

fn scan_right(f: Forrest, x: usize, y:usize) -> usize {

    todo!()
}


fn part_two(file_name: &str) -> i32 {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));
    let forrest = parse_forrest(data);
    let forrest_clone = forrest.clone();
    let len = 0..forrest.0.len();

    for (y, line) in forrest.0.iter().enumerate() {
        for (x, tree) in line.iter().enumerate() {
            for i in len {
                if let Some(tmp_name) = forrest.get(y, x) {

                } else {
                    return i;
                }
            }
        }
    }
    
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     assert_eq!(part_one("dummy-data.txt"), 21);
    // }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("dummy-data.txt"), 8);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    // println!("{}", part_one("data.txt"));
    println!("{}", part_two("data.txt"));

    Ok(())
}
