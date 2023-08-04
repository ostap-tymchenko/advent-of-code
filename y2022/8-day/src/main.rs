// use color_eyre::eyre::Result;
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

fn part_one(file_name: &str) -> i32 {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));

    todo!()
}

// fn part_two(file_name: &str) -> todo! {
//     let data = read_data(Path::new("data/data.txt"));
//     // let data = read_data(Path::new("./dummy-data.txt"));
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), TODO);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two("dummy-data.txt"), todo!);
    // }
}

fn main() {
    // color_eyre::install()?;
    println!("{part_one}");

}
