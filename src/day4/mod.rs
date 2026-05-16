use std::fs;

pub fn solve() {
    let mut diagram = parse_diagram();
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
    let mut pos_to_remove: Vec<(usize, usize)> = vec![];

    loop {
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
                    pos_to_remove.push((row, col));
                }
            }
        }

        if pos_to_remove.len() == 0 {
            break;
        }

        remove_roll_papers(&pos_to_remove, &mut diagram);
        pos_to_remove.clear();
    }

    println!("there are {accessible_roll_papers} rolls of paper that can be removed by a forklift");
}

fn remove_roll_papers(pos_to_remove: &[(usize, usize)], diagram: &mut [Vec<char>]) {
    for &(row, col) in pos_to_remove {
        diagram[row][col] = '.';
    }
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
