use std::fs;
use z3::ast::Int;
use z3::{Optimize, SatResult};

pub fn solve() {
    let mut machines = parse_machines();

    let mut total_presses = 0;
    for machine in machines.iter_mut() {
        total_presses += min_presses_switches(machine, 0);
    }

    println!(
        "The fewest button presses required to correctly configure the indicator lights on all of the machines is {total_presses}."
    );

    let mut total_presses = 0;
    for machine in machines.iter_mut() {
        let matrix = build_matrix(machine);
        total_presses += min_presses_joltages(&matrix);
    }

    println!(
        "The fewest button presses required to correctly configure the joltage levels is {total_presses}."
    );
}

struct Machine {
    target_state: Vec<bool>,
    current_state: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

impl Machine {
    fn is_switches_valid(&self) -> bool {
        if self.target_state.len() != self.current_state.len() {
            panic!("target switches state and current switches state do not have the same length!");
        }

        !self
            .target_state
            .iter()
            .zip(self.current_state.iter())
            .any(|(a, b)| *a != *b)
    }
}

fn build_matrix(machine: &mut Machine) -> Vec<Vec<i64>> {
    let rows = machine.joltages.len();
    let cols = machine.buttons.len();
    let mut matrix = vec![vec![0i64; cols]; rows];

    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &position in button {
            matrix[position][button_idx] = 1;
        }
    }

    for (row, jolt) in machine.joltages.iter().enumerate() {
        matrix[row].push(*jolt as i64);
    }

    matrix
}

fn min_presses_joltages(matrix: &Vec<Vec<i64>>) -> i64 {
    let opt = Optimize::new();

    let n_buttons = matrix[0].len() - 1; // last column is the augmented RHS

    let vars: Vec<Int> = (0..n_buttons)
        .map(|i| Int::new_const(format!("x{i}")))
        .collect();

    let zero = Int::from_i64(0);

    for v in &vars {
        opt.assert(&v.ge(&zero));
    }

    for row in matrix.iter() {
        let terms: Vec<&Int> = row[..n_buttons]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != 0)
            .map(|(j, _)| &vars[j])
            .collect();

        let sum = Int::add(&terms);
        let rhs = Int::from_i64(row[n_buttons]);
        opt.assert(&sum.eq(&rhs));
    }

    let all_refs: Vec<&Int> = vars.iter().collect();
    let total = Int::add(&all_refs);
    opt.minimize(&total);

    assert_eq!(opt.check(&[]), SatResult::Sat);
    let model = opt.get_model().expect("Z3 returned Sat but no model");

    let answer: i64 = vars
        .iter()
        .map(|v| model.eval(v, true).unwrap().as_i64().unwrap())
        .sum();

    answer
}

fn min_presses_switches(machine: &mut Machine, button: usize) -> u32 {
    if machine.is_switches_valid() {
        return 0;
    }

    if button >= machine.buttons.len() {
        return u32::MAX;
    }

    let skip = min_presses_switches(machine, button + 1);

    for &i in &machine.buttons[button] {
        machine.current_state[i] = !machine.current_state[i];
    }

    let mut press = min_presses_switches(machine, button + 1);
    if press != u32::MAX {
        press += 1;
    }

    for &i in &machine.buttons[button] {
        machine.current_state[i] = !machine.current_state[i];
    }

    skip.min(press)
}

fn parse_machines() -> Vec<Machine> {
    let file_path = "./src/day10/input.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let mut machines = vec![];

    for line in input.lines() {
        let mut chunks_iter = line.split_whitespace();

        let target_cfg = chunks_iter.next().expect("Error getting target config");
        let target_cfg = &target_cfg[1..target_cfg.len() - 1];

        let target_state: Vec<bool> = target_cfg
            .chars()
            .map(|ch| match ch {
                '.' => false,
                '#' => true,
                _ => panic!("Error parsing target config"),
            })
            .collect();

        let current_state = vec![false; target_state.len()];
        let mut buttons: Vec<Vec<usize>> = vec![];
        let mut joltages: Vec<u32> = vec![];

        while let Some(chunk) = chunks_iter.next() {
            match chunk.chars().next().expect("Error parsing chunk") {
                '(' => {
                    let chunk = &chunk[1..chunk.len() - 1];
                    let button_combination: Vec<usize> = chunk
                        .split(',')
                        .map(|elem| elem.parse().expect("Error parsing button index"))
                        .collect();

                    buttons.push(button_combination);
                }
                '{' => {
                    let chunk = &chunk[1..chunk.len() - 1];
                    joltages = chunk
                        .split(',')
                        .map(|elem| elem.parse::<u32>().expect("Error parsing button index"))
                        .collect();
                }
                _ => panic!("Error parsing chunk"),
            }
        }

        machines.push(Machine {
            target_state,
            current_state,
            buttons,
            joltages,
        });
    }

    machines
}
