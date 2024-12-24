use std::collections::HashSet;

fn main() {
    let example = include_str!("../../inputs/day25/example.txt");
    let main_input = include_str!("../../inputs/day25/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b();
}
fn parse_input(input: &str) -> (HashSet<i32>, HashSet<i32>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for case in input.split("\n\n") {
        let mut number = 0;
        for line in case.lines() {
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    number += 10_i32.pow((line.len() - 1 - j) as u32)
                }
            }
        }

        if case.starts_with("#####") {
            locks.push(number - 11111);
        } else if case.ends_with("#####") {
            keys.push(number - 11111);
        }
    }

    (HashSet::from_iter(locks), HashSet::from_iter(keys))
}

// 4304 to high
fn solve_a(input: &str) {
    let (locks, keys) = parse_input(input);

    let mut result = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if overlap(*key, *lock) {
                result += 1;
            }
        }
    }

    println!("part 1: {}", result);
}

fn overlap(mut key: i32, mut lock: i32) -> bool {
    for _ in 0..=4 {
        let key_digit = key % 10;
        let lock_digit = lock % 10;

        if key_digit + lock_digit > 5 {
            return false;
        }

        key /= 10;
        lock /= 10;
    }

    true
}
fn solve_b() {
    println!("All good things must come to an end :)")
}
