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
    }

    // display_forrest(&forrest, "og_forrest");

    // let right_reconstruction = filter_from_right(&forrest); 
    // let left_reconstruction = filter_from_left(&forrest); 
    // let top_reconstruction = flip_right(&filter_from_right(&flip_left(&forrest)));
    // let bottom_reconstruction = flip_left(&filter_from_right(&flip_right(&forrest)));
    //
    // display_forrest(&new_forrest, "new forrest");
    //
    // for row in forrest.iter() {
    //     for (tree_iter, tree) in row.chars().enumerate() {
    //         for (side, _) in (0..3).enumerate() {
    //             // order: left, top, right, down
    //             if side == 0 {
    //                 for row[0..tree_iter].chars().rev()
    //                  
    //             }
    //         }
    //     } 
    // }

    for row in forrest.iter() {
        println!("considering: {row}");
        let mut from_left_top = 0;
        let mut from_left_concurent_top = 0;
        for (tree_iter, tree) in row.chars().enumerate() {
            println!("tree: {tree}");
            let tree = tree.to_digit(10).unwrap();
            if tree >= from_left_top {
                from_left_top = tree;
            }

            for _ in row[0..from_left_top].chars() {
                from_left_concurent_top += 1;
            }
        } 
        println!("from_left_top: {from_left_top}\nfrom_left_concurent_top: {from_left_concurent_top}");
    }
}

fn filter_from_right(forrest: &Vec<String>) -> Vec<String> {
    let mut output = Vec::<String>::new(); 
    let mut output_buffer = String::new();
    for (row_iter, row) in forrest.iter().enumerate() {
        let mut top_tree_hight = 0;
        for tree in row.chars() {
            let tree = tree.to_digit(10).unwrap();
            if tree > top_tree_hight || row_iter == 0 || row_iter == forrest.len() -1 {
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
            if tree > top_tree_hight || row_iter == 0 || row_iter == forrest.len() -1{
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
            flipped_buffer[tree_iter].push(row.chars().rev().nth(tree_iter).unwrap_or('F'));
        } 
    }

    let mut flipped = Vec::<String>::new();
    for line in flipped_buffer.iter().rev() {
        flipped.push(line.to_string().chars().rev().collect());
    }
    flipped
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
    println!("\n | {name}");
    for (row_iter, row) in forrest_reconstruction.iter().enumerate() {
        if row_iter < 10 {
            println!("{row_iter}|  {row}");
        } else {
            println!("{row_iter}| {row}");
        }
    }
} 

fn forrest_reconstruction(right_reconstruction: &Vec<String>, left_reconstruction: &Vec<String>, top_reconstruction: &Vec<String>, bottom_reconstruction: &Vec<String>) -> Vec<String> {
    let mut invisible_forrest = vec![String::new(); right_reconstruction.len()]; 
    for (x, row) in right_reconstruction.iter().enumerate() {
        for (y, tree) in row.chars().enumerate() {
            if access_by_cords(x, y, right_reconstruction) != 'x' && access_by_cords(x, y, left_reconstruction) != 'x' && access_by_cords(x, y, top_reconstruction) != 'x' && access_by_cords(x, y, bottom_reconstruction) !='x' {
                invisible_forrest[x].push(tree);
            } else {
                invisible_forrest[x].push('x');
            }
        }
    }

invisible_forrest[0] = "x".repeat(invisible_forrest[0].len()); 
let forrest_len = invisible_forrest.len() -1;
invisible_forrest[forrest_len] = "x".repeat(invisible_forrest[invisible_forrest.len()-1].len());
return invisible_forrest;
}

fn access_by_cords(x: usize, y: usize, forrest: &Vec<String>) -> char {
    return forrest[x].chars().nth(y).unwrap();
}
