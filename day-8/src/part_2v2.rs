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
        // dbg!(line);
        // forrest.push(vec![(line.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() as i8)]);
        let mut parsed_line:Vec<i8> = Vec::new();
        for number in line.chars() {
            parsed_line.push(number.to_digit(10).unwrap() as i8);
        }
        parsed_forrest.push(parsed_line);
    }

    dbg!(&parsed_forrest);
    parsed_forrest
} 

pub fn main() {
    let forrest = parse(open_file("src/dummy-data.txt"));
    // display_forrest(forrest, "Test forrest one");

    // let forrest: Vec<String> = file.split('\n').map(|x| x.to_string()).collect();
    
    // display_forrest(&forrest, "input"); 

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


    //         
    //     }
    // }
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

fn check_height(x: usize, y:usize, forrest: &Vec<String>) -> char {
    forrest[x].chars().nth(y).unwrap()
}

fn display_forrest(forrest: Vec<Vec<i8>>, name: &str) {
    println!("\n | {name}");
    for row in forrest.iter() {
            // The different spacing is on pourpose in 
            // order to get a cleaner formated output
            // println!("{row_iter}|  {row}");
            dbg!(row);
            // println!("{row_iter}| {row}");
            dbg!(row);
    }
} 
