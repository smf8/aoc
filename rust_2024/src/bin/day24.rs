use std::collections::{HashMap, VecDeque};

fn main() {
    let example = include_str!("../../inputs/day24/example.txt");
    let main_input = include_str!("../../inputs/day24/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(main_input);
}

fn solve_a(input: &str) {
    let mut known_values = HashMap::new();

    let input = input.split("\n\n").collect::<Vec<_>>();

    for constants in input[0].lines() {
        let split = constants.split(": ").collect::<Vec<_>>();
        let value = split[1].parse::<i32>().unwrap() != 0;

        known_values.insert(split[0], value);
    }

    let mut gates = input[1]
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            (parts[0], parts[1], parts[2], parts[4])
        })
        .collect::<VecDeque<_>>();

    while let Some(gate) = gates.pop_front() {
        if !known_values.contains_key(gate.0) || !known_values.contains_key(gate.2) {
            gates.push_back(gate);
            continue;
        }

        let result = match gate.1 {
            "AND" => known_values[gate.0] && known_values[gate.2],
            "OR" => known_values[gate.0] || known_values[gate.2],
            "XOR" => known_values[gate.0] ^ known_values[gate.2],
            _ => unreachable!(),
        };

        known_values.insert(gate.3, result);
    }

    let z_list = known_values
        .iter()
        .filter(|(k, v)| k.starts_with("z") && **v)
        .collect::<Vec<_>>();

    let mut result = 0_usize;
    for (key, _) in z_list {
        let number = key.strip_prefix("z").unwrap().parse::<usize>().unwrap();

        result |= 1 << number;
    }

    println!("part 1: {}", result);
}

fn solve_b(_: &str) {
    // I'm too ashamed of my solution to share it here.
    // but I searched for contradictions based on logical structure of
    // adder
    // Z[i] = (x[i] ^ y[i]) ^ c[i-1]
    // and
    // C[i] = (x[i] & y[i]) | ((x[i] ^ y[i]) & c[i-1] )
    // So as we only have a single OR operator. we need to iterate over all of them
    // and validate if they are valid carries (by checking if the result appears in an "^" and  "&" operation
    // after that. we need to verify Z[i] expressions. so z[i] MUST only comes as a value of XOR operation
    // that XOR operation must be between a valid carry (check validity by checking if it's present in a OR expression)
    // and a x[i] ^ y[i] expression
    // doing so for my case revealed most of the corrupt wires 🔌
}
