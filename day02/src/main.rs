use std::fs;

fn main() {
    println!("Part 1 (test)");
    part_1("day02/input_test.txt");
    println!("\nPart 1");
    part_1("day02/input.txt");

    println!("\nPart 2 (test)");
    part_2("day02/input_test.txt");
    println!("\nPart 2");
    part_2("day02/input.txt");
}

fn part_1(filename: &str) {
    // Parse each line into a vector
    let lines: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect();

    // Parse each line into a tuple
    let mut data: Vec<(u32, u32, char, String)> = Vec::new();
    for line in &lines {
        let mut split = line.split(' ');
        let mut range = split.next().unwrap().split('-');
        let min = range.next().unwrap().parse::<u32>().unwrap();
        let max = range.next().unwrap().parse::<u32>().unwrap();
        let letter = split.next().unwrap().chars().next().unwrap();
        let password = split.next().unwrap().to_string();
        data.push((min, max, letter, password));
    }

    // Validate each password
    let mut valid_count = 0;
    for (min, max, letter, password) in &data {
        let mut count = 0;
        for c in password.chars() {
            if &c == letter {
                count += 1;
            }
        }
        if count >= *min && count <= *max {
            valid_count += 1;
        }
    }

    println!("The result is {}", valid_count);
}

fn part_2(filename: &str) {
    // Parse each line into a vector
    let lines: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect();

    // Parse each line into a tuple
    let mut data: Vec<(usize, usize, char, String)> = Vec::new();
    for line in &lines {
        let mut split = line.split(' ');
        let mut range = split.next().unwrap().split('-');
        let min = range.next().unwrap().parse::<usize>().unwrap();
        let max = range.next().unwrap().parse::<usize>().unwrap();
        let letter = split.next().unwrap().chars().next().unwrap();
        let password = split.next().unwrap().to_string();
        data.push((min, max, letter, password));
    }

    // Validate each password
    let mut valid_count = 0;
    for (min, max, letter, password) in &data {
        let char1 = password.chars().nth(*min - 1).unwrap();
        let char2 = password.chars().nth(*max - 1).unwrap();
        if (char1 == *letter || char2 == *letter) && char1 != char2 {
            valid_count += 1;
        }
    }

    println!("The result is {}", valid_count);
}
