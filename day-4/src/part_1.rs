use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    let mut totally_overlapping_sets = 0;

    for line in file.lines() {
        let mut first_start = "";
        let mut first_end = "";
        let mut second_start = "";
        let mut second_end = "";
        
        let mut half_iterator = 0;
        for half in line.split(",") {
            half_iterator += 1;
            // dbg!(half, half_iterator);
            let mut quart_iterator = 0;

            for quart in half.split("-") {
                quart_iterator += 1; 
                // dbg!(quart, quart_iterator);

                // setting the start and end boundries of the first half
                if half_iterator == 1 {
                    if quart_iterator == 1 {
                        first_start = quart;
                    } else if quart_iterator == 2 {
                        first_end = quart;
                    } else {
                        unreachable!();
                    }
                } else if half_iterator == 2 {
                     if quart_iterator == 1 {
                        second_start = quart;
                    } else if quart_iterator == 2 {
                        second_end = quart;
                    } else {
                        unreachable!();
                    } 
                } else {
                    unreachable!();
                }
            }
        }

        dbg!(line);
        dbg!(first_start, first_end, second_start, second_end);

        if first_start <= second_start {
            if first_end >= second_end {
                println!("totally overlapping set");
                totally_overlapping_sets += 1;
            }
        } else if first_start >= second_start {
            if first_end <= second_end {
                println!("totally overlapping set");
                totally_overlapping_sets += 1;
            }
        } else {
            // not overlapping
        }
    }

    dbg!(totally_overlapping_sets);
}
