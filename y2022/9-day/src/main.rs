use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

const DATA_FOLDER: &str = "data";
const FOLDER_SPLIT: &str = "/";

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct RopeEdge {
    x: i32,
    y: i32
}

// impl RopeEdge {
//     fn increase_y(&mut self) {
//         self.y += 1;
//     }
//
//     fn decrease_y(&mut self) {
//         self.y -= 1;
//     }
//
//     fn increase_x(&mut self) {
//         self.x += 1;
//     }
//
//     fn decrease_x(&mut self) {
//         self.x -= 1;
//     }
// }

fn is_to_far(tail: &RopeEdge, head: &RopeEdge) -> bool {
    if (head.y - tail.y).abs() > 1 {
        return true;
    } else if (head.x - tail.x).abs() > 1 {
        return true;
    } else {
        return false;
    }

}

fn part_one(file_name: &str) -> usize {
    let data_path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    let data = read_data(Path::new(&data_path));

    let mut head_position = RopeEdge::default();
    let mut last_head_position = RopeEdge::default();
    let mut tail_position = RopeEdge::default();
    let mut tail_history = HashSet::new();
    tail_history.insert(last_head_position);

    for line in data.lines() {
        let fc = line.chars().next().expect("line should be non empty");
        let repeat = line.split_ascii_whitespace().nth(1).expect("line should have repeat ammount").parse::<i32>().expect("repeat ammount should be i32");

        for _ in 0..repeat {

            last_head_position = head_position.clone();

            if fc == 'R' {
                head_position.x += 1;
            } else if fc == 'L' {
                head_position.x -= 1;
            } else if fc == 'U' {
                head_position.y += 1;
            } else if fc == 'D' {
                head_position.y -= 1;
            }

            // println!("new head pos: {head_position:?}");

            if is_to_far(&tail_position, &head_position) {
                // println!("to far");
                tail_position = last_head_position.clone();
                tail_history.insert(last_head_position);
            }
        }

    }

    tail_history.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), 13);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two("dummy-data.txt"), todo!);
    // }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_one("data.txt"));
    // println!("{}", part_one("dummy-data.txt"));

    Ok(())
}
