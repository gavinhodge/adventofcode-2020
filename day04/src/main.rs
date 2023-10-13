use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    println!("Part 1 (test)");
    part_1("day04/input_test.txt");
    println!("\nPart 1");
    part_1("day04/input.txt");

    println!("\nPart 2 (test)");
    part_2("day04/input_test.txt");
    println!("\nPart 2");
    part_2("day04/input.txt");
}

fn file_to_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect()
}

fn lines_to_passports(lines: &Vec<String>) -> Vec<HashMap<String, String>> {
    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    let mut passport: HashMap<String, String> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            passports.push(passport);
            passport = HashMap::new();
        } else {
            for field in line.split_whitespace() {
                let mut field = field.split(':');
                let key = field.next().unwrap();
                let value = field.next().unwrap();
                passport.insert(key.to_string(), value.to_string());
            }
        }
    }
    passports.push(passport);
    passports
}

fn validate_passport(passport: &HashMap<String, String>) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in &required_fields {
        if !passport.contains_key(*field) {
            return false;
        }
    }
    true
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);
    let passports = lines_to_passports(&lines);

    println!("There are {} passports", passports.len());

    let mut valid_count = 0;
    for passport in &passports {
        if validate_passport(passport) {
            valid_count += 1;
        }
    }

    println!("The result is {}", valid_count);
}

struct Regexes {
    byr: Regex,
    iyr: Regex,
    eyr: Regex,
    hgt: Regex,
    hcl: Regex,
    ecl: Regex,
    pid: Regex,
}

fn validate_passport_values(passport: &HashMap<String, String>, regexes: &Regexes) -> bool {
    let byr = passport.get("byr").unwrap();
    if !regexes.byr.is_match(byr) {
        return false;
    }
    let byr_int = byr.parse::<i32>().unwrap();
    if !(1920..=2002).contains(&byr_int) {
        return false;
    }

    let iyr = passport.get("iyr").unwrap();
    if !regexes.iyr.is_match(iyr) {
        return false;
    }
    let iyr_int = iyr.parse::<i32>().unwrap();
    if !(2010..=2020).contains(&iyr_int) {
        return false;
    }

    let eyr = passport.get("eyr").unwrap();
    if !regexes.eyr.is_match(eyr) {
        return false;
    }
    let eyr_int = eyr.parse::<i32>().unwrap();
    if !(2020..=2030).contains(&eyr_int) {
        return false;
    }

    let hgt = passport.get("hgt").unwrap();
    if !regexes.hgt.is_match(hgt) {
        return false;
    }
    let hgt_int = hgt[..hgt.len() - 2].parse::<i32>().unwrap();
    let hgt_unit = &hgt[hgt.len() - 2..];
    if hgt_unit == "cm" && !(150..=193).contains(&hgt_int) {
        return false;
    }
    if hgt_unit == "in" && !(59..=76).contains(&hgt_int) {
        return false;
    }

    let hcl = passport.get("hcl").unwrap();
    if !regexes.hcl.is_match(hcl) {
        return false;
    }

    let ecl = passport.get("ecl").unwrap();
    if !regexes.ecl.is_match(ecl) {
        return false;
    }

    let pid = passport.get("pid").unwrap();
    if !regexes.pid.is_match(pid) {
        return false;
    }

    true
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);
    let passports = lines_to_passports(&lines);

    let regexes = Regexes {
        byr: Regex::new(r"^\d{4}$").unwrap(),
        iyr: Regex::new(r"^\d{4}$").unwrap(),
        eyr: Regex::new(r"^\d{4}$").unwrap(),
        hgt: Regex::new(r"^\d{2,3}(cm|in)$").unwrap(),
        hcl: Regex::new(r"^#[0-9a-f]{6}$").unwrap(),
        ecl: Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap(),
        pid: Regex::new(r"^\d{9}$").unwrap(),
    };

    let mut valid_count = 0;
    for passport in &passports {
        if validate_passport(passport) && validate_passport_values(passport, &regexes) {
            valid_count += 1;
        }
    }

    println!("The result is {}", valid_count);
}
