use std::io::{self, Write};

fn main() {
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

        println!("Running day {day} against the provided input\n")
    }
}
