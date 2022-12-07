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

    
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Priority {
    charachter: char,
    priority: i32,
    }

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(); 

    let mut priorities = HashSet::new();
    
    let mut iterator = 0;
    for iterchars in alphabet.chars() {
        iterator += 1;
        priorities.insert(Priority {charachter: iterchars, priority: iterator});
    }

    dbg!(&priorities);

    // dbg!(priorities);

    for lines in file.lines() {
        let len = lines.len();
        let first = &lines[0..len / 2];
        let second = &lines[len /2 ..];

        let first = BTreeSet::from_iter(first.chars());
        let second = BTreeSet::from_iter(second.chars());

        let shared_char = (&first) & (&second);

        
    }   
}
