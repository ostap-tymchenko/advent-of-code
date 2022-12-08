use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

pub fn main() {
    let file = open_file();
    println!("{file}");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"; 

    let mut total_priority = 0;
    for lines in file.lines() {
        // splits og string in two, creating first and second
        let len = lines.len();
        let first = &lines[0..len / 2];
        let second = &lines[len /2 ..];

        // turns first and second into a btreeset
        let second_btreeset = BTreeSet::from_iter(second.chars());

        // defines the common charachter between first and second
        let mut common_char = '_';
        for char in first.chars() {
            if second_btreeset.contains(&char) {
                common_char = char;
            } 
        }

        let priority_buffer= alphabet.find(common_char);
        // let priority_buffer= priority_buffer+ 1;

        let priority_buffer= match priority_buffer{
            Some(val) => val,
            None => {
                panic!("panic! on a `None` value");
            }, 
        };
        let priority_buffer = priority_buffer+ 1;
        total_priority += priority_buffer;
    }   

    dbg!(total_priority);
}
