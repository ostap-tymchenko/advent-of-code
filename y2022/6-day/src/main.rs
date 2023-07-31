use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn read_data(path: &str) -> String {
    dbg!("triying to read", {&path});
    fs::read_to_string(&path).expect("data parse fail")
}

fn part_one(file_name: &str) -> usize {
    let path = "data/".to_owned() + file_name;
    let data = read_data(&path);

    let mut buff = vec![];

    for (index, c) in data.chars().enumerate() {
        buff.push(c);
        buff.remove(0);

        if !buff.contains(&c) {
            let seen: HashSet<char> = buff.clone().into_iter().collect();
            if seen.len() == 4 {
                return index;
            }
        }
    }

    panic!("input should have 4 uniuqe chars next to each other at some point");
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
