use std::collections::HashSet;
use std::vec;

fn main() {
    let example = include_str!("../../inputs/day5/example.txt");
    let main_input = include_str!("../../inputs/day5/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let input = input.split("\n\n").collect::<Vec<_>>();

    let ranges = input[0]
        .lines()
        .map(|line| {
            let range = line.split("-").collect::<Vec<_>>();
            let r = range
                .iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (r[0], r[1])
        })
        .collect::<Vec<_>>();

    let numbers = input[1]
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;

    for num in numbers.iter() {
        for range in ranges.iter() {
            if *num >= range.0 && *num <= range.1 {
                result += 1;

                break;
            }
        }
    }

    println!("Part1: {}", result);
}

fn solve_b(input: &str) {
    let input = input.split("\n\n").collect::<Vec<_>>();

    let mut ranges = input[0]
        .lines()
        .map(|line| {
            let range = line.split("-").collect::<Vec<_>>();
            let r = range
                .iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (r[0], r[1])
        })
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result = 0;
    let mut range_start = ranges[0].0;
    let mut range_end = ranges[0].1;

    for range in ranges.iter() {
        // we have reached a non-overlapping range
        if range.0 > range_end {
            result += range_end - range_start + 1;

            range_start = range.0;
            range_end = range.1;

            continue;
        }

        range_end = range_end.max(range.1);
    }

    // add the final overlapping range
    result += range_end - range_start + 1;

    println!("Part2: {}", result);
}
