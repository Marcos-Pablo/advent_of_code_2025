use std::{fs, ops::RangeInclusive};

pub fn solve() {
    let inventory = parse_inventory();

    let fresh_ingredients_count = inventory
        .ingredient_ids
        .iter()
        .filter(|id| inventory.fresh_ranges.iter().any(|r| r.contains(id)))
        .count();

    println!("{fresh_ingredients_count} of the available ingredient IDs are fresh");
}

struct Inventory {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ingredient_ids: Vec<u64>,
}

fn parse_inventory() -> Inventory {
    let file_path = "./src/day5/input.txt";

    let input = fs::read_to_string(file_path).expect("Error reading the file");

    let Some((ranges, ids)) = input.split_once("\n\n") else {
        panic!("Error spliting ranges and ids")
    };

    let mut fresh_ranges = Vec::new();
    for line in ranges.lines() {
        let Some((start, end)) = line.split_once("-") else {
            panic!("Error spliting range line")
        };

        let start: u64 = start.parse().expect("Error parsing range to usize");
        let end: u64 = end.parse().expect("Error parsing range to usize");

        fresh_ranges.push(start..=end);
    }

    let mut ingredient_ids = Vec::new();
    for line in ids.lines() {
        let id: u64 = line.parse().expect("Error parsing id to u64");
        ingredient_ids.push(id);
    }

    Inventory {
        fresh_ranges,
        ingredient_ids,
    }
}
