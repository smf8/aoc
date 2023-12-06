use std::cmp::{min, Ordering};
use std::iter::Map;

fn main() {
    solve_a();
    solve_b();
}

fn solve_a() {
    let (seeds, mappings) = read_input();

    let res = seeds
        .iter()
        .map(|seed| mappings.iter().fold(*seed, |a, x| find_seed_mapping(a, x)))
        .min()
        .unwrap();

    println!("{}", res);
}

// lazy way to compute :(
// will take around 2 min to produce output
fn solve_b() {
    let (mut seeds, mappings) = read_input();

    let res = seeds
        .chunks(2)
        .map(|a| {
            (a[0]..a[0] + a[1])
                .map(|seed| mappings.iter().fold(seed, |a, x| find_seed_mapping(a, x)))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("{}", res);
}

fn find_seed_mapping(seed: i64, mapping: &[Mapping]) -> i64 {
    mapping
        .binary_search_by(|a| match (a.0 .0.cmp(&seed), a.0 .1.cmp(&seed)) {
            (Ordering::Less, Ordering::Greater) | (Ordering::Equal, _) => Ordering::Equal,
            (Ordering::Less, Ordering::Equal | Ordering::Less) => Ordering::Less,
            (Ordering::Greater, _) => Ordering::Greater,
        })
        .map(|mapping_index| seed + mapping[mapping_index].2)
        .unwrap_or(seed)
}

struct Mapping((i64, i64), (i64, i64), i64);

fn read_input() -> (Vec<i64>, Vec<Vec<Mapping>>) {
    let input = include_str!("example");
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let seeds = parts[0]
        .split_whitespace()
        .filter_map(|a| a.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let mappings = parts[1..]
        .iter()
        .map(|mappings| {
            let mut mapping = mappings
                .lines()
                .skip(1)
                .map(|line_number| {
                    let range = line_number
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();

                    Mapping(
                        (range[1], range[1] + range[2]),
                        (range[0], range[0] + range[2]),
                        range[0] - range[1],
                    )
                })
                .collect::<Vec<_>>();

            mapping.sort_by(|a, b| a.0 .0.cmp(&b.0 .1));

            mapping
        })
        .collect::<Vec<_>>();

    (seeds, mappings)
}
