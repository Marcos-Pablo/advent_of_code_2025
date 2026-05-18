use std::fs;

pub fn solve() {
    let problems_pt1 = parse_worksheet_pt1();

    let grand_total = problems_pt1
        .iter()
        .map(|p| match p.operation {
            Operation::Sum => p.operands.iter().sum::<u64>(),
            Operation::Mul => p.operands.iter().product(),
        })
        .sum::<u64>();

    println!("grand total part 1: {grand_total}");

    let problems_pt2 = parse_worksheet_pt2();

    let grand_total = problems_pt2
        .iter()
        .map(|p| match p.operation {
            Operation::Sum => p.operands.iter().sum::<u64>(),
            Operation::Mul => p.operands.iter().product(),
        })
        .sum::<u64>();

    println!("grand total part 2: {grand_total}");
}

struct Problem {
    operands: Vec<u64>,
    operation: Operation,
}

enum Operation {
    Sum,
    Mul,
}

fn parse_worksheet_pt1() -> Vec<Problem> {
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

    let rows = worksheet.len();
    let cols = worksheet[0].len();

    let mut problems = Vec::new();
    for col in 0..cols {
        let symbol = &worksheet[rows - 1][col];
        let operation = match symbol.as_str() {
            "*" => Operation::Mul,
            "+" => Operation::Sum,
            _ => panic!("Invalid operation"),
        };

        let mut operands = Vec::new();
        for row in 0..rows - 1 {
            let operand: u64 = worksheet[row][col].parse().expect("Error parsing value");

            operands.push(operand);
        }

        problems.push(Problem {
            operands,
            operation,
        });
    }

    problems
}

fn parse_worksheet_pt2() -> Vec<Problem> {
    let file_path = "./src/day6/input.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let last_line: String = input
        .lines()
        .last()
        .expect("Error reading last line")
        .to_string();

    let num_cols = last_line.split_whitespace().count();

    let mut col_lens: Vec<usize> = vec![0; num_cols];
    let mut chars_iter = last_line.chars();

    let mut col = 0;
    let mut len = 0;
    loop {
        let Some(ch) = chars_iter.next() else {
            col_lens[col] = len;
            break;
        };

        if (ch == '*' || ch == '+') && len > 0 {
            col_lens[col] = len - 1;
            len = 1;
            col += 1;
            continue;
        }
        len += 1;
    }

    let mut columns: Vec<Vec<String>> = Vec::new();

    let mut offset = 0;
    for col in 0..num_cols {
        let len = col_lens[col];
        let mut column = Vec::new();

        for line in input.lines() {
            let content = &line[offset..offset + len];
            column.push(content.to_string());
        }

        columns.push(column);
        offset += len + 1;
    }

    let mut problems = Vec::new();
    for (col_num, column) in columns.iter().enumerate() {
        let col_len = col_lens[col_num];
        let num_lines = column.len();
        let operation = match column.last().expect("Error parsing operation") {
            line if line.starts_with("*") => Operation::Mul,
            line if line.starts_with("+") => Operation::Sum,
            _ => panic!("Error parsing operation"),
        };
        let mut operands = Vec::new();

        let grid: Vec<Vec<char>> = column.iter().map(|line| line.chars().collect()).collect();

        for col in (0..col_len).rev() {
            let mut num = String::new();
            for row in 0..num_lines - 1 {
                let ch = grid[row][col];
                if ch != ' ' {
                    num += &ch.to_string();
                }
            }

            let num: u64 = num.parse().expect("Error parsing number");
            operands.push(num);
        }

        problems.push(Problem {
            operands,
            operation,
        });
    }

    problems
}
