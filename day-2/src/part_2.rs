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

    let mut points = 0;

    for line in file.lines() {
        dbg!(line);
        let (opponent, result) = line.split_at(1);
        let opponent = opponent.trim();
        let result = result.trim();
        dbg!(opponent, result);
        
        if result == "X" {
            println!("result = loose");
        } else if result == "Y" {
            println!("result = draw");
        } else if result == "Z"{
            println!("result = win");
        } else {
            unimplemented!();
        }

        if opponent == "A" {
            if result == "X" {
                // scissors play
                println!("scissors play");
                points += 3;
            } else if result == "Y" {
                // rock play
                println!("rock play");
                points += 1;
            } else if result == "Z" {
                // paper play
                println!("paper play");
                points += 2;
            } else {
                unreachable!()
            }
        } else if opponent == "B" {
            if result == "X" {
                // rock play
                println!("rock play");
                points +=1;
            } else if result == "Y" {
                // paper play
                println!("paper play");
                points +=2;
            } else if result == "Z" {
                // scissors play
                println!("scissors play");
                points +=3;
            } else {
                unreachable!()
            }
        } else if opponent == "C" {
            if result == "X" {
                // paper play
                println!("paper play");
                points +=2;
            } else if result == "Y" {
                // scissors play
                println!("scissors play");
                points +=3;
            } else if result == "Z" {
                // rock play
                println!("rock play");
                points +=1;
            } else {
                unreachable!()
            }
        } else {
            unreachable!();
        }


        if result == "X" {
            // loose condition
        } else if result == "Y" {
            // draw condition
            points += 3;
        } else if result == "Z" {
            // win condition
            points += 6;
        } else {
            unreachable!();
        }

    }
    println!("{points}")
    
} //main
