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

    for column in forrest {
        remove_left(column);
    }

}        


fn remove_left(column: String) -> String {
    let mut top_tree = 0;
    let mut chars: Vec<char> = column.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        let tree = chars[index];
        let tree = tree.to_digit(10).unwrap();
        if tree >= top_tree {
            top_tree = tree;
            chars.remove(index);
        }
        else {
            index += 1;
        }
    }
    chars.into_iter().collect()
}

println!("forest after: {forest}");
// fn remove_left(column: String) -> String {
//     let mut column: Vec<char> = column.chars().collect();
//     let mut top_tree = 0;
//     let mut removed = vec![];
//     for (index, &tree) in column.iter().enumerate() {
//         let tree = tree.to_digit(10).unwrap();
//         if tree >= top_tree {
//             top_tree = tree;
//             removed.push(index);
//         }
//     }
//     removed.sort();
//     removed.into_iter().rev().for_each(|i| { column.remove(i); } );
//     column.into_iter().collect::<String>()
// }
