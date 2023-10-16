use std::collections::{HashMap, HashSet};

use common::{file_to_lines, runner};

fn main() {
    runner("07", part_1, part_2);
}

struct Rules {
    rules: HashMap<String, Vec<(usize, String)>>,
}

fn lines_to_rules(lines: &Vec<String>) -> Rules {
    let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for line in lines {
        let mut parts = line.split(" bags contain ");
        let container = parts.next().unwrap().to_string();
        let contents = parts.next().unwrap();

        let mut contained: Vec<(usize, String)> = Vec::new();
        if contents != "no other bags." {
            for content in contents.split(", ") {
                let mut content_parts = content.split(" ");
                let count = content_parts.next().unwrap().parse::<usize>().unwrap();
                let color = format!(
                    "{} {}",
                    content_parts.next().unwrap(),
                    content_parts.next().unwrap()
                );
                contained.push((count, color));
            }
        }

        rules.insert(container, contained);
    }

    Rules { rules }
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let rules = lines_to_rules(&lines);

    let mut contained_by: HashMap<String, HashSet<String>> = HashMap::new();
    for rule in rules.rules {
        for (_, contained) in rule.1 {
            if !contained_by.contains_key(&contained) {
                contained_by.insert(contained.clone(), HashSet::new());
            }
            contained_by
                .get_mut(&contained)
                .unwrap()
                .insert(rule.0.clone());
        }
    }

    // println!("{:?}", contained_by);

    let mut containers: Vec<String> = Vec::new();
    let mut to_check: Vec<String> = Vec::new();
    to_check.push("shiny gold".to_string());
    while let Some(color) = to_check.pop() {
        if !contained_by.contains_key(&color) {
            continue;
        }

        for container in contained_by.get(&color).unwrap() {
            if !containers.contains(container) {
                containers.push(container.clone());
                to_check.push(container.clone());
            }
        }
    }

    let result = containers.len();

    println!("The result is {}", result);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let rules = lines_to_rules(&lines);

    let mut bag_totals: HashMap<String, usize> = HashMap::new();

    while bag_totals.len() != rules.rules.len() {
        'outer: for rule in &rules.rules {
            if bag_totals.contains_key(rule.0) {
                continue;
            }

            let mut total = 1;
            for contained in rule.1 {
                if !bag_totals.contains_key(&contained.1) {
                    continue 'outer;
                }

                total += contained.0 * bag_totals.get(&contained.1).unwrap();
            }

            bag_totals.insert(rule.0.clone(), total);
        }
    }

    let result = bag_totals.get("shiny gold").unwrap() - 1;

    println!("The result is {}", result);
}
