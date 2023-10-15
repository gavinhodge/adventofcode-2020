use std::collections::HashSet;

use common::{file_to_lines, runner};

fn main() {
    runner("06", part_1, part_2);
}

struct Group {
    overall_answers: HashSet<char>,
    people: Vec<HashSet<char>>,
}

fn lines_to_groups(lines: &Vec<String>) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let mut people: Vec<HashSet<char>> = Vec::new();
    let mut group_answers: HashSet<char> = HashSet::new();

    for line in lines {
        if line.is_empty() {
            groups.push(Group {
                overall_answers: group_answers.clone(),
                people: people.clone(),
            });
            group_answers.clear();
            people.clear();
            continue;
        }

        let mut person_answers: HashSet<char> = HashSet::new();
        for c in line.chars() {
            person_answers.insert(c);
            group_answers.insert(c);
        }
        people.push(person_answers);
    }

    groups.push(Group {
        overall_answers: group_answers.clone(),
        people,
    });

    groups
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let groups = lines_to_groups(&lines);

    let result = groups
        .iter()
        .map(|group| group.overall_answers.len())
        .sum::<usize>();

    println!("The result is {}", result);
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let groups = lines_to_groups(&lines);

    let mut result = 0;

    for group in &groups {
        let mut common_answers = group.overall_answers.clone();
        // println!("Common answers: {:?}", common_answers);
        for person in &group.people {
            common_answers = common_answers.intersection(person).copied().collect();
            // println!("Person: {:?}, Common answers: {:?}", person, common_answers);
        }
        // println!("Group: {:?}", common_answers);
        result += common_answers.len();
    }

    println!("The result is {}", result);
}
