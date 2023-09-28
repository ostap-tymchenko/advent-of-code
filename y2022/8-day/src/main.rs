use color_eyre::eyre::Result;
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum ForrestMirrorOptions {
    InvisibleTree(u32),
    VisibleTree,
}

#[derive(Debug)]
struct ForrestMirror(Vec<Vec<ForrestMirrorOptions>>);

impl ForrestMirror {
    fn unchecked_set_visible(&mut self, c: Coordinates) {
        self.0[c.y][c.x] = ForrestMirrorOptions::VisibleTree;
    }
}

fn costruct_forrest(data: String) -> Forrest {
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

fn construct_mirror_forrest(data: String) -> ForrestMirror {
    ForrestMirror(
        data.lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("char should be pos <10"))
                    .map(|d| ForrestMirrorOptions::InvisibleTree(d))
                    .collect::<Vec<ForrestMirrorOptions>>()
            })
            .collect::<Vec<Vec<ForrestMirrorOptions>>>(),
    )
}

fn display_mirror(fm: &ForrestMirror) {
    println!();
    for line in fm.0.iter() {
        for tree in line {
            match tree {
                ForrestMirrorOptions::VisibleTree => print!("X"),
                ForrestMirrorOptions::InvisibleTree(t) => print!("{t}"),
            }
        }
        println!();
    }
}

fn flip_forrest_90_degrees(forrest: &mut Forrest) {
    let n = forrest.0.len();
    
    // Transpose the forrest
    for i in 0..n {
        for j in i..n {
            let temp = forrest.0[i][j];
            forrest.0[i][j] = forrest.0[j][i];
            forrest.0[j][i] = temp;
        }
    }
    
    // Reverse each row
    for i in 0..n {
        forrest.0[i].reverse();
    }
}

fn flip_mirror_forrest_90_degrees(forrest_mirror: &mut ForrestMirror) {
    let n = forrest_mirror.0.len();
    
    // Transpose the forrest
    for i in 0..n {
        for j in i..n {
            let temp = forrest_mirror.0[i][j];
            forrest_mirror.0[i][j] = forrest_mirror.0[j][i];
            forrest_mirror.0[j][i] = temp;
        }
    }
    
    // Reverse each row
    for i in 0..n {
        forrest_mirror.0[i].reverse();
    }
}

fn scan_right(forrest: &Forrest, forrest_mirror: &mut ForrestMirror) {
    for (y, line) in forrest.0.iter().enumerate() {
        let mut top_tree = 0;

        for (x, tree) in line.iter().enumerate() {
            if *tree > top_tree || x == 0 {
                forrest_mirror.unchecked_set_visible(Coordinates { x: x, y: y});
                top_tree = *tree;
            }
        }
    }
}

fn scan_left(forrest: &Forrest, forrest_mirror: &mut ForrestMirror) {
    let x_len = checked_constant_x_len(&forrest);

    for (y, line) in forrest.0.iter().rev().enumerate() {
        let mut top_tree = 0;
        for (x, tree) in line.iter().rev().enumerate() {
            if *tree > top_tree || x == 0 {
                forrest_mirror.unchecked_set_visible(Coordinates { x: (x_len -x -1), y: y});
                top_tree = *tree;
            }
        }
    }
}

fn scan_up(forrest: &Forrest, forrest_mirror: &mut ForrestMirror) {
    let y_len = y_len(&forrest);

}

fn part_one(file_name: &str) -> i32 {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));

    let mut forrest = costruct_forrest(data.clone());
    let mut forrest_mirror = construct_mirror_forrest(data);

    scan_right(&forrest, &mut forrest_mirror);
    scan_left(&forrest, &mut forrest_mirror);
    scan_up(&forrest, &mut forrest_mirror);

    // flip_forrest_90_degrees(&mut forrest);
    // flip_mirror_forrest_90_degrees(&mut forrest_mirror);

    // scan_right(&forrest, &mut forrest_mirror);
    // scan_left(&forrest, &mut forrest_mirror);

    display_mirror(&forrest_mirror);
    println!();
    // usize_to_i32(forrest_mirror.0.iter().map(|x| x.iter().filter(|x| **x == ForrestMirrorOptions::VisibleTree)).count())
    let mut seen_trees = 0;
    for line in forrest_mirror.0.iter() {
        for tree in line.iter() {
            if *tree == ForrestMirrorOptions::VisibleTree {
                seen_trees += 1;
            }
        }
    }
    seen_trees
}

fn part_two(file_name: &str) -> i32 {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));

    let forrest = costruct_forrest(data);

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

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("dummy-data.txt"), 8);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_one("data.txt"));
    // println!("{}", part_two("data.txt"));

    Ok(())
}
