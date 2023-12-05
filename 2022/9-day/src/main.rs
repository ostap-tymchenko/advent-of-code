use color_eyre::eyre::Result;
use std::any::type_name;
use std::collections::HashSet;
use std::path::Path;
use std::{fs, vec};

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

enum Direction {
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
}

fn where_to_move_if_to_far(head: &RopeEdge, tail: &RopeEdge) -> Option<Direction> {
    // Calculate the difference in x and y coordinates
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;

    // Check if the head is further than the tail by more than one diagonally or cardinally
    if dx.abs() > 1 || dy.abs() > 1 {
        // Determine the direction based on the differences in x and y coordinates
        match (dx, dy) {
            (0, dy) if dy > 0 => Some(Direction::N),
            (dx, dy) if dx > 0 && dy > 0 => Some(Direction::Ne),
            (dx, 0) if dx > 0 => Some(Direction::E),
            (dx, dy) if dx > 0 && dy < 0 => Some(Direction::Se),
            (0, dy) if dy < 0 => Some(Direction::S),
            (dx, dy) if dx < 0 && dy < 0 => Some(Direction::Sw),
            (dx, 0) if dx < 0 => Some(Direction::W),
            (dx, dy) if dx < 0 && dy > 0 => Some(Direction::Nw),
            _ => None, // Not further than one diagonally or cardinally
        }
    } else {
        None
    }
}

#[allow(dead_code)]
fn display_grid(edges: &HashSet<RopeEdge>) {
    // Initialize minimum and maximum x and y values
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    // Calculate the minimum and maximum x and y values
    for edge in edges {
        if edge.x < min_x {
            min_x = edge.x;
        }
        if edge.x > max_x {
            max_x = edge.x;
        }
        if edge.y < min_y {
            min_y = edge.y;
        }
        if edge.y > max_y {
            max_y = edge.y;
        }
    }

    // Calculate the width and height based on min and max values
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let mut grid = vec![vec!['.'; width]; height];

    for edge in edges {
        let adjusted_x = edge.x - min_x;
        let adjusted_y = edge.y - min_y;
        if adjusted_x >= 0
            && adjusted_x < width as i32
            && adjusted_y >= 0
            && adjusted_y < height as i32
        {
            // grid[adjusted_y as usize][adjusted_x as usize] = '█';
            grid[adjusted_y as usize][adjusted_x as usize] = '#';
        }
    }

    for row in grid.iter().rev() {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn part_two(data: String, tail_len: usize) -> usize {
    let tail_pos_len = tail_len + 1;
    let mut knots = vec![RopeEdge::default(); tail_pos_len];
    let mut unique_tails: HashSet<RopeEdge> = HashSet::new();

    for line in data.lines() {
        let fc = line.chars().next().expect("line should be non empty");

        let repeat = line
            .split_ascii_whitespace()
            .nth(1)
            .expect("line should have repeat ammount")
            .parse::<i32>()
            .expect("repeat ammount should be parseble to i32");

        for _ in 0..repeat {
            match fc {
                'R' => knots[0].x += 1,
                'L' => knots[0].x -= 1,
                'U' => knots[0].y += 1,
                'D' => knots[0].y -= 1,

                _ => panic!("line should be non empty"),
            }

            for tail_iter in 0..tail_pos_len {
                unique_tails.insert(
                    *knots
                        .last()
                        .expect("tail positions should be non empty"),
                );

                // for each element of the array
                if let Some(tail_after_me) = knots.get(tail_iter + 1) {
                    // if theres a next
                    if let Some(move_direction) =
                        where_to_move_if_to_far(&knots[tail_iter], tail_after_me)
                    {
                        // if its to far from relative head
                        let tail_idx_after_me = tail_iter + 1;
                        use Direction as D;
                        match move_direction {
                            // move twords relative head by one
                            D::N => knots[tail_idx_after_me].y += 1,
                            D::S => knots[tail_idx_after_me].y -= 1,

                            D::E => knots[tail_idx_after_me].x += 1,
                            D::W => knots[tail_idx_after_me].x -= 1,

                            D::Ne => {
                                knots[tail_idx_after_me].y += 1;
                                knots[tail_idx_after_me].x += 1;
                            },

                            D::Nw => {
                                knots[tail_idx_after_me].y += 1;
                                knots[tail_idx_after_me].x -= 1;
                            },

                            D::Se => {
                                knots[tail_idx_after_me].y -= 1;
                                knots[tail_idx_after_me].x += 1;
                            }

                            D::Sw => {
                                knots[tail_idx_after_me].y -= 1;
                                knots[tail_idx_after_me].x -= 1;
                            }
                        }
                    }
                }
            }
        }
    }

    // dbg!(&unique_tails);
    display_grid(&unique_tails);
    unique_tails.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_two(read_data_from_name("dummy-data.txt"), 1), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(read_data_from_name("dummy-data-p2.txt"), 9), 36);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    // println!("{}", part_two(read_data_from_name("data.txt"), 1));
    println!("{}", part_two(read_data_from_name("data.txt"), 9));

    Ok(())
}
