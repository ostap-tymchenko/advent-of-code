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

    // let right_reconstruction = filter(&forrest, true);
    // let left_reconstruction = filter(&forrest, false);
    // let top_reconstruction = flip(&filter(&flip(&forrest, true), true), true);
    // let bottom_reconstruction = (flip(&filter(&flip(&forrest, false), true), false));

    let comp_1 = filter_from_right(&forrest);
    let comp_2 = filter(&forrest, true);
    // let test_bottom_reconstruction = flip(&filter(&flip(&forrest, false), true), false);  
    // let t_b_r_no_recon = /* flip(*/ &flip(&forrest, false); /* , true); */
    // let t_b_r_no_recon_p_flip_left = flip( &flip(&forrest, false), true);

    // display_forrest(&right_reconstruction, "right_reconstruction");
    // display_forrest(&left_reconstruction, "left_reconstruction");
    // display_forrest(&top_reconstruction, "top_reconstruction");
    // display_forrest(&bottom_reconstruction, "bottom_reconstruction");

    // display_forrest(&test_bottom_reconstruction, "test_bottom_reconstruction");
    // display_forrest(&t_b_r_no_recon, "t_b_r_no_recon");
    // display_forrest(&t_b_r_no_recon_p_flip_left, "t_b_r_no_recon_p_flip_left");
    display_forrest(&comp_1, "comp_1");
    display_forrest(&comp_2, "comp_2");
}

fn filter_from_right(forrest: &Vec<String>) -> Vec<String> {
    let mut output = Vec::<String>::new(); 
    let mut output_buffer = String::new();
    for row in forrest.iter() {
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

        for line in output_buffer.lines() {
            output.push(line.to_string());
        }
    }
    output
}

fn filter_from_left(forrest: &Vec<String>) -> Vec<String> {
    let mut output = Vec::<String>::new(); 
    let mut output_buffer = String::new();
    for row in forrest.iter() {
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

        for line in output_buffer.lines() {
            output.push(line.chars().rev().collect::<String>())
        }
    }
    output
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

fn flip(forrest: &Vec<String>, flip_left: bool) -> Vec<String> {
    let mut flipped = vec![String::new(); forrest[0].len()]; 
    if flip_left {
        for row in forrest.iter() {
            for (tree_iter, _) in row.chars().enumerate() {
                flipped[tree_iter].push(row.chars().nth(tree_iter).unwrap_or('F'));
            } 
        }
    } else {
        for row in forrest.iter() {
            for (tree_iter, _) in row.chars().enumerate() {
                flipped[tree_iter].push(row.chars().rev().nth(tree_iter).unwrap_or('F'));
            }
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
