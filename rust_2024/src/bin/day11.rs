use std::collections::HashMap;

fn main() {
    let example = include_str!("../../inputs/day11/example.txt");
    let main_input = include_str!("../../inputs/day11/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn apply(list: Vec<u64>) -> Vec<u64> {
    let mut new_list = vec![];

    for stone in list {
        // first rule
        if stone == 0 {
            new_list.push(1);

            continue;
        }

        // second rule
        let number_len = stone.ilog10() + 1;
        if number_len % 2 == 0 {
            let (first, second) = (
                stone / 10_u64.pow(number_len / 2),
                stone % 10_u64.pow(number_len / 2),
            );

            new_list.push(first);
            new_list.push(second);

            continue;
        }

        new_list.push(stone * 2024);
    }

    new_list
}

fn apply_map(list: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_map = HashMap::new();

    for (stone, count) in list.iter() {
        // first rule
        if *stone == 0 {
            new_map
                .entry(1)
                .and_modify(|a| *a += *count)
                .or_insert(*count);

            continue;
        }

        // second rule
        let number_len = stone.ilog10() + 1;
        if number_len % 2 == 0 {
            let (first, second) = (
                stone / 10_u64.pow(number_len / 2),
                stone % 10_u64.pow(number_len / 2),
            );

            new_map
                .entry(first)
                .and_modify(|a| *a += *count)
                .or_insert(*count);
            new_map
                .entry(second)
                .and_modify(|a| *a += *count)
                .or_insert(*count);

            continue;
        }

        new_map
            .entry(stone * 2024)
            .and_modify(|a| *a += *count)
            .or_insert(*count);
    }

    new_map
}

fn solve_a(input: &str) {
    let mut stones_vec = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones_vec = apply(stones_vec);
    }

    println!("part 1: {}", stones_vec.len());
}

fn solve_b(input: &str) {
    let mut map = input
        .split_whitespace()
        .map(|num| (num.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<u64, usize>>();

    for _ in 0..75 {
        map = apply_map(map);
    }

    let result = map.values().sum::<usize>();

    println!("part 2: {}", result);
}
