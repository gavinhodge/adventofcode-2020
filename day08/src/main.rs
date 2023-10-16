use std::collections::HashSet;

use common::{file_to_lines, runner};

fn main() {
    runner("08", part_1, part_2);
}

#[derive(Clone)]
struct Instruction {
    operation: String,
    argument: i32,
}

fn lines_to_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        instructions.push(Instruction {
            operation: parts[0].to_string(),
            argument: parts[1].parse::<i32>().unwrap(),
        });
    }
    instructions
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let instructions = lines_to_instructions(&lines);

    let mut accumulator = 0;
    let mut instruction_pointer = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    while !visited.contains(&instruction_pointer) {
        visited.insert(instruction_pointer);

        let instruction = &instructions[instruction_pointer];
        match instruction.operation.as_str() {
            "acc" => {
                accumulator += instruction.argument;
                instruction_pointer += 1;
            }
            "jmp" => {
                instruction_pointer = (instruction_pointer as i32 + instruction.argument) as usize;
            }
            "nop" => {
                instruction_pointer += 1;
            }
            _ => panic!("Unknown operation {}", instruction.operation),
        }
    }

    let result = accumulator;

    println!("The result is {}", result);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let instructions = lines_to_instructions(&lines);

    for (idx, instruction) in instructions.iter().enumerate() {
        if instruction.operation == "acc" {
            continue;
        }

        let mut modified_instructions = instructions.clone();

        if instruction.operation == "jmp" {
            modified_instructions[idx].operation = "nop".to_string();
        } else {
            modified_instructions[idx].operation = "jmp".to_string();
        }

        let mut accumulator = 0;
        let mut instruction_pointer = 0;
        let mut visited: HashSet<usize> = HashSet::new();

        while !visited.contains(&instruction_pointer) {
            visited.insert(instruction_pointer);

            let instruction = &modified_instructions[instruction_pointer];
            match instruction.operation.as_str() {
                "acc" => {
                    accumulator += instruction.argument;
                    instruction_pointer += 1;
                }
                "jmp" => {
                    instruction_pointer =
                        (instruction_pointer as i32 + instruction.argument) as usize;
                }
                "nop" => {
                    instruction_pointer += 1;
                }
                _ => panic!("Unknown operation {}", instruction.operation),
            }

            if instruction_pointer == instructions.len() {
                let result = accumulator;
                println!("The result is {}", result);
                return;
            }
        }
    }
}
