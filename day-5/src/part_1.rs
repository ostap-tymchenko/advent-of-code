use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//         [H]     [W] [B]            
//     [D] [B]     [L] [G] [N]        
// [P] [J] [T]     [M] [R] [D]        
// [V] [F] [V]     [F] [Z] [B]     [C]
// [Z] [V] [S]     [G] [H] [C] [Q] [R]
// [W] [W] [L] [J] [B] [V] [P] [B] [Z]
// [D] [S] [M] [S] [Z] [W] [J] [T] [G]
// [T] [L] [Z] [R] [C] [Q] [V] [P] [H]
//  1   2   3   4   5   6   7   8   9 


fn open_file(path: &Path) -> String {
    // Create a path to the desired file
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!(""),
    }

    data
}

pub fn main() {
    let data_path = Path::new("src/dummy-data.txt");
    let data = open_file(data_path);

    let chart_path = Path::new("src/chart.txt");
    let chart = open_file(chart_path);

    let mut chart_vec = vec![];

    for line in chart.lines() {
        chart_vec.push(line)
    }

    // later replace with let mut chart_vec = vec![chart];
    
    for line in data.lines() {
        let move_times:i32 = line.chars().nth(0).unwrap().to_string().parse().unwrap();
        let crate_from:usize = line.chars().nth(1).unwrap().to_string().parse().unwrap();
        let crate_from = crate_from - 1;
        let crate_to:  i32 = line.chars().nth(2).unwrap().to_string().parse().unwrap();

        println!("line {line:?} asks to move {move_times:?} crates from {crate_from:?} to {crate_to:?}");

        for _ in 0..move_times {
            // the move buffer is a copy of the crate that will be moved
            let move_buffer = chart_vec[crate_from].chars().last();
            chart_vec[crate_from].to_string().pop();
            println!("{move_buffer:?}");
        } 
    }
}
