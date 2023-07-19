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
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains: s", display),
    }

    s
}

pub fn main() {
    let file = open_file();
    // println!("{file}");
    
    let mut top = 0;
    let mut second = 0;
    let mut third = 0;


    for found in file.split("\n\n") {
        // dbg!(found);
        let mut buffer = 0;
        for induvidual_str in found.split("\n"){
            let induvidual_int = match induvidual_str.parse::<i32>() {
                Ok(i) => i,
                Err(_e) => -1,
            }; 

            buffer = buffer + induvidual_int;
            // dbg!(buffer);
        }

        if buffer > top {
            third = second;
            second = top;
            top = buffer;
        
        } else if buffer > second {
            third = second;
            second = buffer;
        

        } else if buffer > third {
            third = buffer;
        }

        else {
            //nothing new
        }
    }
    println!("top: {top}, second:{second}, third:{third}");
    let top_three = top + second + third;
    println!("top three together: {top_three}");
}
