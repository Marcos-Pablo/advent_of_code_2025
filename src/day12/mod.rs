use std::fs;

pub fn solve() {
    let (presents, regions) = parse_input();
    let mut result = 0;

    for region in &regions {
        if can_fit(&region, &presents) {
            result += 1;
        }
    }

    println!("{result} regions can fit all of their listed presents.");
}

fn can_fit(region: &Region, presents: &[Present; 6]) -> bool {
    let region_area = region.width * region.height;

    let required_area: u32 = region.presents.iter().sum();

    required_area * 9 <= region_area
}

#[derive(Debug)]
struct Present {
    area: u32,
    min_width: u32,
    min_height: u32,
    shape: [[bool; 3]; 3],
}

#[derive(Debug)]
struct Region {
    height: u32,
    width: u32,
    presents: [u32; 6],
}

fn parse_input() -> ([Present; 6], Vec<Region>) {
    let file_path = "./src/day12/input.txt";
    let input = fs::read_to_string(file_path).expect("Error reading input");

    let presents: Vec<Present> = input
        .split("\n\n")
        .take(6)
        .map(|content| {
            let mut shape = [[false; 3]; 3];
            let mut min_row = 2;
            let mut min_col = 2;
            let mut max_row = 0;
            let mut max_col = 0;

            content.lines().skip(1).enumerate().for_each(|(i, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(j, b)| match b {
                        b'#' => {
                            shape[i][j] = true;
                            min_row = min_row.min(i);
                            min_col = min_col.min(j);
                            max_row = max_row.max(i);
                            max_col = max_col.max(j);
                        }
                        b'.' => shape[i][j] = false,
                        _ => panic!("Invalid char"),
                    });
            });

            let area = shape
                .as_flattened()
                .iter()
                .map(|&flag| if flag { 1 } else { 0 })
                .sum();

            Present {
                area,
                shape,
                min_width: (max_col - min_col + 1) as u32,
                min_height: (max_row - min_row + 1) as u32,
            }
        })
        .collect();

    let regions = input
        .split("\n\n")
        .skip(6)
        .next()
        .expect("Error parsing regions")
        .lines()
        .map(|line| {
            let (area, presents) = line
                .split_once(':')
                .expect("Error spliting region area and presents");

            let (cols, rows) = area
                .split_once('x')
                .expect("Error spliting region cols and rows");
            let cols: u32 = cols.parse().expect("Error parsing cols to u32");
            let rows: u32 = rows.parse().expect("Error parsing rows to u32");

            let presents: Vec<u32> = presents
                .split_whitespace()
                .map(|amount| amount.parse().expect("Error parsing amount to u32"))
                .collect();

            let presents: [u32; 6] = presents.try_into().unwrap();

            Region {
                height: rows,
                width: cols,
                presents,
            }
        })
        .collect();

    let presents: [Present; 6] = presents.try_into().unwrap();

    (presents, regions)
}
