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

// GOAL: last 4 chars need to all be different


fn has_repeated_chars(s: &str) -> bool {
    // Convert the string to a character iterator
    let chars = s.chars();

    // Use a hash map to count the number of occurrences of each character
    let mut char_counts = std::collections::HashMap::new();
    for c in chars {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }

    // Check if any of the counts are greater than 1
    for (_, count) in char_counts {
        if count > 1 {
            return true;
        }
    }

    false
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

        if !has_repeated_chars(&buffer) && awnser == 0 && index > 13 {
            awnser = index;
            println!("awnser reached at index ={index} with buffer ={buffer}");
        }

        if index > 13 {
            buffer.remove(0);
        }
    } 

    println!("awnser is {awnser}");
}        
