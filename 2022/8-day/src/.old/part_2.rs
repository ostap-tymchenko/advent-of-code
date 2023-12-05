use std::path::Path;
use std::fs::File;
use std::io::Read;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn open_file(file_path: &str) -> String {
    let file_path = Path::new(file_path);
    let display = file_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains: data\n", display),
        Ok(_) => println!("{display} contains: data"),
    }

    data
}

fn parse(input_data:String) -> Vec<Vec<i8>> {
    let mut parsed_forrest: Vec<Vec<i8>> = Vec::new();
    for line in input_data.split('\n') {
        let mut parsed_line:Vec<i8> = Vec::new();
        for number in line.chars() {
            parsed_line.push(number.to_digit(10).unwrap() as i8);
        }
        if !parsed_line.is_empty() {
            parsed_forrest.push(parsed_line);
        }
    }
    parsed_forrest
}

#[derive(Debug, EnumIter, PartialEq)]
enum CardinalDirections {
    NORTH,
    WEST,
    SOUTH,
    EAST
}

pub fn main() {
    let forrest = parse(open_file("src/dummy-data.txt"));
    let mut top_scenic_score = 0;

    println!("reached 1");
    for (x, row) in forrest.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            let mut scenic_score = 1;
            for direction in CardinalDirections::iter(){
                let mut iter: usize = 0;
                loop {
                    println!("reached 2");
                    iter += 1;
                    match forrest.get(calc_offset(&iter, &direction, x)) { // checks if x is in bounds
                        Some(x_in_bounds) => {
                            match x_in_bounds.get(y) { // checks if y is in bounds
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
                        } None => {
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

    println!("{top_scenic_score}");
} 

fn calc_offset (iter: &usize, direction:&CardinalDirections, x_or_y: usize) -> usize {
    println!("reached calc_offset");
    if *direction == CardinalDirections::NORTH {
        return x_or_y+iter; //x only
    } else if *direction == CardinalDirections::SOUTH {
        return x_or_y-iter; //x only
    } else if *direction == CardinalDirections::EAST {
        return x_or_y+iter; //y only
    } else if *direction == CardinalDirections::WEST {
        return x_or_y-iter; // y only
    } else {
        panic!("unreachable state reached");
    }
}
