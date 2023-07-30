use color_eyre::eyre::Result;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Default)]
struct Stack(Vec<char>);

impl Stack {
    fn push(&mut self, c: char) {
        self.0.push(c);
    }

    fn pop(&mut self) -> char {
        self.0.pop().expect("stack should not be empty")
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for c in self.0.iter() {
            write!(f, "{c}")?
        }

        Ok(())
    }
}

fn print_stacks(stacks: &[Stack]) {
    let Some(height) = stacks.iter().map(|s| s.height()).max() else {
        eprintln!("print_stacks: nothing to print");
        return;
    };

    for index in (0..height).rev() {
        for stack in stacks {
            let s = if index < stack.height() {
                format!(" [{}]", stack.0[index])
            } else {
                String::from("    ")
            };
            print!("{s}");
        }
        println!();
    }

    for index in 1..=stacks.len() {
        print!("  {index} ");
    }
    println!();
}

fn data_path(name: &str) -> PathBuf {
    let this_file = PathBuf::from(file!());
    let src = this_file
        .parent()
        .expect("source file should be in src dir");
    let repo = src.parent().expect("src should be in repo dir");
    repo.join("data").join(name)
}

fn read_data(name: &str) -> String {
    let path = data_path(name);
    fs::read_to_string(&path).expect("data file should be readable")
}

fn parse_count(line: &str) -> usize {
    line.split_ascii_whitespace()
        .rev()
        .next()
        .expect("count line needs counts")
        .parse()
        .expect("count should be an int")
}

fn parse_head_char(line: &str, index: usize) -> Option<char> {
    line.chars()
        .nth(index * 4 + 1)
        .filter(|c| !c.is_ascii_whitespace())
}

fn parse_head(head: &str) -> Vec<Stack> {
    let mut headlines = head.lines().rev();
    let count_line = headlines.next().expect("expected a counting line");
    let count = parse_count(count_line);
    let mut stacks = vec![Stack::default(); count];
    for line in headlines {
        for index in 0..count {
            if let Some(c) = parse_head_char(line, index) {
                stacks[index].push(c);
            }
        }
    }
    stacks
}

fn part_one(file_name: &str) -> String {
    let data = read_data(file_name);
    let (head, body) = data
        .split_once("\n\n")
        .expect("head and body should be separated by a blank line");

    let mut stacks = parse_head(head);

    for line in body.lines() {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let amount: usize = words[1].parse().expect("Amount should be int");
        let source: usize = words[3].parse().expect("Source should be int");
        let target: usize = words[5].parse().expect("Target should be int");

        for _ in 0..amount {
            let c = stacks[source - 1].pop();
            stacks[target - 1].push(c);
        }
    }

    // print_stacks(&stacks);
    stacks.iter().filter_map(|s| s.0.last()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("dummy-data.txt"), "CMZ");
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_one("data.txt"));

    Ok(())
}
