use std::{fs, ops::Range};

pub fn solve() {
    let inventory = parse_inventory();

    let fresh_ingredients_count = inventory
        .ingredient_ids
        .iter()
        .filter(|id| inventory.fresh_ranges.iter().any(|r| r.contains(id)))
        .count();

    let unique_fresh_ids = inventory
        .fresh_ranges
        .iter()
        .map(|r| r.end - r.start)
        .sum::<u64>();

    println!("{fresh_ingredients_count} of the available ingredient IDs are fresh");
    println!("{unique_fresh_ids} IDs are fresh in total");
}

struct Inventory {
    fresh_ranges: Vec<Range<u64>>,
    ingredient_ids: Vec<u64>,
}

fn parse_inventory() -> Inventory {
    let file_path = "./src/day5/input.txt";

    let input = fs::read_to_string(file_path).expect("Error reading the file");

    let Some((ranges, ids)) = input.split_once("\n\n") else {
        panic!("Error spliting ranges and ids")
    };

    let mut fresh_ranges: Vec<Range<u64>> = ranges
        .lines()
        .map(|line| {
            let Some((start, end)) = line.split_once('-') else {
                panic!("Error spliting range");
            };

            let start: u64 = start.parse().expect("Error parsing range to usize");
            let end: u64 = end.parse().expect("Error parsing range to usize");
            start..end + 1
        })
        .collect();

    fresh_ranges.sort_by_key(|r| (r.start, r.end));

    let mut merged: Vec<Range<u64>> = Vec::new();
    for range in fresh_ranges {
        if let Some(last) = merged.last_mut() {
            if range.start <= last.end {
                last.end = last.end.max(range.end);
                continue;
            }
        }
        merged.push(range);
    }

    let ingredient_ids: Vec<u64> = ids
        .lines()
        .map(|line| line.parse().expect("Error parsing line to u64"))
        .collect();

    Inventory {
        fresh_ranges: merged,
        ingredient_ids,
    }
}
