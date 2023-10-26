use std::collections::HashMap;

use common::{file_to_lines, runner};

fn main() {
    runner("11", part_1, part_2);
}

type Coord2D = (i32, i32);
type Grid = HashMap<Coord2D, char>;

fn lines_to_grid(lines: &[String]) -> Grid {
    let mut grid = Grid::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c);
        }
    }
    grid
}

fn next_grid_part_1(grid: &Grid) -> Grid {
    let mut next = Grid::new();
    for (coord, c) in grid.iter() {
        let mut occupied = 0;
        for x in coord.0 - 1..coord.0 + 2 {
            for y in coord.1 - 1..coord.1 + 2 {
                if x == coord.0 && y == coord.1 {
                    continue;
                }
                if let Some(c) = grid.get(&(x, y)) {
                    if *c == '#' {
                        occupied += 1;
                    }
                }
            }
        }
        if *c == 'L' && occupied == 0 {
            next.insert(*coord, '#');
        } else if *c == '#' && occupied >= 4 {
            next.insert(*coord, 'L');
        } else {
            next.insert(*coord, *c);
        }
    }
    next
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let grid = lines_to_grid(&lines);

    let mut prev = grid;
    let mut next = next_grid_part_1(&prev);
    while prev != next {
        prev = next;
        next = next_grid_part_1(&prev);
    }

    // Count occupied seats
    let answer = next.values().filter(|c| **c == '#').count();

    println!("The answer is {}", answer);
}

fn visible_seat_count(grid: &Grid, coord: Coord2D) -> i32 {
    if grid.get(&coord).unwrap() == &'.' {
        return 0; // No point counting if we're on the floor
    }

    let mut count = 0;

    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for dir in directions.iter() {
        let mut x = coord.0 + dir.0;
        let mut y = coord.1 + dir.1;
        while let Some(c) = grid.get(&(x, y)) {
            if *c == '#' {
                count += 1;
                break;
            } else if *c == 'L' {
                break;
            }
            x += dir.0;
            y += dir.1;
        }
    }

    count
}

fn next_grid_part_2(grid: &Grid) -> Grid {
    let mut next = Grid::new();
    for (coord, c) in grid.iter() {
        let occupied = visible_seat_count(grid, *coord);
        if *c == 'L' && occupied == 0 {
            next.insert(*coord, '#');
        } else if *c == '#' && occupied >= 5 {
            next.insert(*coord, 'L');
        } else {
            next.insert(*coord, *c);
        }
    }
    next
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let grid = lines_to_grid(&lines);

    let mut prev = grid;
    let mut next = next_grid_part_2(&prev);
    while prev != next {
        prev = next;
        next = next_grid_part_2(&prev);
    }

    // Count occupied seats
    let answer = next.values().filter(|c| **c == '#').count();

    println!("The answer is {}", answer);
}
