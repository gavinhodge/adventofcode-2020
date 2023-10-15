use std::fs;

pub fn file_to_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

pub fn runner(day: &str, part_1: fn(&str), part_2: fn(&str)) {
    println!("Day {}, Part 1 (test)", day);
    part_1(format!("./day{}/input_test.txt", day).as_str());
    println!("\nDay {}, Part 1", day);
    part_1(format!("./day{}/input.txt", day).as_str());

    println!("\nDay {}, Part 2 (test)", day);
    part_2(format!("./day{}/input_test.txt", day).as_str());
    println!("\nDay {}, Part 2", day);
    part_2(format!("./day{}/input.txt", day).as_str());
}
