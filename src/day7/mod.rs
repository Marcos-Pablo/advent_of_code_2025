use std::{collections::HashMap, fs};

pub fn solve() {
    let mut grid = parse_grid();

    let mut total_split = 0;
    for row in 0..grid.len() - 1 {
        for col in 0..grid[row].len() {
            if grid[row][col] != 'S' {
                continue;
            }

            if grid[row + 1][col] == '.' {
                grid[row + 1][col] = 'S';
                continue;
            }

            if grid[row + 1][col] == '^' {
                total_split += 1;
                let new_row = row + 1;

                if col > 0 {
                    let new_col_left = col - 1;
                    grid[new_row][new_col_left] = 'S';
                }

                if col < grid[row].len() - 1 {
                    let new_col_right = col + 1;
                    grid[new_row][new_col_right] = 'S';
                }
            }
        }
    }

    let grid = parse_grid();
    let start_col = grid
        .first()
        .expect("Error getting start col")
        .iter()
        .position(|&elem| elem == 'S')
        .expect("Error getting start col");

    let total_realities = calc_realities(&grid, 0, start_col, &mut HashMap::new());

    println!("In this example, a tachyon beam is split a total of {total_split} times.");
    println!("In this example, there are a total of {total_realities} different realities");
}

fn calc_realities(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&val) = cache.get(&(row, col)) {
        return val;
    }

    if row == grid.len() {
        return 1;
    }

    if grid[row][col] == '.' {
        return calc_realities(grid, row + 1, col, cache);
    }

    let left = if col > 0 {
        calc_realities(grid, row + 1, col - 1, cache)
    } else {
        0
    };

    let right = if col + 1 < grid[row].len() {
        calc_realities(grid, row, col + 1, cache)
    } else {
        0
    };

    cache.insert((row, col), left + right);

    left + right
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            print!("{}", grid[row][col])
        }
        println!();
    }
}

fn parse_grid() -> Vec<Vec<char>> {
    let file_path = "./src/day7/input.txt";
    let content = fs::read_to_string(file_path).expect("Error reading file");

    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    grid
}
