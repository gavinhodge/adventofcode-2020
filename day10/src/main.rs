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

    // Sort, then loop through and count the differences
    let mut numbers = numbers.clone();
    numbers.sort();

    let mut one_diff = 0;
    let mut three_diff = 0;
    let mut last = 0;
    for n in numbers {
        match n - last {
            1 => one_diff += 1,
            3 => three_diff += 1,
            _ => (),
        }
        last = n;
    }

    let answer = one_diff * (three_diff + 1);
    println!("One diff: {}, Three diff: {}", one_diff, three_diff);
    println!("The answer is {}", answer);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let numbers = lines_to_vec(&lines);

    // Sort
    let mut numbers = numbers.clone();
    numbers.sort();

    // Add the start and end
    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);

    // Find the number of ways to get to each number
    let mut ways: Vec<i64> = vec![0; numbers.len()];
    ways[0] = 1;
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[j] - numbers[i] <= 3 {
                ways[j] += ways[i];
            } else {
                break;
            }
        }
    }

    let answer = ways.last().unwrap();

    println!("The answer is {}", answer);
}
