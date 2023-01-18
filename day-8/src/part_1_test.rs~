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
    let mut forrest: Vec<String> = Vec::new();

    println!("forrest:");
    for line in file.lines() {
        forrest.push(line.parse().unwrap());
        println!("{line}");
    }

    let right_reconstruction = filter(&forrest, true);
    let left_reconstruction = filter(&forrest, false);
    // the filter function takes a forrest and  a direction, and filters. it can only go left and right.

    println!("\nright_reconstruction:");
    for row in right_reconstruction {
        println!("{row}");
    }

    println!("\nleft_reconstruction:");
    for row in left_reconstruction {
        println!("{row}");
    }
}

fn filter(forrest: &Vec<String>, from_right: bool) -> Vec<String> {

    let mut output = Vec::<String>::new(); 
    let mut output_buffer = String::new();
    for row in forrest.iter() {
        let mut top_tree_hight = 0;
        if from_right {
            for tree in row.chars() {
                let tree = tree.to_digit(10).unwrap();
                if tree >= top_tree_hight {
                    top_tree_hight = tree;
                    output_buffer.push('x');
                } else {
                    output_buffer.push_str(tree.to_string().as_str());
                }
            }
        } else {
            for tree in row.chars().rev() {
                let tree = tree.to_digit(10).unwrap();
                if tree >= top_tree_hight {
                    top_tree_hight = tree;
                    output_buffer.push('x');
                } else {
                    output_buffer.push_str(tree.to_string().as_str());
                }
            }
        }
        output_buffer.push_str("\n");
    }

    if from_right {
        for line in output_buffer.lines() {
            output.push(line.to_string());
        }
    } else {
        for line in output_buffer.lines() {
            output.push(line.chars().rev().collect::<String>())
            }
        }
    
output
}
