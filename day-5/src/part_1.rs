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
//
// 1   2   3   4   5   6   7   8   9 


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
    let data_path = Path::new("src/data.txt");
    let data = open_file(data_path);

    let chart_path = Path::new("src/chart.txt");
    let chart = open_file(chart_path);

    // println!("{chart}");
    for line in chart.lines() {
        if line == "" {
            // nothing
        } else {
            println!("{line}")
        }
    }
    println!();

    let mut chart_vec = vec![];

    for line in chart.lines() {
        chart_vec.push(line.to_string())
    }

    for line in data.lines() {
        if line != "" {

            let i: Vec<&str> = line.split("-").collect();

            let mut move_times:usize = i[0].parse().unwrap();
            let crate_from:usize = i[1].parse().unwrap();
            let crate_to:usize = i[2].parse().unwrap();

            // these 4 lines make sure that there are enough crates to move, and prevents a crash later on if there arent.
            let mut old_move_times = move_times; 
            if move_times > chart_vec[crate_from].len() {
                move_times = chart_vec[crate_from].len() 
            } 

            if move_times != old_move_times {
                println!("line {line:?} asks to move {old_move_times:?} crates to be moved but {move_times:?} crates will be moved from {crate_from:?} to {crate_to:?}");
            } else {
                println!("line {line:?} asks to move {move_times:?} crates from {crate_from:?} to {crate_to:?}");
            }


            for _ in 0..move_times {
                // we save the var were about to remove to a buffer because this is a move not delete operation
                let move_buffer = &chart_vec[crate_from].chars().last();
                // the last crate is removed
                if let Some(x) = chart_vec.get_mut(crate_from) {x.pop();}
                // the move buffer than becomes is than saved to where it is moved to
                if let Some(x) = chart_vec.get_mut(crate_to) {x.push(move_buffer.unwrap());}
                // chart_vec.get_mut(crate_to).unwrap().push(move_buffer.unwrap_or_default());
                
                println!("moved crate = {move_buffer:?} from {crate_from:?} to {crate_to:?}");
            }

            let chart_vec_crate_from = &chart_vec[crate_from].to_uppercase();
            let chart_vec_crate_to = &chart_vec[crate_to].to_uppercase();

            if chart_vec_crate_from == "" {
                println!("line (removed from) now contains nothing")
            } else {
                println!("line (removed from) is now {chart_vec_crate_from}");
            }

            println!("line (added to) is now {chart_vec_crate_to}");
        } 
    }

    let mut top = "".to_string();

    for stack in &chart_vec {
        if stack != "" {
            if stack == " " {
                top.push('|');
            } else {
                top.push(stack.chars().last().unwrap_or('/'));
            }
        }
    }

    let top = top.replace("/", "");
    let top = top.replace("|", " ");
    let top = top.to_uppercase();

    print!("\nthe final crate chart looks like:");
    let mut index = 0;
    for line in chart_vec {
        index += 1;
        let line = line.to_uppercase();
        print!("{index} ");
        for char in line.chars() {
            print!("[{char}]");
        }
        println!();
    }

    println!("\nthe top crates are:\n{top}");
}
