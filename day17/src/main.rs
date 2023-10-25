use itertools::iproduct;
use std::collections::HashSet;

use common::{file_to_lines, runner};

fn main() {
    runner("17", part_1, part_2);
}

type Coord3D = (i32, i32, i32);

#[derive(Clone)]
struct Grid3D {
    coords: HashSet<Coord3D>,
    min_coord: Coord3D,
    max_coord: Coord3D,
}

fn make_grid_from_coords(coords: &HashSet<Coord3D>) -> Grid3D {
    if coords.is_empty() {
        return Grid3D {
            coords: coords.clone(),
            min_coord: (0, 0, 0),
            max_coord: (0, 0, 0),
        };
    }

    Grid3D {
        coords: coords.clone(),
        min_coord: (
            coords.iter().map(|coord| coord.0).min().unwrap(),
            coords.iter().map(|coord| coord.1).min().unwrap(),
            coords.iter().map(|coord| coord.2).min().unwrap(),
        ),
        max_coord: (
            coords.iter().map(|coord| coord.0).max().unwrap(),
            coords.iter().map(|coord| coord.1).max().unwrap(),
            coords.iter().map(|coord| coord.2).max().unwrap(),
        ),
    }
}

fn lines_to_grid(lines: &[String]) -> Grid3D {
    let mut coords: HashSet<Coord3D> = HashSet::new();

    for (y, line) in lines.iter().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                coords.insert((x as i32, y as i32, 0));
            }
        }
    }

    make_grid_from_coords(&coords)
}

fn next_grid(grid: &Grid3D) -> Grid3D {
    let mut new_coords: HashSet<Coord3D> = HashSet::new();

    for x in grid.min_coord.0 - 1..=grid.max_coord.0 + 1 {
        for y in grid.min_coord.1 - 1..=grid.max_coord.1 + 1 {
            for z in grid.min_coord.2 - 1..=grid.max_coord.2 + 1 {
                let coord = (x, y, z);
                let count = count_adjacent(&grid.coords, coord);
                if grid.coords.contains(&coord) {
                    if count == 2 || count == 3 {
                        new_coords.insert(coord);
                    }
                } else if count == 3 {
                    new_coords.insert(coord);
                }
            }
        }
    }

    make_grid_from_coords(&new_coords)
}

fn count_adjacent(coords: &HashSet<Coord3D>, coord: Coord3D) -> i32 {
    let distances: [i32; 3] = [-1, 0, 1];
    let mut count = 0;
    //println!("{:?}", distances.iter().permutations(3).collect::<Vec<_>>());
    for d in iproduct!(distances, distances, distances) {
        let (dx, dy, dz) = d;
        let new_coord = (coord.0 + dx, coord.1 + dy, coord.2 + dz);
        if coord != new_coord && coords.contains(&new_coord) {
            count += 1;
        }
    }
    count
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let mut grid = lines_to_grid(&lines);

    for _ in 1..=6 {
        grid = next_grid(&grid);
    }

    let result = grid.coords.len();
    println!("The result is {}", result);
}

type Coord4D = (i32, i32, i32, i32);

#[derive(Clone)]
struct Grid4D {
    coords: HashSet<Coord4D>,
    min_coord: Coord4D,
    max_coord: Coord4D,
}

fn make_grid_from_coords_4d(coords: &HashSet<Coord4D>) -> Grid4D {
    if coords.is_empty() {
        return Grid4D {
            coords: coords.clone(),
            min_coord: (0, 0, 0, 0),
            max_coord: (0, 0, 0, 0),
        };
    }

    Grid4D {
        coords: coords.clone(),
        min_coord: (
            coords.iter().map(|coord| coord.0).min().unwrap(),
            coords.iter().map(|coord| coord.1).min().unwrap(),
            coords.iter().map(|coord| coord.2).min().unwrap(),
            coords.iter().map(|coord| coord.3).min().unwrap(),
        ),
        max_coord: (
            coords.iter().map(|coord| coord.0).max().unwrap(),
            coords.iter().map(|coord| coord.1).max().unwrap(),
            coords.iter().map(|coord| coord.2).max().unwrap(),
            coords.iter().map(|coord| coord.3).max().unwrap(),
        ),
    }
}

fn lines_to_grid_4d(lines: &[String]) -> Grid4D {
    let mut coords: HashSet<Coord4D> = HashSet::new();

    for (y, line) in lines.iter().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                coords.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    make_grid_from_coords_4d(&coords)
}

fn next_grid_4d(grid: &Grid4D) -> Grid4D {
    let mut new_coords: HashSet<Coord4D> = HashSet::new();

    for x in grid.min_coord.0 - 1..=grid.max_coord.0 + 1 {
        for y in grid.min_coord.1 - 1..=grid.max_coord.1 + 1 {
            for z in grid.min_coord.2 - 1..=grid.max_coord.2 + 1 {
                for w in grid.min_coord.3 - 1..=grid.max_coord.3 + 1 {
                    let coord = (x, y, z, w);
                    let count = count_adjacent_4d(&grid.coords, coord);
                    if grid.coords.contains(&coord) {
                        if count == 2 || count == 3 {
                            new_coords.insert(coord);
                        }
                    } else if count == 3 {
                        new_coords.insert(coord);
                    }
                }
            }
        }
    }

    make_grid_from_coords_4d(&new_coords)
}

fn count_adjacent_4d(coords: &HashSet<Coord4D>, coord: Coord4D) -> i32 {
    let distances: [i32; 3] = [-1, 0, 1];
    let mut count = 0;
    //println!("{:?}", distances.iter().permutations(3).collect::<Vec<_>>());
    for d in iproduct!(distances, distances, distances, distances) {
        let (dx, dy, dz, dw) = d;
        let new_coord = (coord.0 + dx, coord.1 + dy, coord.2 + dz, coord.3 + dw);
        if coord != new_coord && coords.contains(&new_coord) {
            count += 1;
        }
    }
    count
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let mut grid = lines_to_grid_4d(&lines);

    for _ in 1..=6 {
        grid = next_grid_4d(&grid);
    }

    let result = grid.coords.len();
    println!("The result is {}", result);
}
