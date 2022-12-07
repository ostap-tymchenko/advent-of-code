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
        Ok(_) => print!("{} contains: data", display),
    }

    data
}

fn main() {
    let file = open_file();
    println!("{file}");

    let mut points = 0;

    for line in file.split("\n") {
        dbg!(line);
        let (opponent, player) = line.split_at(1);
        // dbg!(opponent, player);

        if player == "X" {
            points += 1;
        } else if player == "Y" {
            points += 2;
        } else if player == "Z" {
            points += 3;
        } else {
            panic!("bad logic / input");
        }

        if player == "X" {
            if opponent == "A" {
                //draw condition
                points += 3;
            } else if opponent == "B" {
                // loose condition
            } else if opponent == "C" {
                // win condition
                points +=  6;
            } else {
                panic!("bad logic / input");
            }

        } else if player == "Y" {
            if opponent == "A" {
                // win condition
                points += 6;
            } else if opponent == "B" {
                // draw condition
                points += 3;
            } else if opponent == "C" {
                // loose condition
            } else {
                panic!("bad logic / input")
            }

        } else if player == "Z" {
            if opponent == "A" {
                // lose condition
            } else if opponent == "B" {
                // win condition
                points += 6;
            } else if opponent == "C" {
                // draw condition
                points +=3;
            } else {
                panic!("bad logic / input")
            }
        }
     
    } // for line in ...
    
} //main
