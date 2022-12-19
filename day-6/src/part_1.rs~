use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use colored::*;
use std::collections::HashSet;

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

// GOAL: last 4 chars need to all be different

fn are_chars_different(s: &str) -> bool {
    s.chars().collect::<Vec<_>>().into_iter().all(|c| s.chars().filter(|x| x == &c).count() == 1)
}

pub fn main() {
    let data_path = Path::new("src/data.txt");
    let data = open_file(data_path);

    println!("data = {data}");

    let mut buffer = "".to_string();
    let mut index = 0;
    let mut awnser = 0;

    for char in data.chars() {
        index += 1;
        buffer += &char.to_string();

        if are_chars_different(&buffer) && awnser == 0 && index > 3 {
            awnser = index;
            println!("awnser reached at index ={index} with buffer ={buffer}, with char ={char}");
        }

        if index > 3 {
            buffer.remove(0);
        }
    } 

    println!("awnser is {awnser}");
}        
