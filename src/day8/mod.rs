use std::{cmp::Reverse, collections::HashSet, fs};

pub fn solve() {
    let mut nodes = parse_nodes();
    let result_pt1 = solve_pt1(&mut nodes);

    let mut nodes = parse_nodes();
    let result_pt2 = solve_pt2(&mut nodes);

    println!("result part 1: {result_pt1}");
    println!("result part 2: {result_pt2}");
}

fn solve_pt2(nodes: &mut Vec<Node>) -> u64 {
    let mut num_of_diff_colors = nodes.len() - 1;

    let mut distances: Vec<(u64, usize, usize)> = Vec::new();

    for i in 0..nodes.len() - 1 {
        for j in i + 1..nodes.len() {
            let a = &nodes[i];
            let b = &nodes[j];
            let dist = calc_dist(a, b);
            distances.push((dist, i, j));
        }
    }

    distances.sort_by_key(|&elem| Reverse(elem));
    let result;

    loop {
        let (_, a, b) = distances.pop().expect("Not enough values to pop");

        let color_to_change = nodes[b].color;
        let new_color = nodes[a].color;

        if color_to_change == new_color {
            continue;
        }

        for node in nodes.iter_mut() {
            if node.color == color_to_change {
                node.color = new_color;
            }
        }

        nodes[a].neighbours.insert(b);
        nodes[b].neighbours.insert(a);

        num_of_diff_colors -= 1;

        if num_of_diff_colors == 0 {
            result = nodes[a].x * nodes[b].x;
            break;
        }
    }

    result
}

fn solve_pt1(nodes: &mut Vec<Node>) -> u32 {
    let mut distances: Vec<(u64, usize, usize)> = Vec::new();

    for i in 0..nodes.len() - 1 {
        for j in i + 1..nodes.len() {
            let a = &nodes[i];
            let b = &nodes[j];
            let dist = calc_dist(a, b);
            distances.push((dist, i, j));
        }
    }

    distances.sort_by_key(|&elem| Reverse(elem));

    for _ in 0..1000 {
        let (_, a, b) = distances.pop().expect("Not enough values to pop");

        let color_to_change = nodes[b].color;
        let new_color = nodes[a].color;

        if color_to_change == new_color {
            continue;
        }

        for node in nodes.iter_mut() {
            if node.color == color_to_change {
                node.color = new_color;
            }
        }

        nodes[a].neighbours.insert(b);
        nodes[b].neighbours.insert(a);
    }

    let mut visited: HashSet<usize> = HashSet::new();
    let mut sizes: Vec<u32> = Vec::new();

    for i in 0..nodes.len() {
        if !visited.contains(&i) {
            let size = calc_group_size(&nodes, i, &mut visited);
            sizes.push(size);
        }
    }

    sizes.sort_by_key(|&elem| Reverse(elem));

    let result = sizes.iter().take(3).product::<u32>();
    result
}

fn calc_group_size(nodes: &Vec<Node>, pos: usize, visited: &mut HashSet<usize>) -> u32 {
    if visited.contains(&pos) {
        return 0;
    }

    visited.insert(pos);
    let mut result = 1;

    for neighbour in &nodes[pos].neighbours {
        result += calc_group_size(nodes, *neighbour, visited);
    }

    result
}

fn calc_dist(a: &Node, b: &Node) -> u64 {
    let result =
        (a.x.abs_diff(b.x)).pow(2) + (a.y.abs_diff(b.y)).pow(2) + (a.z.abs_diff(b.z)).pow(2);

    result.isqrt()
}

#[derive(Debug)]
struct Node {
    x: u64,
    y: u64,
    z: u64,
    color: u32,
    neighbours: HashSet<usize>,
}

fn parse_nodes() -> Vec<Node> {
    let file_path = "./src/day8/input.txt";
    let input = fs::read_to_string(file_path).expect("Error reading input file");

    let mut nodes = Vec::new();
    let mut color = 0;

    for line in input.lines() {
        let mut content = line
            .split(',')
            .map(|elem| elem.parse::<u64>().expect("Error parsing coordinate"));

        let Some(x) = content.next() else {
            panic!("Error getting x coordinate")
        };

        let Some(y) = content.next() else {
            panic!("Error getting x coordinate")
        };

        let Some(z) = content.next() else {
            panic!("Error getting x coordinate")
        };

        nodes.push(Node {
            x,
            y,
            z,
            color,
            neighbours: HashSet::new(),
        });

        color += 1;
    }

    nodes
}
