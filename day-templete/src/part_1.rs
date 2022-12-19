use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use colored::*;

fn open_file() -> String {
    // Create a path to the desired file
    let path = Path::new("src/data.txt");
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
        Ok(_) => print!("{} contains: data\n", display),
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
    let file = open_file();

    for line in file.lines() {
    
    }
}        
