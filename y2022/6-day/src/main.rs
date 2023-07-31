use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

fn read_data(path: &str) -> String {
    fs::read_to_string(&path).expect("data parse fail")
}

fn part_one(file_name: &str) -> usize {
    let path = "data/".to_owned() + file_name;
    let data = read_data(&path);

    let mut recently_seen = String::from("");

    for (index, c) in data.chars().enumerate() {
        recently_seen.push(c);
        if recently_seen.len() > 4 {
            recently_seen.remove(0);
        }

        let seen: HashSet<char> = HashSet::from_iter(recently_seen.chars());
        if seen.len() == 4 {
            return index + 1;
        }
    } 
    panic!("inp data should have 4 consequitive uniuqe chars")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), 7);
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
