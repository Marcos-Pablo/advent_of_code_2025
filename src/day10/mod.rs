use std::fs;

pub fn solve() {
    let mut machines = parse_machines();

    let mut total_presses = 0;
    for machine in machines.iter_mut() {
        total_presses += min_presses_to_valid_state(machine, 0);
    }

    println!(
        "The fewest button presses required to correctly configure the indicator lights on all of the machines is {total_presses}."
    );

    let mut total_presses = 0;
    for machine in machines.iter_mut() {
        total_presses += min_presses_to_valid_joltage(machine, 0);
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

    fn is_joltages_valid(&self) -> bool {
        !self.joltages.iter().any(|&jolt| jolt > 0)
    }
}

fn min_presses_to_valid_joltage(machine: &mut Machine, button: usize) -> u32 {
    if button == machine.buttons.len() {
        return if machine.is_joltages_valid() {
            0
        } else {
            u32::MAX
        };
    }

    let max_count = machine.buttons[button]
        .iter()
        .map(|&i| machine.joltages[i])
        .min()
        .expect("Error getting max count");

    let mut min_presses = u32::MAX;

    for count in 0..=max_count {
        for &i in &machine.buttons[button] {
            machine.joltages[i] -= 1 * count;
        }

        let result = min_presses_to_valid_joltage(machine, button + 1).saturating_add(count);
        min_presses = min_presses.min(result);

        for &i in &machine.buttons[button] {
            machine.joltages[i] += 1 * count;
        }
    }

    min_presses
}

fn min_presses_to_valid_state(machine: &mut Machine, button: usize) -> u32 {
    if machine.is_switches_valid() {
        return 0;
    }

    if button >= machine.buttons.len() {
        return u32::MAX;
    }

    let skip = min_presses_to_valid_state(machine, button + 1);

    for &i in &machine.buttons[button] {
        machine.current_state[i] = !machine.current_state[i];
    }

    let mut press = min_presses_to_valid_state(machine, button + 1);
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
