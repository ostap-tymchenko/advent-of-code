use color_eyre::eyre::Result;
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

#[derive(Debug, Clone, Default)]
struct Forrest(Vec<String>);

impl Forrest {
    fn get(&self, x:usize, y:usize) -> char {
        self.0.iter().nth(x).expect("get out of bounds").chars().nth(x).expect("get out of bounds")
    }

    fn push(&mut self, line: String) {
        self.0.push(line);
    }

    // fn delete(&mut self, x:usize, y:usize) {
    //     // self.0.iter().nth(y).expect("y out of bounds").chars().nth(x) = '_'
    //     self.0.it
    // }
}

fn part_one(file_name: &str) -> i32 {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));
    let mut f: Forrest = Forrest::default();
    for line in data.lines() {
        f.push(line.to_string());
    }

    let f_clone = f.clone();

    for (y, line) in data.lines().enumerate() {
        let mut top = 0;
        for (x, tree) in line.chars().enumerate() {
            let tree = tree.to_digit(10).expect("should be digit");
            if tree > top {
                let top = tree;
                f_clone.0.iter().nth(y).expect("y out of bounds").chars().nth(x)
            }
        }
    }

    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), 21);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two("dummy-data.txt"), todo!);
    // }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_one("data.txt"));

    Ok(())
}
