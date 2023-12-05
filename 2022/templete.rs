use color_eyre::eyre::Result;
use std::any::type_name;
use std::path::Path;
use std::fs;

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

    todo!()
}

// fn part_two(data: String) -> i32 {
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(read_data_from_name("dummy-data.txt")), TODO);
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
