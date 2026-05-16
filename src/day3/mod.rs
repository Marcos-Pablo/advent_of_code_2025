use std::fs;

pub fn solve() {
    let banks = parse_batteries_banks();

    let mut max_joltage_part1 = 0;
    for bank in &banks {
        let max_joltage = get_max_joltage(bank, 2);
        max_joltage_part1 += max_joltage;
    }

    println!("Total joltage part 1: {max_joltage_part1}");

    let mut max_joltage_part2 = 0;
    for bank in &banks {
        let max_joltage = get_max_joltage(bank, 12);
        max_joltage_part2 += max_joltage;
    }

    println!("Total joltage part 2: {max_joltage_part2}");
}

fn parse_batteries_banks() -> Vec<Vec<char>> {
    let s = fs::read_to_string("./src/day3/input.txt").expect("Error reading file");

    s.lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn get_max_joltage(bank: &Vec<char>, size: usize) -> u64 {
    let mut best_combination = String::new();
    let mut remaining = size;
    let mut start = 0;

    while remaining > 0 {
        let mut largest = start;
        for i in start..=bank.len() - remaining {
            if bank[i] > bank[largest] {
                largest = i;
            }
        }
        remaining -= 1;
        start = largest + 1;
        best_combination += &bank[largest].to_string();
    }

    let best_combination: u64 = best_combination
        .parse()
        .expect("Error parsing best combination");

    best_combination
}
