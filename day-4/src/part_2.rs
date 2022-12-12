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
    let mut overlapping_sets = 0;

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

        println!("{line}");
        // dbg!(start1, end1, start2, end2);

        let (start1, start2, end1, end2) = (start1.parse::<i32>().unwrap(), start2.parse::<i32>().unwrap(), end1.parse::<i32>().unwrap(), end2.parse::<i32>().unwrap());


        let range1 = start1..end1;
        let range2 = start2..end2;
        dbg!(&range1, &range2);

        if range1.end >= range2.start && range1.start <= range2.end {
            overlapping_sets += 1;
            println!("true")
        } else {
            println!("false")
        }
    }
    dbg!(overlapping_sets);
}
