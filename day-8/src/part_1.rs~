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
//A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or row; that is, only look up, down, left, or right from any given tree.

pub fn main() {
    let file = open_file();
    let mut forrest: Vec<String> = Vec::new();

    for line in file.lines() {
        forrest.push(line.parse().unwrap()); 
    }

    println!("{forrest:?}\n");
    for row in forrest.iter() {
        println!("{row}"); 
    }
    println!("\n");

    let mut invisible_left_trees = String::new();
    let mut invisible_right_trees = String::new();
    
    let mut invisible_up_trees = String::new();

    for (iter_row, row) in forrest.iter().enumerate() {
        let mut top_left_tree = 0;
        for tree in row.chars() {
            let tree = tree.to_digit(10).unwrap(); // this line just turns tree to u32
            if tree >= top_left_tree {
                top_left_tree = tree;
                invisible_left_trees.push('x')
            } else {invisible_left_trees.push_str(&tree.to_string())}    
        }
        invisible_left_trees.push_str("\n");

        let mut top_right_tree = 0;
            for tree in row.chars().rev() {
            let tree = tree.to_digit(10).unwrap(); // this line just turns tree to u32
            if tree >= top_right_tree {
                top_right_tree = tree;
                invisible_right_trees.push('x');
            } else {invisible_right_trees.push_str(&tree.to_string())}    
        }
        invisible_right_trees.push_str("\n");
        
        // let mut top_up_tree = 0;
        // for tree in forrest.iter().map(|s| s.chars().nth(iter_row).unwrap()).collect::<String>().chars() {
        //     // println!("up_trees: {tree}");
        //     let tree = tree.to_digit(10).unwrap(); // this line just turns tree to u32
        //     if tree >= top_up_tree {
        //         top_up_tree = tree;
        //         invisible_up_trees.push('x');
        //     } else {
        //         invisible_up_trees.push_str(&tree.to_string())
        //     }
        // }
        // invisible_up_trees.push_str("\n");

        let mut top_up_tree = 0;
        println!("forrest: {forrest:?}");
        // vv line below vv beends to iterate over the lenght of the line not the # of lines;
        for tree in forrest.iter().map(|s| s.chars().nth(iter_row).unwrap()) {//(|s| s.chars().nth(iter_row).unwrap()).collect::<String>().chars() {
            // println!("up_trees: {tree}");
            let tree = tree.to_digit(10).unwrap(); // this line just turns tree to u32
            if tree >= top_up_tree {
                top_up_tree = tree;
                println!("pushing: x");
                invisible_up_trees.push('x');
            } else {
                println!("pushing: {tree}");
                invisible_up_trees.push_str(&tree.to_string())
            }
        }
        invisible_up_trees.push_str("\n");
    }

    let mut left_reconstruction = "".to_string();

    for row in invisible_left_trees.lines() {
        let row_buffer = row.chars().rev().collect::<String>();
        left_reconstruction.push_str(&row_buffer);
        left_reconstruction.push_str("\n");
        // dbg!(row);
    } 

    left_reconstruction.push_str("\n");

    let mut right_reconstruction = "".to_string();

    for row in invisible_right_trees.lines() {
        let row_buffer = row.chars().rev().collect::<String>();
        right_reconstruction.push_str(&row_buffer);
        right_reconstruction.push_str("\n");
        // dbg!(row);
    }

    right_reconstruction.push_str("\n");
    
    let mut up_reconstruction_buffer = String::new();
    for (row_iter, row_debug) in invisible_up_trees.lines().enumerate() {
        for tree in invisible_up_trees.lines().map(|s| s.chars().nth(row_iter).unwrap()).collect::<String>().chars() {
            up_reconstruction_buffer.push(tree);
            println!("pushing tree {tree}, in row_iter {row_iter} row {row_debug}")
        }
        up_reconstruction_buffer.push_str("\n");
    }

    // let mut up_reconstruction = String::new();
    // for row in up_reconstruction_buffer.lines().rev() {
    //     up_reconstruction.push_str(row);
    //     up_reconstruction.push_str("\n");
    // }

    println!("up_reconstruction_buffer: \n{up_reconstruction_buffer}");
    println!("left_reconstruction: \n{left_reconstruction}");
    println!("right_reconstruction: \n{right_reconstruction}");
}        

