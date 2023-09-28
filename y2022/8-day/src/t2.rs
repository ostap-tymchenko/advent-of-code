use color_eyre::eyre::Result;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

const DIR_SPLIT_STR: &str = "/";
const DATA_FOLDER: &str = "data";

#[derive(Debug, PartialEq)]
enum CardinalDirections {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(data_path).expect("data parse fail")
}

fn filter_from_right(forrest: &Vec<String>) -> Vec<String> {
    let mut output = Vec::<String>::new();
    let mut output_buffer = String::new();
    for (row_iter, row) in forrest.iter().enumerate() {
        let mut top_tree_hight = 0;
        for tree in row.chars() {
            let tree = tree.to_digit(10).unwrap();
            if tree > top_tree_hight || row_iter == 0 || row_iter == forrest.len() - 1 {
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
            if tree > top_tree_hight || row_iter == 0 || row_iter == forrest.len() - 1 {
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

fn forrest_reconstruction(
    right_reconstruction: &Vec<String>,
    left_reconstruction: &Vec<String>,
    top_reconstruction: &Vec<String>,
    bottom_reconstruction: &Vec<String>,
) -> Vec<String> {
    let mut invisible_forrest = vec![String::new(); right_reconstruction.len()];
    for (x, row) in right_reconstruction.iter().enumerate() {
        for (y, tree) in row.chars().enumerate() {
            if access_by_cords(x, y, right_reconstruction) != 'x'
                && access_by_cords(x, y, left_reconstruction) != 'x'
                && access_by_cords(x, y, top_reconstruction) != 'x'
                && access_by_cords(x, y, bottom_reconstruction) != 'x'
            {
                invisible_forrest[x].push(tree);
            } else {
                invisible_forrest[x].push('x');
            }
        }
    }

    invisible_forrest[0] = "x".repeat(invisible_forrest[0].len());
    let forrest_len = invisible_forrest.len() - 1;
    invisible_forrest[forrest_len] =
        "x".repeat(invisible_forrest[invisible_forrest.len() - 1].len());
    return invisible_forrest;
}

fn access_by_cords(x: usize, y: usize, forrest: &Vec<String>) -> char {
    return forrest[x].chars().nth(y).unwrap();
}

fn part_one(file_name: &str) -> i32 {
    let data_path = DATA_FOLDER.to_owned() + DIR_SPLIT_STR + file_name;
    let data = read_data(Path::new(&data_path));

    // dbg!(&data);
    let mut forrest: Vec<String> = Vec::new();

    // println!("forrest:");
    for line in data.lines() {
        forrest.push(line.parse().unwrap());
        // println!("{line}");
    }

    // display_forrest(&forrest, "og_forrest");

    let right_reconstruction = filter_from_right(&forrest);
    let left_reconstruction = filter_from_left(&forrest);
    let top_reconstruction = flip_right(&filter_from_right(&flip_left(&forrest)));
    let bottom_reconstruction = flip_left(&filter_from_right(&flip_right(&forrest)));

    let new_forrest = forrest_reconstruction(
        &right_reconstruction,
        &left_reconstruction,
        &top_reconstruction,
        &bottom_reconstruction,
    );
    // display_forrest(&forrest, "og forrest");
    // display_forrest(&new_forrest, "new forrest");

    let mut visible_trees = 0;
    for row in &new_forrest {
        for tree in row.chars() {
            if tree == 'x' {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn parse(input_data: String) -> Vec<Vec<i8>> {
    let mut parsed_forrest: Vec<Vec<i8>> = Vec::new();
    for line in input_data.split('\n') {
        let mut parsed_line: Vec<i8> = Vec::new();
        for number in line.chars() {
            parsed_line.push(number.to_digit(10).unwrap() as i8);
        }
        if !parsed_line.is_empty() {
            parsed_forrest.push(parsed_line);
        }
    }
    parsed_forrest
}

fn calc_offset(iter: &usize, direction: &CardinalDirections, x_or_y: usize) -> usize {
    println!("reached calc_offset");
    if *direction == CardinalDirections::NORTH {
        return x_or_y + iter; //x only
    } else if *direction == CardinalDirections::SOUTH {
        return x_or_y - iter; //x only
    } else if *direction == CardinalDirections::EAST {
        return x_or_y + iter; //y only
    } else if *direction == CardinalDirections::WEST {
        return x_or_y - iter; // y only
    } else {
        panic!("unreachable state reached");
    }
}

fn part_two(file_name: &str) -> usize {
    let data_path = DATA_FOLDER.to_owned() + DIR_SPLIT_STR + file_name;
    let data = read_data(Path::new(&data_path));
    let forrest = parse(data);

    let mut top_scenic_score = 0;
    let directions = [
        CardinalDirections::NORTH,
        CardinalDirections::EAST,
        CardinalDirections::SOUTH,
        CardinalDirections::WEST,
    ];

    for (x, row) in forrest.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            let mut scenic_score = 1;
            for direction in &directions {
                let mut iter: usize = 0;
                loop {
                    iter += 1;
                    match forrest.get(calc_offset(&iter, &direction, x)) {
                        // checks if x is in bounds
                        Some(x_in_bounds) => {
                            match x_in_bounds.get(y) {
                                // checks if y is in bounds
                                Some(y_in_bounds) => {
                                    if !y_in_bounds >= *tree {
                                        scenic_score = scenic_score * iter;
                                        break;
                                    }
                                }
                                None => {
                                    scenic_score = scenic_score * iter;
                                    break;
                                }
                            }
                        }
                        None => {
                            scenic_score = scenic_score * iter;
                            break;
                        }
                    }
                }
            }
            if scenic_score > top_scenic_score {
                top_scenic_score = scenic_score;
            }
        }
    }

    top_scenic_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_example_p1() {
        assert_eq!(part_one("dummy-data.txt"), 21);
    }

    // #[test]
    // fn test_default_example_p2() {
    //     assert_eq!(part_one("dummy-data.txt"), 21);
    // }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_one("data.txt"));
    // println!("{}", part_two("data.txt"));

    Ok(())
}
