use std::collections::HashMap;

use common::{file_to_lines, runner};

fn main() {
    runner("14", part_1, part_2);
}

fn parse_mask(line: &str) -> (u64, u64) {
    let mut ones = 0;
    let mut zeros = 0;
    for (i, c) in line.chars().rev().enumerate() {
        match c {
            'X' => {
                zeros |= 1 << i;
            }
            '1' => {
                ones |= 1 << i;
                zeros |= 1 << i;
            }
            '0' => {
                //ones |= 1 << i;
            }
            ' ' => {
                break;
            }
            _ => panic!("Invalid mask character"),
        }
    }
    (ones, zeros)
}

fn parse_mem_instruction(line: &str) -> (u64, u64) {
    let mut parts = line.split(" = ");
    let first_part = parts.next().unwrap();
    let address = first_part[4..first_part.len() - 1].parse::<u64>().unwrap();
    let value = parts.next().unwrap().parse::<u64>().unwrap();
    (address, value)
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);

    let mut mask_ones = 0;
    let mut mask_zeros = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in lines {
        if line.starts_with("mask") {
            (mask_ones, mask_zeros) = parse_mask(&line[7..]);
        } else {
            let (address, value) = parse_mem_instruction(&line);
            memory.insert(address, (value | mask_ones) & mask_zeros);
        }
    }

    let answer = memory.values().sum::<u64>();
    println!("The answer is {}", answer);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);

    let mut mask = "";
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in &lines {
        if line.starts_with("mask") {
            mask = &line[7..];
        } else {
            let (address, value) = parse_mem_instruction(line);
            let mut addresses = vec![address];
            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    'X' => {
                        let mut new_addresses = Vec::new();
                        for address in addresses {
                            new_addresses.push(address | (1 << i));
                            new_addresses.push(address & !(1 << i));
                        }
                        addresses = new_addresses;
                    }
                    '1' => {
                        for address in &mut addresses {
                            *address |= 1 << i;
                        }
                    }
                    '0' => {}
                    _ => panic!("Invalid mask character"),
                }
            }
            for address in addresses {
                memory.insert(address, value);
            }
        }
    }

    let answer = memory.values().sum::<u64>();

    println!("The answer is {}", answer);
}
