use common::{file_to_lines, runner};

fn main() {
    runner("13", part_1, part_2);
}

fn part_1(filename: &str) {
    let lines = file_to_lines(filename);

    let start_time = lines[0].parse::<i32>().unwrap();
    let bus_ids = lines[1]
        .split(",")
        .filter(|&id| id != "x")
        .map(|id| id.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut min_wait = i32::MAX;
    let mut min_bus_id = 0;
    for bus_id in bus_ids {
        let wait = bus_id - (start_time % bus_id);
        if wait < min_wait {
            min_wait = wait;
            min_bus_id = bus_id;
        }
    }

    let answer = min_wait * min_bus_id;

    println!("The answer is {}", answer);
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn part_2(filename: &str) {
    let lines = file_to_lines(filename);

    // Parse but include the x as a placeholder
    let bus_ids = lines[1]
        .split(",")
        .map(|id| id.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();

    // We're going to use the Chinese Remainder Theorem to solve this
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    // https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let mut n = 1;
    for bus_id in &bus_ids {
        if *bus_id != 0 {
            n *= bus_id;
        }
    }

    let mut sum = 0;
    for (i, bus_id) in bus_ids.iter().enumerate() {
        if *bus_id != 0 {
            let a = *bus_id - i as i64;
            let p = n / bus_id;
            sum += a * mod_inv(p, *bus_id).unwrap() * p;
        }
    }

    let answer = sum % n;

    println!("The answer is {}", answer);
}
