use std::io::{self, Write};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run() {
    println!("Welcome to Advent Of Code 2025!");
    let mut input = String::new();

    loop {
        println!("Type \"q\" to quit");
        print!("Select the day you want to solve (1-25): ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        if input.trim() == "q" {
            println!("Bye!");
            break;
        }

        let day: u8 = match input.trim().parse() {
            Ok(num) if (1..=25).contains(&num) => num,
            _ => {
                println!("Type a valid number between 1 and 25 or \"q\" to quit");
                continue;
            }
        };

        println!("Running day {day} against the provided input\n");

        match day {
            1 => day1::solve(),
            2 => day2::solve(),
            3 => day3::solve(),
            4 => day4::solve(),
            5 => day5::solve(),
            6 => day6::solve(),
            7 => day7::solve(),
            8 => day8::solve(),
            9 => day9::solve(),
            _ => println!("day {day} solution not available yet"),
        }
    }
}
