use unicode_segmentation::UnicodeSegmentation;
use color_eyre::eyre::Result;
use std::any::type_name;
use std::path::Path;
use std::fs;

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

#[derive(Debug)]
struct Monkeyid(u32);

// #[derive(Debug)]
// struct DivisibilityTest {
//     devisor: i32,
//     send_to_if_true: Monkeyid,
//     send_to_if_false: Monkeyid,
// }

#[derive(Debug)]
enum FearOps {
    Add(i32),
    Multiply(i32),
}

#[derive(Debug)]
#[allow(dead_code)]
struct Monkey {
    monkey_id: Monkeyid,
    inventory: Vec<i32>,
    fear_op: FearOps,
    devisor: i32,
    send_to_if_div: Monkeyid,
    send_to_if_not_div: Monkeyid,
    item_send_tally: i32,
}

trait StrExt {
    fn remove_last(&self) -> &str;
}

impl StrExt for str {
    fn remove_last(&self) -> &str {
        match self.grapheme_indices(true).next_back() {
            Some((i, _)) => &self[..i],
            None => self,
        }
    }
}

// monkey_id: Monkeyid,
// inventory: Vec<i32>,
// fear_op: FearOps,
// devisor: i32,
// send_to_if_div: Monkeyid,
// send_to_if_not_div: Monkeyid,
// item_send_tally: i32,

#[allow(dead_code)]
impl Monkey {
    pub fn construct_monkey(
        monkey_id: Monkeyid,
        fear_op: FearOps,
        devisor: i32,
        send_to_if_div: Monkeyid,
        send_to_if_not_div: Monkeyid,
    ) -> Monkey {
        Monkey {
            monkey_id,
            inventory: vec![],
            item_send_tally: 0,
            fear_op,
            devisor,
            send_to_if_div,
            send_to_if_not_div,
        }
    }
}

#[allow(dead_code, unused_variables)]
fn part_one(data: String) -> i32 {
    let monkeys: Vec<Monkey> = vec![];

    for monkey_data in data.split("\n\n") {
        println!("{monkey_data}");
        println!("------------------------------");
        let monkey_id: Monkeyid = Monkeyid(monkey_data.lines().nth(1).unwrap().remove_last().parse::<i32>().unwrap() as u32);
        let fear_op: FearOps = match monkey_data.lines().nth(2).unwrap().split_at("old") {
            x.conta
        };
    }

    // for monkey_data in data.split("\n\n") {
    //     let mut mid = Monkeyid(-1);
    //     let mut inventory = vec![];
    //     let mut fear_op = -1;
    //     let mut devisor = -1;
    //     let mut send_to_if_true = Monkeyid(-1);
    //     let mut send_to_if_false = Monkeyid(-1);
    //     let mut operator:FearOps = FearOps::Add;
    //
    //     for (indx, line) in monkey_data.lines().enumerate() {
    //         // let ellements = line.split_whitespace().for_each(|e| print!("{e}|"));
    //         // println!("{indx}{line}");
    //         if indx == 0 {
    //             mid = Monkeyid(
    //                 line.split_whitespace()
    //                     .nth(1)
    //                     .expect("mid should exist")
    //                     .remove_last()
    //                     .parse::<i32>()
    //                     .expect("mid should be parse to i32"),
    //             );
    //         } else if indx == 1 {
    //             inventory = line
    //                 .split_once(':')
    //                 .expect("second line should have inventory data")
    //                 .1
    //                 .split(", ")
    //                 .map(|i| {
    //                     i.trim()
    //                         .replace(",", "")
    //                         .parse::<i32>()
    //                         .expect("invent itms should parse to i32")
    //                 })
    //                 .collect::<Vec<i32>>();
    //         } else if indx == 2 {
    //             fear_op = line
    //                 .split_ascii_whitespace()
    //                 .last()
    //                 .expect("monkey should have fear multiplier")
    //                 .parse::<i32>()
    //                 .expect("should be able to parse to i32");
    //
    //             fear_op
    //         } else if indx == 3 {
    //             devisor = line
    //                 .split_ascii_whitespace()
    //                 .last()
    //                 .expect("monkey should have devisor")
    //                 .parse::<i32>()
    //                 .expect("should be able to parse to i32");
    //         } else if indx == 4 {
    //             send_to_if_true = Monkeyid(
    //                 line.split_ascii_whitespace()
    //                     .last()
    //                     .expect("if true send to monkey condition should exist")
    //                     .parse::<i32>()
    //                     .expect("should be able to parse to i32"),
    //             );
    //         } else if indx == 5 {
    //             send_to_if_false = Monkeyid(
    //                 line.split_ascii_whitespace()
    //                     .last()
    //                     .expect("if false send to monkey condition should exist")
    //                     .parse::<i32>()
    //                     .expect("should be able to parse to i32"),
    //             );
    //         } else {
    //             unreachable!();
    //         }
    //     }
    //
    //     let div_test = DivisibilityTest::construct_divisibility_test(
    //         devisor,
    //         send_to_if_true,
    //         send_to_if_false,
    //     );
    //
    //     let m = Monkey::construct_monkey(mid, div_test, inventory, fear_op);
    // }
    todo!()
}

// fn part_two(data: String) -> i32 {
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(read_data_from_name("dummy-data.txt")), 10605);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(read_data_from_name("dummy-data.txt")), TODO);
    // }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("{}", part_one(read_data_from_name("data.txt")));

    Ok(())
}
