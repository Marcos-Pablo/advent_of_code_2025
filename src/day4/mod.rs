use std::fs;

pub fn solve() {
    let diagram = parse_diagram();
    let adjacent_pos: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut accessible_roll_papers = 0;

    for row in 0..diagram.len() {
        for col in 0..diagram[row].len() {
            if diagram[row][col] == '.' {
                continue;
            }

            let mut count = 0;

            for (x, y) in adjacent_pos {
                let Some(new_row) = row.checked_add_signed(x) else {
                    continue;
                };

                let Some(new_col) = col.checked_add_signed(y) else {
                    continue;
                };

                if new_row >= diagram.len() {
                    continue;
                }

                if new_col >= diagram[row].len() {
                    continue;
                }

                if diagram[new_row][new_col] == '@' {
                    count += 1;
                }
            }

            if count < 4 {
                accessible_roll_papers += 1;
            }
        }
    }

    println!(
        "there are {accessible_roll_papers} rolls of paper that can be accessed by a forklift"
    );
}

fn parse_diagram() -> Vec<Vec<char>> {
    let mut diagram: Vec<Vec<char>> = Vec::new();

    let file_path = "./src/day4/input.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    for line in input.lines() {
        diagram.push(Vec::new());
        let len = diagram.len();
        for ch in line.chars() {
            diagram[len - 1].push(ch);
        }
    }

    diagram
}
