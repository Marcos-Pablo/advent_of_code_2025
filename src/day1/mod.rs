use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() {
    let file_path = "./src/day1/input.txt";
    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);
    let rotations = parse_rotations(reader);

    let mut curr_pos: i32 = 50;
    let mut password_part_1 = 0;
    let mut password_part_2 = 0;

    println!("Solving day 1...");

    for rotation in rotations {
        let starting_from = curr_pos;
        curr_pos = match rotation {
            Rotation::Left { num_rotations } => curr_pos - num_rotations,
            Rotation::Right { num_rotations } => curr_pos + num_rotations,
        };

        let rotations_complete = match curr_pos {
            0 => 1,
            pos if pos < 0 && starting_from != 0 => 1 + (pos / 100).abs(),
            pos if pos < 0 => (pos / 100).abs(),
            pos if pos > 99 => pos / 100,
            _ => 0,
        };

        curr_pos = curr_pos.rem_euclid(100);

        if curr_pos == 0 {
            password_part_1 += 1;
        }

        if rotations_complete > 0 {
            password_part_2 += rotations_complete;
        }
    }

    println!();
    println!("The solution for part 1 is {password_part_1}");
    println!("The solution for part 2 is {password_part_2}");
    println!();
}

enum Rotation {
    Left { num_rotations: i32 },
    Right { num_rotations: i32 },
}

fn parse_rotations(reader: impl BufRead) -> impl Iterator<Item = Rotation> {
    reader.lines().into_iter().map(|line| {
        let line = line.expect("Error reading line");
        let (direction, num_rotations) = line.trim().split_at(1);

        let num_rotations: i32 = match num_rotations.trim().parse() {
            Ok(num) if num >= 0 => num,
            _ => panic!(
                "Invalid number of rotations, number of rotations should a positive integer number"
            ),
        };

        match direction {
            "L" => Rotation::Left { num_rotations },
            "R" => Rotation::Right { num_rotations },
            _ => panic!(
                "Invalid direction: {direction}, the allowed directions are L (Left) and R (Right)"
            ),
        }
    })
}
