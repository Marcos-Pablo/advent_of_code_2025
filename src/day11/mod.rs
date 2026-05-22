use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() {
    let graph = parse_graph();
    let num_diff_paths_pt1 = calc_num_diff_paths(&graph, "you", "out");

    println!("In total, there are {num_diff_paths_pt1} different paths leading from you to out.");

    let num_diff_paths_pt2 = {
        calc_num_diff_paths(&graph, "svr", "dac")
            * calc_num_diff_paths(&graph, "dac", "fft")
            * calc_num_diff_paths(&graph, "fft", "out")
            + calc_num_diff_paths(&graph, "svr", "fft")
                * calc_num_diff_paths(&graph, "fft", "dac")
                * calc_num_diff_paths(&graph, "dac", "out")
    };

    println!(
        "In total, there are {num_diff_paths_pt2} different paths leading from you to out that passes through dac and fft"
    );
}

fn calc_num_diff_paths(graph: &HashMap<String, HashSet<String>>, from: &str, to: &str) -> u32 {
    dfs(graph, &mut HashSet::new(), from, to)
}

fn dfs<'a>(
    graph: &'a HashMap<String, HashSet<String>>,
    visited: &mut HashSet<&'a str>,
    device: &'a str,
    to: &str,
) -> u32 {
    if device == to {
        return 1;
    }

    if let Some(_) = visited.get(device) {
        return 0;
    }

    visited.insert(device);

    let neighbours = graph
        .get(device)
        .unwrap_or_else(|| panic!("{device} not in graph"));
    let mut count = 0;

    for neighbour in neighbours {
        count += dfs(graph, visited, neighbour, to);
    }

    visited.remove(device);

    count
}

fn parse_graph() -> HashMap<String, HashSet<String>> {
    let file_path = "./src/day11/input.txt";

    let input = fs::read_to_string(file_path).expect("Error reading file");
    let mut graph = HashMap::new();

    for line in input.lines() {
        let (device, remaining) = line
            .split_once(':')
            .expect("Error spliting device label and neighbours list");

        let mut neighbours = HashSet::new();

        for neighbour in remaining.split_whitespace() {
            if !graph.contains_key(neighbour) {
                graph.insert(neighbour.to_string(), HashSet::new());
            }
            neighbours.insert(neighbour.to_string());
        }

        graph.insert(device.to_string(), neighbours);
    }

    graph
}
