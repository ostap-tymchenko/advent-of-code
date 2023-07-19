use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use itertools::Itertools;

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

    let mut lines = file.lines();
    let mut total_priority = 0;


    for (a, b, c) in lines.tuples() {
        let b_btreeset = BTreeSet::from_iter(b.chars());
        let c_btreeset = BTreeSet::from_iter(c.chars());

        let mut a_b_common = "".to_string();
        let mut a_b_c_common = "".to_string();

        for char in a.chars() {
            if b_btreeset.contains(&char) {
                a_b_common.push(char); 
            }
        }

        for char in a_b_common.chars() {
            if c_btreeset.contains(&char) {
                a_b_c_common.push(char);
            }
        }

        let a_b_c_common = a_b_c_common.chars().next().unwrap();
        dbg!(&a_b_c_common);

        let priority_buffer = alphabet.find(*&a_b_c_common);

        let priority_buffer = match priority_buffer {
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
