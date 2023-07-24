use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use colored::*;

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

// GOAL : "Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?"


// fn move_up<'a> (browse_dir: &'a str, move_to: &'a str) -> &'a str {
//     let new_dir = browse_dir.to_owned() + move_to + "/";
//     &new_dir
// }


fn move_up (browse_dir: String, move_to: String) -> String{
    let new_dir = browse_dir + &move_to + "/";
    new_dir
}

fn move_down (browse_dir: &str) -> &str {
    let last_slash_index = browse_dir.find('/').unwrap();
    let new_dir = browse_dir;
    let (new_dir, _) = browse_dir.split_at(last_slash_index);
    println!("new dir={new_dir}");

    new_dir
}

enum Reposponse {
    Command,
    Output,
}


pub fn main () {
    let file = open_file().to_string();
    

    for mut line in file.lines() {
        if line.contains("$") {
            let line_type = Reposponse::Command;
            
        }
    }
}



// pub fn main() {
//     let file = open_file().to_string();
//
//     let mut browse_dir = "/";
//     let mut last_command = "";
//
//     for mut line in file.lines() {
//         if line.chars().nth(0).unwrap() == '$' {
//             // let line = line.replace("$", "");
//             let command = &line[2..4];
//             let argument = &line[4..];
//             let argument = argument.trim();
//             println!("command={command}, with argument={argument}");
//
//             if command == "cd" {
//                 if argument == "" || argument == "/" {
//                     // this means there going to the root dir
//                     browse_dir = "/";
//                     println!("new browse dir={browse_dir}");
//                 } else if argument == ".." {
//                     // this means there going one dir down
//                     move_down(browse_dir);
//                     println!("new dir={browse_dir}");
//                 } else {
//                     // this means were moving up to a specific folder
//                     println!("old browse_dir={browse_dir}");
//                     browse_dir = browse_dir.to_owned() + argument + "/";
//                     println!("new browse_dir={browse_dir}");
//                 } 
//             }
//         } else {
//             // output
//         }
//     }
// }   
//
