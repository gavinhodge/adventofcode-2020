use common::{file_to_lines, runner};

fn main() {
    runner("12", part_1, part_2);
}

#[derive(Clone)]
struct Turtle {
    x: i32,
    y: i32,
    dir: i32,
}

fn next_turtle_part_1(turtle: &Turtle, instruction: &str) -> Turtle {
    let mut turtle = turtle.clone();
    let (action, value) = instruction.split_at(1);
    let value = value.parse::<i32>().unwrap();
    match action {
        "N" => turtle.y += value,
        "S" => turtle.y -= value,
        "E" => turtle.x += value,
        "W" => turtle.x -= value,
        "L" => turtle.dir = (turtle.dir + value) % 360,
        "R" => turtle.dir = (turtle.dir - value + 360) % 360,
        "F" => match turtle.dir {
            0 => turtle.x += value,
            90 => turtle.y += value,
            180 => turtle.x -= value,
            270 => turtle.y -= value,
            _ => panic!("Invalid direction {}", turtle.dir),
        },
        _ => panic!("Invalid action {}", action),
    }
    turtle
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);

    let mut turtle = Turtle {
        x: 0,
        y: 0,
        dir: 0, // East
    };

    for line in lines {
        turtle = next_turtle_part_1(&turtle, &line);
    }

    let answer = turtle.x.abs() + turtle.y.abs();

    println!("The answer is {}", answer);
}

struct ShipAndWaypoint {
    ship: Turtle,
    waypoint: Turtle,
}

fn next_state_part_2(state: &ShipAndWaypoint, instruction: &str) -> ShipAndWaypoint {
    let mut ship = state.ship.clone();
    let mut waypoint = state.waypoint.clone();
    let (action, value) = instruction.split_at(1);
    let value = value.parse::<i32>().unwrap();
    match action {
        "N" => waypoint.y += value,
        "S" => waypoint.y -= value,
        "E" => waypoint.x += value,
        "W" => waypoint.x -= value,
        "L" => {
            waypoint.dir = (waypoint.dir + value) % 360;
            // Also rotate the coords
            match value {
                90 => {
                    let tmp = waypoint.x;
                    waypoint.x = -waypoint.y;
                    waypoint.y = tmp;
                }
                180 => {
                    waypoint.x = -waypoint.x;
                    waypoint.y = -waypoint.y;
                }
                270 => {
                    let tmp = waypoint.x;
                    waypoint.x = waypoint.y;
                    waypoint.y = -tmp;
                }
                _ => panic!("Invalid rotation {}", value),
            }
        }
        "R" => {
            waypoint.dir = (waypoint.dir - value + 360) % 360;
            // Also rotate the coords
            match value {
                90 => {
                    let tmp = waypoint.x;
                    waypoint.x = waypoint.y;
                    waypoint.y = -tmp;
                }
                180 => {
                    waypoint.x = -waypoint.x;
                    waypoint.y = -waypoint.y;
                }
                270 => {
                    let tmp = waypoint.x;
                    waypoint.x = -waypoint.y;
                    waypoint.y = tmp;
                }
                _ => panic!("Invalid rotation {}", value),
            }
        }
        "F" => {
            ship.x += waypoint.x * value;
            ship.y += waypoint.y * value;
        }
        _ => panic!("Invalid action {}", action),
    }
    ShipAndWaypoint { ship, waypoint }
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);

    let waypoint = Turtle {
        x: 10,
        y: 1,
        dir: 0, // East
    };
    let ship = Turtle {
        x: 0,
        y: 0,
        dir: 0, // East
    };
    let mut state = ShipAndWaypoint { ship, waypoint };

    for line in lines {
        state = next_state_part_2(&state, &line);
    }

    let answer = state.ship.x.abs() + state.ship.y.abs();

    println!("The answer is {}", answer);
}
