use std::fs;

fn main() {
    let day = "05";
    println!("Day {}, Part 1 (test)", day);
    part_1(format!("day{}/input_test.txt", day).as_str());
    println!("\nDay {}, Part 1", day);
    part_1(format!("day{}/input.txt", day).as_str());

    println!("\nDay {}, Part 2 (test)", day);
    part_2(format!("day{}/input_test.txt", day).as_str());
    println!("\nDay {}, Part 2", day);
    part_2(format!("day{}/input.txt", day).as_str());
}

fn file_to_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

fn value_to_number(value: &str, true_value: &char) -> usize {
    let mut value_so_far = 0;
    let two: usize = 2;
    for (idx, c) in value.chars().rev().enumerate() {
        if c == *true_value {
            value_so_far += two.pow(idx as u32);
        }
    }
    value_so_far
}

fn lines_to_numbers(lines: Vec<String>) -> Vec<(usize, usize)> {
    lines
        .iter()
        .map(|line| {
            let first = value_to_number(&line[..7], &'B');
            let second = value_to_number(&line[7..], &'R');
            (first, second)
        })
        .collect()
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let numbers = lines_to_numbers(lines);

    let number_scores: Vec<usize> = numbers
        .iter()
        .map(|(first, second)| first * 8 + second)
        .collect();
    let result = number_scores.iter().max().unwrap();

    println!("The result is {}", result);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let numbers = lines_to_numbers(lines);

    // Collect all seats ids
    let mut number_scores: Vec<usize> = numbers
        .iter()
        .map(|(first, second)| first * 8 + second)
        .collect();
    number_scores.sort();

    // Find the missing seat id
    let mut result = 0;
    for (idx, number) in number_scores.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        if number_scores[idx - 1] + 1 != *number {
            result = number_scores[idx - 1] + 1;
            break;
        }
    }

    println!("The result is {}", result);
}
