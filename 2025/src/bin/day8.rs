use std::collections::{HashMap, HashSet};
use std::vec;

fn main() {
    let example = include_str!("../../inputs/day8/example.txt");
    let main_input = include_str!("../../inputs/day8/main.txt");

    solve_a(example, 10);
    solve_a(main_input, 1000);

    solve_b(example);
    solve_b(main_input);
}

//3360 too low
fn solve_a(input: &str, limit: usize) {
    let junctions = input
        .lines()
        .map(|l| {
            let numbers = l
                .split(',')
                .map(|m| m.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            (numbers[0], numbers[1], numbers[2])
        })
        .collect::<Vec<_>>();

    // let distances :HashMap<f64, (i64, i64, i64)> = HashMap::new();
    let mut distances: Vec<(f64, ((i64, i64, i64), (i64, i64, i64)))> = Vec::new();

    for (i1, j1) in junctions[..junctions.len() - 1].iter().enumerate() {
        for j2 in junctions[i1 + 1..].iter() {
            let dist = (((j1.0 - j2.0).pow(2) + (j1.1 - j2.1).pow(2) + (j1.2 - j2.2).pow(2))
                as f64)
                .sqrt();

            distances.push((dist, (*j1, *j2)));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits: Vec<HashSet<(i64, i64, i64)>> =
        junctions.iter().map(|a| HashSet::from([*a])).collect();

    for d in distances[..limit].iter() {
        let circuit_a1 = circuits.iter().position(|a| a.contains(&d.1 .0));
        let circuit_a2 = circuits.iter().position(|a| a.contains(&d.1 .1));

        if circuit_a1.unwrap() == circuit_a2.unwrap() {
            continue;
        }

        // merge two circuits
        let old_circuit_a1 = circuits[circuit_a1.unwrap()].clone();

        circuits[circuit_a2.unwrap()].extend(old_circuit_a1.iter());

        circuits.remove(circuit_a1.unwrap());
    }

    circuits.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());

    println!(
        "Part1: {}",
        circuits[0].len() * circuits[1].len() * circuits[2].len()
    );
}

fn solve_b(input: &str) {
    let junctions = input
        .lines()
        .map(|l| {
            let numbers = l
                .split(',')
                .map(|m| m.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            (numbers[0], numbers[1], numbers[2])
        })
        .collect::<Vec<_>>();

    // let distances :HashMap<f64, (i64, i64, i64)> = HashMap::new();
    let mut distances: Vec<(f64, ((i64, i64, i64), (i64, i64, i64)))> = Vec::new();

    for (i1, j1) in junctions[..junctions.len() - 1].iter().enumerate() {
        for j2 in junctions[i1 + 1..].iter() {
            let dist = (((j1.0 - j2.0).pow(2) + (j1.1 - j2.1).pow(2) + (j1.2 - j2.2).pow(2))
                as f64)
                .sqrt();

            distances.push((dist, (*j1, *j2)));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits: Vec<HashSet<(i64, i64, i64)>> =
        junctions.iter().map(|a| HashSet::from([*a])).collect();

    for d in distances.iter() {
        let circuit_a1 = circuits.iter().position(|a| a.contains(&d.1 .0));
        let circuit_a2 = circuits.iter().position(|a| a.contains(&d.1 .1));

        if circuit_a1.unwrap() == circuit_a2.unwrap() {
            continue;
        }

        // merge two circuits
        let old_circuit_a1 = circuits[circuit_a1.unwrap()].clone();
        circuits[circuit_a2.unwrap()].extend(old_circuit_a1.iter());

        if circuits.len() == 2 {
            println!("Part2: {}", d.1 .0 .0 * d.1 .1 .0);

            return;
        }

        circuits.remove(circuit_a1.unwrap());
    }
}
