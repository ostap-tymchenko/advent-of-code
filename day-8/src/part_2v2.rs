use std::path::Path;
use std::fs::File;
use std::io::Read;

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
        // forrest.push(vec![(line.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() as i8)]);
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
    let forrest = parse(open_file("src/dummy-data.txt"));
    // dbg!(&forrest);
    
    // for row in forrest {
    //     for considered_tree in row.chars() {
    //         let considered_tree = considered_tree.try_into().unwrap();
    //         let mut scenic_score = 0;
    //         for (iter, very_scenic_tree) in row.chars().enumerate() {
    //             if very_scenic_tree >= considered_tree {
    //                 if scenic_score != 0 {
    //                     scenic_score = scenic_score * iter
    //                 }
    //             } 
    //         }

    //     }
    // }
    
    // for (x, row) in forrest.iter().enumerate() {
    //     for (y, tree) in row.chars().enumerate() {
    //         // println!("x:{x}, row:{row}, y:{y}, tree:{tree}")
    //         // println!("row {x} column {y} is {}", check_height(x, y, &forrest));
    
    for (x, row) in forrest.iter().enumerate() {
        // dbg!(row);
        for (y, tree) in row.iter().enumerate() {
            // dbg!(x, y, tree);
            let mut counter = 0;
            loop {
                counter += 1;
                if check_height(x+counter, y+counter, &forrest) >= tree {
                    break;
                }
            }
        }
    }
}

// fn calc_senic_score(x: usize, y:usize, forrest: &Vec<String>) {
//     let center_height = check_height(x, y, forrest);
//     let mut counter: usize = 0;
//     loop {
//         counter += 1;
//         if !check_height(x+counter, y+counter, forrest) >= center_height {
//             break;
//         }
//     }
// }

fn check_height(x: i8, y: i8, forrest: &Vec<Vec<i8>>) -> char {
    forrest[x]
}
