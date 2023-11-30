use std::collections::HashMap;

use common::{file_to_lines, runner};

fn main() {
    runner("15", part_1, part_2);
}

fn lines_to_ints(lines: Vec<String>) -> Vec<u64> {
    lines[0]
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);

    let numbers = lines_to_ints(lines);
    let turns = 2020;
    let mut turn_history = numbers
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect::<HashMap<u64, usize>>();

    let mut last_number = *numbers.last().unwrap();
    for turn in numbers.len() + 1..turns + 1 {
        let next_number = match turn_history.get(&last_number) {
            Some(last_turn) => (turn - 1 - last_turn) as u64,
            None => 0,
        };
        turn_history.insert(last_number, turn - 1);
        last_number = next_number;
    }

    let answer = last_number;
    println!("The answer is {}", answer);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);

    let numbers = lines_to_ints(lines);
    let turns = 30000000;
    let mut turn_history = numbers
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect::<HashMap<u64, usize>>();

    let mut last_number = *numbers.last().unwrap();
    for turn in numbers.len() + 1..turns + 1 {
        let next_number = match turn_history.get(&last_number) {
            Some(last_turn) => (turn - 1 - last_turn) as u64,
            None => 0,
        };
        turn_history.insert(last_number, turn - 1);
        last_number = next_number;
    }

    let answer = last_number;
    println!("The answer is {}", answer);
}
