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

// GOAL:
// how many trees are visible from outside the grid?
//
//A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

pub fn main() {
    let file = open_file();
    let mut forrest: Vec<String> = Vec::new();

    for line in file.lines() {
        forrest.push(line.parse().unwrap()); 
    }

    println!("{forrest:?}");

    let mut invisible_left_trees = String::new();
    let mut invisible_right_trees = String::new();
    
    let mut top_up_tree = String::new();

    for column in forrest {
        let mut top_left_tree = 0;
        for tree in column.chars() {
            let tree = tree.to_digit(10).unwrap(); // this line just turns tree to u32
            if tree >= top_left_tree {
                top_left_tree = tree;
                invisible_left_trees.push('x')
            } else {invisible_left_trees.push_str(&tree.to_string())}    
        }
        invisible_left_trees.push_str("\n");

        let mut top_right_tree = 0;
        for tree in column.chars().rev() {
            let tree = tree.to_digit(10).unwrap(); // this line just turns tree to u32
            if tree >= top_right_tree {
                top_right_tree = tree;
                invisible_right_trees.push('x');
            } else {invisible_right_trees.push_str(&tree.to_string())}    
        }
        invisible_right_trees.push_str("\n");

        // top_up_tree.push(column.chars().nth(0).unwrap());

        // this shouldnt be in this loop, it needs to check just one column not all of them
        for iter in column.chars().enumerate() {
        println!("enumeration: {iter:?}");
        } 
    }
    println!("top up tree: {top_up_tree}");

    let mut tree_format = "".to_string();

    for column in invisible_left_trees.lines() {
        let column_buffer = column.chars().collect::<String>();
        tree_format.push_str(&column_buffer);
        tree_format.push_str("\n");
        dbg!(column);
    } 

    tree_format.push_str("\n");

    for column in invisible_right_trees.lines() {
        let column_buffer = column.chars().rev().collect::<String>();
        tree_format.push_str(&column_buffer);
        tree_format.push_str("\n");
        dbg!(column);
    }

    // figure ouev}");
    println!("forest after: \n\n{tree_format}");
}        

