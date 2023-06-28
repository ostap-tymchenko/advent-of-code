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

pub fn main() {

    #[derive(Debug, EnumIter)]
    enum Directions {
        NORTH,
        WEST,
        SOUTH,
        EAST
    }

    let forrest = parse(open_file("src/dummy-data.txt"));
    
    let mut top_scenic_score: i32;

    for (x, row) in forrest.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            let mut scenic_score = 1;
            for direction in Directions::iter(){
                let mut iter = 0;
                dbg!(&direction);
                loop {
                    iter += 1;
                    // if !forrest[x+iter][y] >= *tree {
                    //     scenic_score = scenic_score * iter;
                    //     break;
                    // }
                    // match forrest.get(x+iter).get(y) {
                    //     Some(height) => {
                    //         if !height >= tree {
                    //             scenic_score = scenic_score * iter;
                    //             break;
                    //         }
                    //     } None => {

                    //     }
                    // } 
                    
                    match forrest.get(x+iter) { // checks if x is in bounds
                        Some(x_in_bounds) => {
                            match x_in_bounds.get(y) { // checks if y is in bounds
                                Some(y_in_bounds) => {
                                    if !y_in_bounds >= *tree {
                                        scenic_score = scenic_score * iter;
                                        break;
                                    }
                                }
                                None => {
                                    todo!() // TODO
                                }
                            }
                        } None => {
                            todo!(); // TODO
                        }
                    }
                } 
            } 
        } 
    } 
} 
