use std::fs;
use std::path::Path;

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

struct Data {
    data: String,
}

struct Head {
    data: String,
    ends_at_line: i32,
}

struct Body {
    data: String,
}

fn parse_head(data:Data) -> Head {
    let mut head = String::from("");
    for (line_num, line_data) in data.data.lines().enumerate() {
        if line_data.is_empty() {
            break;
        } else {
            head.push_str(line);
            head.push('\n');
        }
    }
    Head { data: head }
}

fn parse body

fn part_one() {
    let data = read_data(Path::new("./data.txt"));

}

fn main() {
    part_one();
}
