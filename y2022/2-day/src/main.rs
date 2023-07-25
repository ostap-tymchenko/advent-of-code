use core::panic;
use std::{fs, path::Path};

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data parse fail")
}

#[derive(PartialEq)]
enum Plays{
    Rock,
    Paper,
    Scissors,
}

fn match_play(play: char) -> Plays {
    match play{
        'X' | 'A' => Plays::Rock,
        'Y' | 'B' => Plays::Paper,
        'Z' | 'C' => Plays::Scissors,
        _ => panic!("unreachable state reached"),
    }
}

pub fn part_one() {
    let data = read_data(Path::new("./data.txt"));

    let mut points = 0;

    for round in data.lines() {
        let opponent = round.chars().nth(0).expect("bad input for opponent");
        let player = round.chars().nth(2).expect("bad input for player");

        let opponent = match_play(opponent);
        let player = match_play(player);

        let play_points = match player {
            Plays::Rock => 1,
            Plays::Paper => 2,
            Plays::Scissors => 3,
        };

        let win_points: i32;
        if opponent == player {
            win_points = 3;
        } else if opponent == Plays::Rock {
            if player == Plays::Paper {
                win_points = 6
            } else {
                win_points = 0;
            }
        } else if opponent == Plays::Paper {
            if player == Plays::Scissors {
                win_points = 6
            } else {
                win_points = 0;
            }
        } else if opponent == Plays::Scissors {
            if player == Plays::Rock {
                win_points = 6
            } else {
                win_points = 0;
            }
        } else {
            panic!();
        }

        points = points + play_points + win_points;
    }

    println!("Part one solution: {points}");
}

pub fn part_two() {
    let data = read_data(Path::new("./data.txt"));

    let mut points = 0;

    for round in data.lines() {
        let opponent = round.chars().nth(0).expect("bad input for opponent");
        let outcome = round.chars().nth(2).expect("bad input for outcome");

        let win_points = match outcome {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!(),
        };

        let opponent = match_play(opponent);

        #[derive(PartialEq)]
        enum Outcomes {
            Loss,
            Draw,
            Win
        }

        let outcome = match outcome {
            'X' => Outcomes::Loss,
            'Y' => Outcomes::Draw,
            'Z' => Outcomes::Win,
            _ => panic!()
        };

        let play_points: i32;

        if outcome == Outcomes::Draw {
            play_points = match opponent {
                Plays::Rock => 1,
                Plays::Paper => 2,
                Plays::Scissors => 3,
            }
        } else if opponent == Plays::Rock {
            if outcome == Outcomes::Win {
                play_points = 2;
            } else if outcome == Outcomes::Loss {
                play_points = 3;
            } else {
                panic!()
            }
        } else if opponent == Plays::Paper{
            if outcome == Outcomes::Win {
                play_points = 3;
            } else if outcome == Outcomes::Loss {
                play_points = 1;
            } else {
                panic!()
            }
        } else if opponent == Plays::Scissors{
            if outcome == Outcomes::Win {
                play_points = 1;
            } else if outcome == Outcomes::Loss {
                play_points = 2;
            } else {
                panic!()
            }
        } else {
            panic!();
        }

        points = points + play_points + win_points;
    }
    println!("Part two solution: {}", points);
}

pub fn main() {
    part_one();
    part_two();
}
