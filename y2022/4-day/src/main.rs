use color_eyre::eyre::Result;
use itertools::Itertools;
use std::{fs, path::Path};

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(data_path).expect("data parse fail")
}

fn part_1() -> i32 {
    let data = read_data(Path::new("./dummy-data.txt"));
    // let data = read_data(Path::new("./dummy-data.txt"));
    let mut fully_containded_pairs = 0;

    for line in data.lines() {
        if !line.is_empty() {
            let (str_range_a, str_range_b) = line.split(',').collect_tuple().unwrap();

            let (start1, end1) = str_range_a
                .split('-')
                .collect_tuple::<(&str, &str)>()
                .unwrap();

            let (start2, end2) = str_range_b
                .split('-')
                .collect_tuple::<(&str, &str)>()
                .unwrap();

            let range1 = start1..end1;
            let range2 = start2..end2;

            if (range1.start <= range2.start && range1.end >= range2.end) || (range2.start <= range1.start && range2.end >= range1.end) {
                println!("matching pattern: {line}");
                fully_containded_pairs += 1;
            }
        }
    }
    fully_containded_pairs
}


pub fn main()  -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_1());

    Ok(())
}
