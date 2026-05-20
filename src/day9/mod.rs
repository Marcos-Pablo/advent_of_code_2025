use std::fs;

pub fn solve() {
    let coordinates = parse_coordinates();
    let (vert, horz) = parse_walls(&coordinates);
    let (pt1, pt2) = calc_biggest_areas(&coordinates, &vert, &horz);

    println!("Biggest area part 1: {pt1}");
    println!("Biggest area part 2: {pt2}");
}

struct Coordinate {
    x: u64,
    y: u64,
}

struct VertWall {
    column: u64,
    y1: u64,
    y2: u64,
}

struct HorzWall {
    line: u64,
    x1: u64,
    x2: u64,
}

impl Coordinate {
    fn calc_area(&self, coord: &Coordinate) -> u64 {
        (self.x.abs_diff(coord.x) + 1) * (self.y.abs_diff(coord.y) + 1)
    }
}

fn calc_biggest_areas(
    coordinates: &Vec<Coordinate>,
    vert: &Vec<VertWall>,
    horz: &Vec<HorzWall>,
) -> (u64, u64) {
    let mut pt1 = 0;
    let mut pt2 = 0;

    for i in 0..coordinates.len() - 1 {
        for j in i + 1..coordinates.len() {
            let a = &coordinates[i];
            let b = &coordinates[j];

            let area = a.calc_area(b);
            pt1 = pt1.max(area);
            if !is_rec_valid(vert, horz, a, b) {
                continue;
            }
            pt2 = pt2.max(area);
        }
    }

    (pt1, pt2)
}

fn is_rec_valid(
    vert: &Vec<VertWall>,
    horz: &Vec<HorzWall>,
    a: &Coordinate,
    b: &Coordinate,
) -> bool {
    let left = a.x.min(b.x);
    let right = a.x.max(b.x);

    let bottom = a.y.min(b.y);
    let top = a.y.max(b.y);

    let center_x = (left + right) / 2;
    let center_y = (bottom + top) / 2;

    if !is_point_inside(center_x, center_y, vert) {
        return false;
    }

    for wall in vert {
        if left < wall.column && wall.column < right && wall.y1.max(bottom) < wall.y2.min(top) {
            return false;
        }
    }

    for wall in horz {
        if bottom < wall.line && wall.line < top && wall.x1.max(left) < wall.x2.min(right) {
            return false;
        }
    }

    true
}

fn is_point_inside(x: u64, y: u64, vert: &Vec<VertWall>) -> bool {
    let mut inside = false;

    for wall in vert {
        let column = wall.column * 2;
        let top = wall.y1 * 2;
        let bottom = wall.y2 * 2;

        if x < column && top <= y && y < bottom {
            inside = !inside;
        }
    }

    inside
}

fn parse_walls(coordinates: &Vec<Coordinate>) -> (Vec<VertWall>, Vec<HorzWall>) {
    let mut horizontal = vec![];
    let mut vertical = vec![];

    for i in 0..coordinates.len() {
        let a = &coordinates[i];
        let b = &coordinates[(i + 1) % coordinates.len()];

        let is_horizontal = a.y == b.y;

        match is_horizontal {
            true => horizontal.push(HorzWall {
                line: a.y,
                x1: a.x.min(b.x),
                x2: a.x.max(b.x),
            }),
            false => vertical.push(VertWall {
                column: a.x,
                y1: a.y.min(b.y),
                y2: a.y.max(b.y),
            }),
        }
    }

    (vertical, horizontal)
}

fn parse_coordinates() -> Vec<Coordinate> {
    let file_path = "./src/day9/input.txt";

    let input = fs::read_to_string(file_path).expect("Error reading file");

    let coordinates = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("Error spliting coordintes");

            let (x, y): (u64, u64) = (
                x.parse().expect("Error parsing x coordinate"),
                y.parse().expect("Error parsing y coordinate"),
            );

            Coordinate { x, y }
        })
        .collect();

    coordinates
}
