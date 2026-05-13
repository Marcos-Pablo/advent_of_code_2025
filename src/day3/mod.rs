use std::{cmp::max, fs};

pub fn solve() {
    let banks = parse_batteries_banks();

    let mut max_joltage_part1 = 0;
    for bank in &banks {
        let max_joltage = get_max_joltage_part1(bank);
        max_joltage_part1 += max_joltage;
    }

    println!("Total joltage part 1: {max_joltage_part1}");

    let mut max_joltage_part2 = 0;
    for bank in &banks {
        let max_joltage = get_max_joltage_part2(bank);
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

fn get_max_joltage_part1(bank: &Vec<char>) -> u32 {
    let mut max_joltage = 0;

    for i in 0..bank.len() - 1 {
        let left = bank[i];
        let mut j = i + 1;
        while j < bank.len() {
            let right = bank[j];

            let val: u32 = format!("{left}{right}")
                .parse()
                .expect("Error parsing batteries pair");

            max_joltage = max(max_joltage, val);
            j += 1;
        }
    }

    max_joltage
}

fn get_max_joltage_part2(bank: &Vec<char>) -> u64 {
    let mut max_joltage = 0;
    let mut curr: Vec<char> = Vec::new();

    take_or_skip(bank, &mut curr, &mut max_joltage, 0);

    max_joltage
}

fn take_or_skip(bank: &Vec<char>, curr: &mut Vec<char>, max_joltage: &mut u64, i: usize) {
    if curr.len() == 12 {
        let val: String = curr.iter().collect();
        let val: u64 = val
            .parse()
            .unwrap_or_else(|_| panic!("Error parsing val: {val}"));

        *max_joltage = max(*max_joltage, val);
        return;
    }

    if i >= bank.len() {
        return;
    }

    // Take
    curr.push(bank[i]);
    take_or_skip(bank, curr, max_joltage, i + 1);
    curr.pop();

    // Skip
    take_or_skip(bank, curr, max_joltage, i + 1);
}
