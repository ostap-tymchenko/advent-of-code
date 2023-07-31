use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashMap;
use std::fs;
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

fn go_to_target(path: String, target: &str) -> String {
    if target == "/" {
        return target.to_string();
    } else if target == ".." {
        println!("input:'{}' '{}'", path, target);
        // return path.rsplit(FOLDER_SPLIT_CHAR).collect::<Vec<&str>>()[2].to_string();
        return path.rsplit_once(FOLDER_SPLIT_CHAR).expect("when goto .. should have at least 2 / to remove").0.rsplit_once(FOLDER_SPLIT_CHAR).unwrap_or(("/", "DONT USE ME")).0.to_string();
    } else {
        return path.to_owned() +  target + &FOLDER_SPLIT_STR;
    }
}

fn get_top_lvl_dir(path: &str ) -> &str {
    path.split_once(FOLDER_SPLIT_CHAR).expect("get_top_lvl_dir should never be called on root folder. Call it in a dir bellow root").0
}

fn part_one(file_name: &str) -> i32 {
    let data_path = "data/".to_owned() + file_name;
    let data = read_data(Path::new(&data_path));

    let mut sim_path = String::from("/");
    let mut top_lvl_dir_size: HashMap<&str, i32> = HashMap::new();

    for line in data.lines() {
        let fw = line.split_at(1); // get the first word
        let fc = fw.0; // get the first char
        if fc == "$" {
            // if fw.1.split_whitespace().next().expect("line with command should have whitespace") == "cd" {
            let first_command_c = line.chars().nth(2).expect("line with command should have at least 3 chars");
            if first_command_c == 'c' {
                print!("goto from '{sim_path}' with target '{}'", line.split_whitespace().nth(2).unwrap());
                let target = line.split_whitespace().nth(2).expect("line with command should have at least 3 words");
                sim_path = go_to_target(sim_path, target);
                println!(" new path:'{sim_path}'")
            } else if first_command_c == 'l' {
                // println!("ls.. found but done use me!");
            } else {
                panic!("line with command should have at least 3 chars");
            }
        } else if fc == "d" {
            // shows dir name
            // should never be used
        } else {
            // shows files and sized
            let file_size = line.split_once(WHITESPACE).expect("file line should have space").0.parse::<i32>().expect("file size should be parsable");
            if file_size <= 100_000 {
                top_lvl_dir_size.insert(get_top_lvl_dir(&sim_path), file_size);
            }
        }
    }
    0
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
