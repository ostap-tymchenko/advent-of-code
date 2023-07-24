use std::fs;
use std::path::Path;

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data read fail")
}

pub fn main() {
    let data = read_data(Path::new("./data.txt"));
    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;

    let mut buff: i32 = 0;

    for line in data.lines() {
        if line.is_empty() {
            if buff > top1 {
                top3 = top2;
                top2 = top1;
                top1 = buff;
            } else if buff > top2 {
                top3 = top2;
                top2 = buff;
            } else if buff > top3 {
                top3 = buff;
            }

            buff = 0;
        } else {
            buff = buff + line.parse::<i32>().expect("not number");
        }
    }
    println!("Part one solution: {}", top1);
    println!("Part two solution: {}", top1 + top2 + top3);
}
