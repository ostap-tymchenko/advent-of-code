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

fn part_one(file_name: &str) -> todo! {
    let data = read_data(Path::new("data/data.txt"));
    // let data = read_data(Path::new("./dummy-data.txt"));
}

fn part_two(file_name: &str) -> todo! {
    let data = read_data(Path::new("data/data.txt"));
    // let data = read_data(Path::new("./dummy-data.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), todo!);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("dummy-data.txt"), todo!);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    part_one();

    Ok(())
}
