

use common::{file_to_lines, runner};

fn main() {
    runner("10", part_1, part_2);
}

fn lines_to_vec(lines: &[String]) -> Vec<i64> {
    lines.iter().map(|s| s.parse::<i64>().unwrap()).collect()
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let numbers = lines_to_vec(&lines);

    let window_size = if filename.contains("test") {5} else {25};

    let mut invalid_number = 0;
    let mut current_index = window_size;

    while current_index < numbers.len() {
        let mut found = false;
        let current_number = numbers[current_index];

        'outer: for i in current_index - window_size..current_index {
            for j in current_index - window_size..current_index {
                if i == j {
                    continue;
                }

                if numbers[i] + numbers[j] == current_number {
                    found = true;
                    break 'outer;
                }
            }
        }

        if !found {
            invalid_number = current_number;
            break;
        }

        current_index += 1;
    }

    println!("The answer is {}", invalid_number);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let numbers = lines_to_vec(&lines);
 
    let search_num = if filename.contains("test") {127} else {22477624};
    let mut start_idx = 0;
    while start_idx < numbers.len() {
        let mut current_sum = 0;
        let mut current_idx = start_idx;
        while current_sum < search_num {
            current_sum += numbers[current_idx];
            current_idx += 1;
        }

        if current_sum == search_num {
            let min = numbers[start_idx..current_idx].iter().min().unwrap();
            let max = numbers[start_idx..current_idx].iter().max().unwrap();
            println!("The answer is {} + {} = {}", min, max, min + max);
            break;
        }

        start_idx += 1;
    }
}
