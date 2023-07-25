use std::{fs, path::Path};
use itertools::Itertools;

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

fn part_1() {
    let data = read_data(Path::new("./data.txt"));
    for line in data.lines() {
        let ranges: (&str, &str) = line.split(',').into_iter().next_tuple().unwrap()
            // for tomorow: potentially merge line 11 and 14. also auto turn to i32?
        // let generated_iter = bounds.split('-').into_iter().next_tuple().unwrap();
        // let range_1: (i32, i32) = ranges.0.split('-').into_iter().next_tuple().unwrap()
        dbg!(range_1);

        panic!()
    }

}

pub fn main () {
    part_1()
}
