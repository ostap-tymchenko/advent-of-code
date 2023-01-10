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

    let mut invisible_trees = 0;
    
    let file = open_file();
    let mut forrest: Vec<String> = Vec::new();

    for line in file.lines() {
        forrest.push(line.parse().unwrap()); 
    }

    println!("{forrest:?}");

    let forrest_copy = forrest.clone();
    for (column_iter, column) in forrest_copy.iter().enumerate() {
        for (tree_iter, tree) in column.chars().enumerate() {
            let tree = tree.to_digit(10).unwrap();
            println!("tree :{tree}");

            let mut tallest_tree_left = 0;
            let mut tallest_tree_right = 0;
            // let mut tallest_tree_left = 0;
            // let mut tallest_tree_left = 0;

            if tree >= tallest_tree_left {    
                tallest_tree_left = tree;
                println!("new tallest tree from left: {tallest_tree_left}");
            } else {
                 forrest[column_iter].remove(tree_iter);
            }

            println!("new forrest: {forrest:?}")
        }
    }

}        

fn remove_left (forrest: Vec<String>) -> Vec<String> {
    
    forrest
}


// fn check_top_visibility (file: String) {
//
//     let mut last_line: Vec<char> = vec![];
//     for line in file.lines() {
//         for (iter, tree_horizontal_index) in (line).chars().enumerate() {
//             if !last_line.is_empty() {
//                 let last_line_iter = last_line[iter];
//                 if last_line_iter == tree_horizontal_index {
//                     println!("last_line[iter]: {last_line_iter} == tree_horizontal_index"); 
//                 } else {
//                     println!("last_line[iter]: {last_line_iter} !== tree_horizontal_index");
//                 }
//             }
//         }
//         last_line = line.chars().collect();
//         println!("last line = {last_line:?}");
//     }
// }
//
