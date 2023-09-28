use color_eyre::eyre::Result;
use std::any::type_name;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[allow(dead_code)]
fn print_object<T: std::fmt::Display>(o: T) {
    println!("{o}");
}

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(data_path).expect("data parse fail")
}

fn update_return(counter: i32, register: i32, ret: &mut i32) {
    if (counter + 20) % 40 == 0 || counter == 20 {
        *ret += counter * register;
        // println!("{counter}:{register}, c*r={}", counter * register);
    }
}

const DATA_FOLDER: &str = "data";
const FOLDER_SPLIT: &str = "/";

fn part_one(file_name: &str) -> i32 {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));

    let mut counter = 0;
    let mut register = 1;
    let mut ret = 0;

    for line in data.lines() {
        if line.chars().next().expect("line should be non-empty") == 'a' {
            counter += 1;
            update_return(counter, register, &mut ret);
            counter += 1;
            update_return(counter, register, &mut ret);
            register += line
                .split_whitespace()
                .nth(1)
                .expect("addx should have num")
                .parse::<i32>()
                .expect("addx's add should be valid i32");
        } else {
            counter += 1;
            update_return(counter, register, &mut ret);
        }
    }

    ret
}

fn print_crt(crt_output: Vec<char>) {
    println!();
    for (i, _) in crt_output.iter().enumerate() {
        if i % 40 == 0 && i != 0 {
            println!("  {i}");
        }

        if crt_output[i] == '#' {
            // print!("##");
            print!("██");
        } else {
            // print!(".")
            print!("░░");
        }

        if i == 239 {
            println!("  240")
        }
    }
}

fn update_crt(crt_output: &mut Vec<char>, register: i32) {
    let horizontal_index = crt_output.len() % 40;
    let reg_as_usize = register as usize;

    println!("index:{} hindex:{} register:{register}",crt_output.len(), horizontal_index);
    if horizontal_index == reg_as_usize
        || horizontal_index == reg_as_usize.checked_add(1).unwrap_or(reg_as_usize)
        || horizontal_index == reg_as_usize.checked_sub(1).unwrap_or(reg_as_usize)
    {
        crt_output.push('#')
    } else {
        crt_output.push('.')
    }
}

fn part_two(file_name: &str) -> Vec<char> {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));

    let mut register = 1;
    let mut crt_output: Vec<char> = vec![];

    for line in data.lines() {
        if line.chars().next().expect("line should be non-empty") == 'a' {
            update_crt(&mut crt_output, register);
            update_crt(&mut crt_output, register);

            let addx = line
                .split_whitespace()
                .nth(1)
                .expect("addx should have num")
                .parse::<i32>()
                .expect("addx's add should be valid i32");

            // println!("addx:{addx}");
            register += addx;

        } else {
            update_crt(&mut crt_output, register);
        }
    }

    crt_output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let p1 = part_one("dummy-data.txt");
        assert_eq!(p1, 13140);
    }

    #[test]
    fn test_part_two() {
        let p2 = part_two("dummy-data.txt");
        assert_eq!(p2, Vec::from_iter("##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....".chars()));
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    // part_one("data.txt");
    print_crt(part_two("data.txt"));
    // print_crt(part_two("dummy-data.txt"));

    Ok(())
}
