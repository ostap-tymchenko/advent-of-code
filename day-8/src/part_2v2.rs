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
                if direction == CardinalDirections::NORTH {
                    let mut iter: usize = 0;
                    loop {
                        iter += 1;
                        match forrest.get(x+iter) {
                            Some(north_tree) => {
                            if north_tree[y] >= tree {
                                scenic_score = scenic_score * *iter;
                                break;
                            }
                        } None => 
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
