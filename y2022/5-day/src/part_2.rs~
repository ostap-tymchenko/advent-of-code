use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use colored::*;

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

fn dbg_print(severity: &str, messege: &str, from: &str) {
    if severity == "warn" {
        println!("{}", format!("{messege} called by {from}").red());
    } else if severity == "debug" {
        println!("{}", format!("{messege} called by {from}"));
    } else if severity == "result"{
        println!("{}", format!("{messege} called by {from}"));
    } else {
        println!("{}", format!("malformed debug: severity: {severity}, messege: {messege}, from: {from}").purple());
    }
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

    let chart_vec_old = &chart_vec.to_owned();

    for line in data.lines() {
        if line != "" {
            

            let i: Vec<&str> = line.split("-").collect();

            let mut move_times:usize = i[0].parse().unwrap();
            let crate_from:usize = i[1].parse().unwrap();
            let crate_to:usize = i[2].parse().unwrap();
            
            let crate_from = crate_from -1;
            let crate_to = crate_to -1;

            // these 4 lines make sure that there are enough crates to move, and prevents a crash later on if there arent.
            let old_move_times = move_times; 
            if move_times > chart_vec[crate_from].len() {
                move_times = chart_vec[crate_from].len() 
            } 

            if move_times != old_move_times {
                println!("line {line:?} asks to move {old_move_times:?} crates to be moved but {move_times:?} crates will be moved from {crate_from:?} to {crate_to:?}");
            } else {
                println!("line {line:?} asks to move {move_times:?} crates from {crate_from:?} to {crate_to:?}");
            }





            // CORELOGIC


            // prereq
            let move_line = &mut chart_vec[crate_from];
            let mut move_buffer = "".to_string();
            
            // create buffer and remove crates
            for _ in 0..move_times {
                move_buffer += move_line.pop().unwrap_or('/').to_string().as_str();
            } 

            // reversing the buffer
            let move_buffer: String = move_buffer.chars().rev().collect();

            // debug
            dbg_print("debug", format!("move buffer = [{move_buffer}]").as_str(), "CORELOGIC");

            // adding crates
            if let Some(x) = chart_vec.get_mut(crate_to) {x.push_str(&move_buffer);}

            // prereq
            let chart_vec_crate_from = &chart_vec[crate_from].to_uppercase();
            let chart_vec_crate_to = &chart_vec[crate_to].to_uppercase();

            // debug
            if chart_vec_crate_from == "" {
                println!("line (removed from) now contains nothing")
            } else {
                println!("line (removed from) is now {chart_vec_crate_from}");
            }
            println!("line (added to) is now {chart_vec_crate_to}");


            // CORELOGIC





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

    print!("\nthe old crate chart looks like:\n");
    println!("   1  2  3  4  5  6  7  8  9");

    let mut index = 0;
    for line in chart_vec_old {
        index += 1;
        let line = line.to_uppercase();
        print!("{index} ");
        for char in line.chars() {
            print!("[{char}]");
        }
        println!();
    }

    let mut num_crates_one = 0;
    for stack in chart_vec_old {
        for _ in stack.chars() {
            num_crates_one += 1;
        }
    }

    print!("\nthe final crate chart looks like:\n");
    println!("   1  2  3  4  5  6  7  8  9");

    let mut index = 0;
    for line in &chart_vec {
        index += 1;
        let line = line.to_uppercase();
        print!("{index} ");
        for char in line.chars() {
            print!("[{char}]");
        }
        println!();
    }

    let mut num_crates_two = 0;
    for stack in &chart_vec {
        for _ in stack.chars() {
            num_crates_two += 1;
        }
    }

    if num_crates_one == num_crates_two {
        println!("{}", format!("\nthe number of crates is the same ✅ ({num_crates_one})").green())
    } else {
        let error = "NOT the same ❎";
        println!("\nthe number of crates is {} ({}, {})", error.red() , {num_crates_one}, {num_crates_two})
    }

    // let string_alphabet = "abcdefghijklmnopqrstuvwxyz";
    // let string_alphabet = string_alphabet.to_owned() + &string_alphabet.to_uppercase();
    // dbg_print("debug", &format!("string_alphabet = {string_alphabet}"), "windown-debuging");
    //
    //
    // let mut old_contains = "".to_string();
    // for stack in chart_vec_old {
    //     for var_crate in stack.chars() {
    //         if string_alphabet.contains(var_crate) {
    //             old_contains += &var_crate.to_string();
    //         }
    //     } 
    // }
    //
    // println!("chart_vec_old = {chart_vec_old:?}");
    //
    // let mut do_letters_match = true;
    // let mut false_called_on = "".to_string();
    //
    // for stack in chart_vec_old {
    //     for char in stack.chars() {
    //         if chart_vec.contains(&char.to_string()) {
    //             //nothing            
    //         } else {
    //             do_letters_match = false;
    //             false_called_on += &char.to_string();
    //         }
    //     }
    // } 
    //
    // if do_letters_match == false {
    //     dbg_print("warn", "do_letters_match = false", "windown-debuging");
    //     println!("not matching = {false_called_on}");
    // }

    dbg_print("result", format!("the top crates are: {top}").as_str(), "final-result");
}
