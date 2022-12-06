use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn open_file() -> String {
    // Create a path to the desired file
    let path = Path::new("src/ac1_data.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    s
}

pub fn main() {
    let file = open_file();
    // println!("{file}");

   

    for found in file.split("\n\n") {
        dbg!(found);
        let mut buffer = 0;
        for induvidual_str in found.split("\n"){
            let induvidual_int: i32 = induvidual_str.parse().unwrap();
            buffer = buffer + induvidual_int;
            dbg!(buffer);
        }
    }
}
