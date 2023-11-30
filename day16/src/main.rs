use common::{file_to_lines, runner};

fn main() {
    runner("16", part_1, part_2);
}

type Range = (u64, u64);
type Ranges = Vec<Range>;

fn parse_lines(lines: &Vec<String>) -> (Ranges, Vec<u64>, Vec<Vec<u64>>) {
    let mut ranges = Vec::new();
    let mut my_ticket: Vec<u64> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u64>> = Vec::new();

    let mut section = 0;
    for line in lines {
        if line.is_empty() {
            // section += 1;
            continue;
        }
        if line.starts_with("your ticket") {
            section += 1;
            continue;
        }
        if line.starts_with("nearby tickets") {
            section += 1;
            continue;
        }
        match section {
            0 => {
                let mut parts = line.split(": ");
                parts.next().unwrap();
                let first_part = parts.next().unwrap();
                let mut ranges_parts = first_part.split(" or ");
                let first_range = ranges_parts.next().unwrap();
                let second_range = ranges_parts.next().unwrap();
                let mut first_range_parts = first_range.split('-');
                let first_range_start = first_range_parts.next().unwrap().parse::<u64>().unwrap();
                let first_range_end = first_range_parts.next().unwrap().parse::<u64>().unwrap();
                let mut second_range_parts = second_range.split('-');
                let second_range_start = second_range_parts.next().unwrap().parse::<u64>().unwrap();
                let second_range_end = second_range_parts.next().unwrap().parse::<u64>().unwrap();
                ranges.push((first_range_start, first_range_end));
                ranges.push((second_range_start, second_range_end));
            }
            1 => {
                my_ticket = line.split(',').map(|x| x.parse::<u64>().unwrap()).collect();
            }
            2 => {
                nearby_tickets.push(line.split(',').map(|x| x.parse::<u64>().unwrap()).collect());
            }
            _ => panic!("Invalid section"),
        }
    }

    (ranges, my_ticket, nearby_tickets)
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);

    let (ranges, _, nearby_tickets) = parse_lines(&lines);

    let mut error_rate = 0;
    for ticket in nearby_tickets {
        for value in ticket {
            let mut valid = false;
            for (start, end) in &ranges {
                if value >= *start && value <= *end {
                    valid = true;
                    break;
                }
            }
            if !valid {
                error_rate += value;
            }
        }
    }

    let answer = error_rate;
    println!("The answer is {}", answer);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);

    let (ranges, my_ticket, nearby_tickets) = parse_lines(&lines);

    // Find valid tickets
    let mut valid_tickets = Vec::new();
    for ticket in nearby_tickets {
        let mut valid = true;
        for value in &ticket {
            let mut valid_value = false;
            for (start, end) in &ranges {
                if value >= start && value <= end {
                    valid_value = true;
                    break;
                }
            }
            if !valid_value {
                valid = false;
                break;
            }
        }
        if valid {
            valid_tickets.push(ticket);
        }
    }

    // Put the ranges into pairs
    let mut range_pairs = Vec::new();
    for i in 0..ranges.len() {
        if i % 2 == 0 {
            range_pairs.push((ranges[i], ranges[i + 1]));
        }
    }

    // Find the possible fields for each column
    let mut possible_fields = Vec::new();
    for i in 0..my_ticket.len() {
        let mut possible_fields_for_column = Vec::new();
        for (range_idx, (start, end)) in range_pairs.iter().enumerate() {
            let mut valid = true;
            for ticket in &valid_tickets {
                let value = ticket[i];
                if !((start.0..=start.1).contains(&value) || (end.0..=end.1).contains(&value)) {
                    valid = false;
                    break;
                }
            }
            // Add to possible fields if not already there
            if valid && !possible_fields_for_column.contains(&range_idx) {
                possible_fields_for_column.push(range_idx);
            }
        }
        possible_fields.push(possible_fields_for_column);
    }

    // Find the field for each column
    let mut fields = vec![0; my_ticket.len()];
    let mut found = 0;
    while found < my_ticket.len() {
        for i in 0..possible_fields.len() {
            if possible_fields[i].len() == 1 {
                let field = possible_fields[i][0];
                fields[i] = field;
                found += 1;
                for j in possible_fields.iter_mut() {
                    if j.contains(&field) {
                        j.retain(|&x| x != field);
                    }
                }
            }
        }
    }

    // Calculate the answer
    let mut answer = 1;
    for i in 0..fields.len() {
        if fields[i] < 6 {
            answer *= my_ticket[i];
        }
    }

    println!("The answer is {}", answer);
}
