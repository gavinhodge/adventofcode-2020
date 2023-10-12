use std::fs;

fn main() {
    println!("Part 1 (test)");
    part_1("day01/input_test.txt");
    println!("\nPart 1");
    part_1("day01/input.txt");

    println!("\nPart 2 (test)");
    part_2("day01/input_test.txt");
    println!("\nPart 2");
    part_2("day01/input.txt");
}

fn part_1(filename: &str) {
    // Parse each line into an integer in a vector
    let numbers: Vec<i32> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    // Find the two numbers that sum to 2020
    let mut result = 0;
    for i in &numbers {
        for j in &numbers {
            if i + j == 2020 {
                result = i * j;
                break;
            }
        }
    }

    println!("The result is {result}");
}

fn part_2(filename: &str) {
    // Parse each line into an integer in a vector
    let numbers: Vec<i32> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    // Find the three numbers that sum to 2020
    let mut result = 0;
    for i in &numbers {
        for j in &numbers {
            for k in &numbers {
                if i + j + k == 2020 {
                    result = i * j * k;
                    break;
                }
            }
        }
    }

    println!("The result is {result}");
}
