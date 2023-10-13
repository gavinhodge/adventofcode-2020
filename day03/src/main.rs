use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 (test)");
    part_1("day03/input_test.txt");
    println!("\nPart 1");
    part_1("day03/input.txt");

    println!("\nPart 2 (test)");
    part_2("day03/input_test.txt");
    println!("\nPart 2");
    part_2("day03/input.txt");
}

fn file_to_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

struct Grid {
    coords: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
}

fn lines_to_grid(lines: &[String]) -> Grid {
    let mut coords: HashMap<(usize, usize), char> = HashMap::new();
    for (line_idx, line) in lines.iter().enumerate() {
        for (row_idx, c) in line.chars().enumerate() {
            coords.insert((row_idx, line_idx), c);
        }
    }
    let width = coords.keys().map(|(x, _)| x).max().unwrap() + 1;
    let height = coords.keys().map(|(_, y)| y).max().unwrap() + 1;
    Grid {
        coords,
        width,
        height,
    }
}

fn char_at_coord(x: usize, y: usize, grid: &Grid) -> char {
    let x = x % grid.width;
    let y = y % grid.height;
    *grid.coords.get(&(x, y)).unwrap()
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let grid = lines_to_grid(&lines);

    let mut valid_count = 0;
    let mut my_coord = (0, 0);

    while my_coord.1 < grid.height {
        if char_at_coord(my_coord.0, my_coord.1, &grid) == '#' {
            valid_count += 1;
        }
        my_coord.0 += 3;
        my_coord.1 += 1;
    }

    println!("The result is {}", valid_count);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let grid = lines_to_grid(&lines);

    let vectors = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let totals = vectors
        .iter()
        .map(|(dx, dy)| {
            let mut valid_count = 0;
            let mut my_coord = (0, 0);

            while my_coord.1 < grid.height {
                if char_at_coord(my_coord.0, my_coord.1, &grid) == '#' {
                    valid_count += 1;
                }
                my_coord.0 += dx;
                my_coord.1 += dy;
            }

            valid_count
        })
        .collect::<Vec<usize>>();

    let answer: usize = totals
        .iter()
        .copied()
        .reduce(|a: usize, b: usize| a * b)
        .unwrap();

    println!("The result is {}", answer);
}
