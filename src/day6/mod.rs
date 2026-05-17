use std::fs;

pub fn solve() {
    let worksheet = parse_worksheet();

    let rows = worksheet.len();
    let cols = worksheet[0].len();
    let mut grand_total = 0;

    for col in 0..cols {
        let symbol = &worksheet[rows - 1][col];
        let mut acc = match symbol.as_str() {
            "*" => 1,
            "+" => 0,
            _ => panic!("Invalid symbol!"),
        };
        for row in 0..rows - 1 {
            let val: u64 = worksheet[row][col].parse().expect("Error parsing value");

            acc = match symbol.as_str() {
                "*" => acc * val,
                "+" => acc + val,
                _ => panic!("Invalid symbol"),
            }
        }
        grand_total += acc;
    }

    println!("grand total: {grand_total}");
}

fn parse_worksheet() -> Vec<Vec<String>> {
    let file_path = "./src/day6/input.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let mut worksheet: Vec<Vec<String>> = Vec::new();
    for line in input.lines() {
        worksheet.push(Vec::new());
        let last_pos = worksheet.len() - 1;
        for symbol in line.split_whitespace() {
            worksheet[last_pos].push(symbol.to_string());
        }
    }

    worksheet
}
