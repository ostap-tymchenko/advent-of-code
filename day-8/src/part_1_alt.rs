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

    // let right_reconstruction = filter_from_right(&forrest); 
    // let left_reconstruction = filter_from_left(&forrest); 
    // let top_reconstruction = flip_right(&filter_from_right(&flip_right(&forrest)));
    let bottom_reconstruction = flip_left(&filter_from_right(&flip_right(&forrest)));

    let flip_right_test = flip_right(&forrest);
    let flip_right_og_test = flip_right(&forrest);

    // display_forrest(&right_reconstruction, "right_reconstruction");
    // display_forrest(&left_reconstruction, "left_reconstruction");
    // display_forrest(&top_reconstruction, "top_reconstruction_alt");
    display_forrest(&bottom_reconstruction, "bottom_reconstruction");

    display_forrest(&flip_right(&forrest), "flip_right");
    display_forrest(&flip_left(&forrest), "flip_left"); 
}

fn filter_from_right(forrest: &Vec<String>) -> Vec<String> {
    let mut output = Vec::<String>::new(); 
    let mut output_buffer = String::new();
    for (row_iter, row) in forrest.iter().enumerate() {
        let mut top_tree_hight = 0;
        for tree in row.chars() {
            let tree = tree.to_digit(10).unwrap();
            if tree >= top_tree_hight {
                top_tree_hight = tree;
                output_buffer.push('x');
            } else {
                output_buffer.push_str(tree.to_string().as_str());
            }
        }
        output_buffer.push_str("\n");
    }
    for line in output_buffer.lines() {
        output.push(line.to_string());
    }
    output
}

fn filter_from_left(forrest: &Vec<String>) -> Vec<String> {
    let mut output = Vec::<String>::new(); 
    let mut output_buffer = String::new();
    for (row_iter, row) in forrest.iter().enumerate() {
        let mut top_tree_hight = 0;
        for tree in row.chars().rev() {
            let tree = tree.to_digit(10).unwrap();
            if tree >= top_tree_hight {
                top_tree_hight = tree;
                output_buffer.push('x');
            } else {
                output_buffer.push_str(tree.to_string().as_str());
            }
        }
        output_buffer.push_str("\n");
    }
    for line in output_buffer.lines() {
        output.push(line.chars().rev().collect::<String>())
    }
    output
}

fn flip_right(forrest: &Vec<String>) -> Vec<String> {
    let mut flipped_buffer = vec![String::new(); forrest[0].len()]; 
    for row in forrest.iter() {
        for (tree_iter, _) in row.chars().enumerate() {
            flipped_buffer[tree_iter].push(row.chars().nth(tree_iter).unwrap_or('F'));
        } 
    }

    // let mut flipped = Vec::<String>::new();
    // for line in flipped_buffer.iter().rev() {
    //     flipped.push(line.to_string());
    // }
    flipped_buffer
}

fn flip_left(forrest: &Vec<String>) -> Vec<String> {
    let mut flipped = vec![String::new(); forrest[0].len()]; 
    for row in forrest.iter() {
        for (tree_iter, _) in row.chars().enumerate() {
            flipped[tree_iter].push(row.chars().rev().nth(tree_iter).unwrap_or('F'));
        } 
    }
    flipped
}

fn display_forrest(forrest_reconstruction: &Vec<String>, name: &str) {
    println!("\n{name}");
    for row in forrest_reconstruction {
        println!("{row}");
    }
} 
