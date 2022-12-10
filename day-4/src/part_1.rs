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
    let mut totally_overlapping_sets = 0;

    for line in file.lines() {
        let mut start1 = "";
        let mut end1 = "";
        let mut start2 = "";
        let mut end2 = "";
        
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
                        start1 = quart;
                    } else if quart_iterator == 2 {
                        end1 = quart;
                    } else {
                        unreachable!();
                    }
                } else if half_iterator == 2 {
                     if quart_iterator == 1 {
                        start2 = quart;
                    } else if quart_iterator == 2 {
                        end2 = quart;
                    } else {
                        unreachable!();
                    } 
                } else {
                    unreachable!();
                }
            }
        }

        dbg!(line);
        dbg!(start1, end1, start2, end2);

         
        if (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1) {
            println!("true");
            totally_overlapping_sets += 1
        } else {
            println!("false")
        }
    }

    dbg!(totally_overlapping_sets);
}
