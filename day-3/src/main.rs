use std::collections::HashSet;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::Chars;

fn open_file() -> String {
    // Create a path to the desired file
    let path = Path::new("src/dummy-data.txt");
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

    for lines in file.lines() {
        let len = lines.len();
        let first = &lines[0..len / 2];
        let second = &lines[len /2 ..];

        let first_btreeset = BTreeSet::from_iter(first.chars());
        let second_btreeset = BTreeSet::from_iter(second.chars());

        let mut common_char = '_';

        for char in first.chars() {
            if second_btreeset.contains(&char) {
                common_char = char;
            } 
        }

        let priority = alphabet.find(common_char);
        // let priority = priority + 1;
        dbg!(priority);
    }   
}
