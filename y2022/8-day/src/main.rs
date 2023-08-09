use color_eyre::eyre::Result;
// use color_eyre::Report;
use core::fmt;
// use std::any::type_name;
use std::fs;
use std::path::Path;

const DATA_FOLDER: &str = "data";
const FOLDER_SPLIT: &str = "/";

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(data_path).expect("data parse fail")
}

#[derive(Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Clone)]
struct Forrest(Vec<Vec<u32>>);

impl Forrest {
    fn unchecked_get(&self, c: Coordinates) -> u32 {
        self.0[c.y][c.x]
    }

    // fn get(&self, c: Coordinates) -> Result<u32, Report> {
    //     Ok(self.0[c.y][c.x])
    // }

    // fn push(&mut self, v: Vec<u32>) {
    //     self.0.push(v);
    // }
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

fn checked_constant_x_len(f: &Forrest) -> usize {
    let l = f.0.first().expect("no len with no trees").len();
    for y in &f.0 {
        if !y.len() == l {
            panic!("x len inconsistant in forrest");
        }
    }

    l
}

fn y_len(f: &Forrest) -> usize {
    f.0.len()
}

fn usize_to_i32(u: usize) -> i32 {
    u.try_into().expect("couldnt convert usize to i32")
}
fn part_two(file_name: &str) -> i32 {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));

    let forrest = parse_forrest(data);

    let x_len = checked_constant_x_len(&forrest);
    let y_len = y_len(&forrest);

    let mut top = 0;
    for (y_iter, line) in forrest.0.iter().enumerate() {
        for (x_iter, current_tree) in line.iter().enumerate() {
            if y_iter != 0 && x_iter != 0 {
                let mut view_len = [0; 4];

                for (i, up) in (0..y_iter).rev().enumerate() {
                    if forrest.unchecked_get(Coordinates { x: x_iter, y: up }) >= *current_tree {
                        view_len[0] = i + 1;
                        break;
                    } else {
                        view_len[0] += 1;
                    }
                }

                for (i, left) in (0..x_iter).rev().enumerate() {
                    if forrest.unchecked_get(Coordinates { x: left, y: y_iter }) >= *current_tree {
                        view_len[1] = i + 1;
                        break;
                    } else {
                        view_len[1] += 1;
                    }
                }

                for (i, down) in ((y_iter + 1)..y_len).enumerate() {
                    if forrest.unchecked_get(Coordinates { x: x_iter, y: down }) >= *current_tree {
                        view_len[2] = i + 1;
                        break;
                    } else {
                        view_len[2] += 1;
                    }
                }

                for (i, right) in ((x_iter + 1)..x_len).enumerate() {
                    if forrest.unchecked_get(Coordinates {
                        x: right,
                        y: y_iter,
                    }) >= *current_tree
                    {
                        view_len[3] = i + 1;
                        break;
                    } else {
                        view_len[3] += 1;
                    }
                }

                let scenic_score = view_len[0] * view_len[1] * view_len[2] * view_len[3];

                // dbg!(view_len);
                if scenic_score > top {
                    top = scenic_score;
                    // println!(
                    //     "new top: {top}, from view_len: {view_len:?}, at x:{x_iter}, y:{y_iter}"
                    // );
                }
            }
        }
    }

    usize_to_i32(top)
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
