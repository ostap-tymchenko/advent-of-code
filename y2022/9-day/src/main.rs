use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

const DATA_FOLDER: &str = "data";
const FOLDER_SPLIT: &str = "/";

fn read_data_from_name(file_name: &str) -> String {
    let path = DATA_FOLDER.to_owned() + FOLDER_SPLIT + file_name;
    fs::read_to_string(Path::new(&path)).expect("data parse fail")
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct RopeEdge {
    x: i32,
    y: i32,
}

fn is_to_far(tail: &RopeEdge, head: &RopeEdge) -> bool {
    if (head.y - tail.y).abs() > 1 {
        return true;
    } else if (head.x - tail.x).abs() > 1 {
        return true;
    } else {
        return false;
    }
}

// fn display_rope_edges(rope_edges: Vec<RopeEdge>) {
// }

#[allow(dead_code)]
fn part_one(data: String) -> usize {
    let mut head_position = RopeEdge::default();
    let mut last_head_position = RopeEdge::default();
    let mut tail_position = RopeEdge::default();
    let mut tail_history = HashSet::new();
    tail_history.insert(last_head_position);

    for line in data.lines() {
        let fc = line.chars().next().expect("line should be non empty");
        let repeat = line
            .split_ascii_whitespace()
            .nth(1)
            .expect("line should have repeat ammount")
            .parse::<i32>()
            .expect("repeat ammount should be i32");

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

// fn is_to_far(tail: &RopeEdge, head: &RopeEdge) -> bool {
//     if (head.y - tail.y).abs() > 1 {
//         return true;
//     } else if (head.x - tail.x).abs() > 1 {
//         return true;
//     } else {
//         return false;
//     }
// }

#[allow(dead_code, unused_variables, unused_mut)]
fn part_two(data: String) -> usize {
    let tail_len = 9;
    let mut tail_history;
    let mut tail_positions = vec![RopeEdge::default(); tail_len + 1];
    let mut unique_tail_positions: HashSet<RopeEdge> = HashSet::new();

    for line in data.lines() {
        let fc = line.chars().next().expect("line should be non empty");
        let repeat = line
            .split_ascii_whitespace()
            .nth(1)
            .expect("line should have repeat ammount")
            .parse::<i32>()
            .expect("repeat ammount should be i32");

        for _ in 0..repeat {
            tail_history = tail_positions.clone();

            if fc == 'R' {
                tail_positions[0].x += 1;
            } else if fc == 'L' {
                tail_positions[0].x -= 1;
            } else if fc == 'U' {
                tail_positions[0].y += 1;
            } else if fc == 'D' {
                tail_positions[0].y -= 1;
            }

            let tail_pos_len = tail_positions.len();
            for tail_iter in 0..tail_pos_len {
                // if let Some(tail_before_idx) = tail_iter.checked_add(1) {
                if let tail_before = tail_positions[tail_iter + 1] {
                    dbg!(tail_before);
                    if is_to_far(&tail_positions[tail_iter], &tail_before) {
                        tail_before = tail_history[tail_iter];
                    }
                }
            }
            unique_tail_positions.insert(*tail_positions.last().unwrap());
        }
    }

    dbg!(tail_positions);
    // dbg!(&unique_tail_positions);
    unique_tail_positions.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(read_data_from_name("dummy-data.txt")), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(read_data_from_name("dummy-data.txt")), 36);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    // println!("{}", part_one(read_data_from_name("data.txt")));
    println!("{}", part_two(read_data_from_name("data.txt")));

    Ok(())
}
