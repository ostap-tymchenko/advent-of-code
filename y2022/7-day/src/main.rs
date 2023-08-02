use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashMap;
use std::{fs, fmt};
use std::path::Path;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

const FOLDER_SPLIT_STR: &str = "/";
const FOLDER_SPLIT_CHAR: char = '/';
const WHITESPACE: char = ' ';
const ROOT: &str = "/";
const ROOT_WORD: &str = "ROOT";

#[derive(Clone, Default, Debug)]
struct StackPath(Vec<String>);

impl StackPath {
    fn push(&mut self, s: String) {
        self.0.push(s);
    }

    fn pop(&mut self) -> String {
        self.0.pop().expect("stack should not be empty")
    }
}

impl fmt::Display for StackPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[/]")?;
        for s in self.0.iter() {
            write!(f, "[{s}]")?
        }

        Ok(())
    }
}

fn go_to_target(mut path: StackPath, target: String) -> StackPath {
    if target == ROOT {
        return StackPath::default();
    } else if target == ".." {
        StackPath(vec![path.pop()])
    } else {
        // target is file
        path.push(target);
        path
    }
}

fn stringify_path(sim_path: &StackPath) -> String {
    sim_path.0.iter().map(|s| s.to_owned() + FOLDER_SPLIT_STR).collect()
}

fn add_my_size_to_all_my_parents(sim_path: &StackPath,file_name: &str, file_size: i32, dir_size_index: &mut HashMap<String, i32>) {
    let location_of_me = stringify_path(&sim_path);


    // println!("\nloop start [");
    // println!("added file {file_name}, of size {file_size} to [ROOT] bc path is {sim_path}");
    if let Some(root_folder_size) = dir_size_index.get_mut(ROOT_WORD) {
        *root_folder_size += file_size;
        // println!("via adding");
    } else {
        // the root folder is imaginary, and not found in the real path
        // because of this we add it seperatly here instead.
        dir_size_index.insert(ROOT_WORD.to_string(), file_size);
        // println!("via creating");
    }

    for dir in &sim_path.0 {
        // println!("added file {file_name}, of size {file_size} to folder [{dir}] bc path is {sim_path}");
        if let Some(existing_size) = dir_size_index.get_mut(&location_of_me) {
            *existing_size += file_size;
            // println!("via adding");
        } else {
            dir_size_index.insert(location_of_me.clone(), file_size);
            // println!("via creating");
        }
    }
    // println!("\n] loop end");
}

fn part_one(file_name: &str) -> i32 {
    let data_path = "data/".to_owned() + file_name;
    let data = read_data(Path::new(&data_path));

    let mut sim_path = StackPath::default();
    let mut dir_size_index: HashMap<String, i32> = HashMap::new();

    for line in data.lines() {
        let fc = line.chars().nth(0).expect("line should have at least one char");

        if fc == '$' {
            // comman0d
            let sc = line.chars().nth(2).expect("line command should have at least two chars");
            if sc == 'c' {
                // command starting with c is cd
                // print!("goto from '{}' with target '{}'", sim_path.clone(), line.split_whitespace().nth(2).unwrap());
                let target = line.split_whitespace().nth(2).expect("line with command should have at least 3 words").to_string();
                sim_path = go_to_target(sim_path, target);
                // println!(" new path:'{sim_path:?}'");

            } else if sc == 'l' {
                // ls
            } else {
                // println!("line: {line}");
                panic!("line with command should have at least 3 chars");
            }
        } else if fc == 'd' {
            // dir name
        } else {
            // file
            let split = line.split_once(WHITESPACE).expect("file line should have a space");

            let file_size = split.0.parse::<i32>().expect("file size should be parsable");
            let file_name = split.1;

            add_my_size_to_all_my_parents(&sim_path, file_name, file_size, &mut dir_size_index);
        }
    }

    let mut total = 0;
    // dbg!(&dir_size_index);
    for folder_size in dir_size_index.values() {
        if folder_size <= &100_000{
            total += folder_size 
        }
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), 95437);
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
