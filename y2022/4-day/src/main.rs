use std::{fs, path::Path};
use itertools::Itertools;
use color_eyre::eyre::Result;

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

fn part_1() {
    let data = read_data(Path::new("./data.txt"));
    let fully_containded_pairs = 0;

    for line in data.lines() {
        let (str_range_a, str_range_b) = line.split(',').collect_tuple().unwrap();

        let (s0, e0) = str_range_a.split('-').collect_tuple::<(&str, &str)>().unwrap();
        let (s1, e1) = str_range_b.split('-').collect_tuple::<(&str, &str)>().unwrap();

        let range_a = s0.parse::<i32>()..s1.parse::<i32>();
        let range_b = s0.parse::<i32>()..s1.parse::<i32>();



        // dbg!(range_a, range_b);
        panic!("MANUAL PANIC")
    }

}

pub fn main() -> Result<()> {
    color_eyre::install()?;
    part_1();

    Ok(())
}
