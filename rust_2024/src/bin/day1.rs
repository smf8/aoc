use std::collections::HashMap;

fn main() {
    let example = include_str!("../../inputs/day3/example.txt");
    let main_input = include_str!("../../inputs/day3/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let lists = input
        .lines()
        .map(|line| {
            let pairs = line
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (pairs[0], pairs[1])
        })
        .collect::<Vec<_>>();

    let (mut first_list, mut second_list): (Vec<i32>, Vec<i32>) = lists.into_iter().unzip();

    first_list.sort();
    second_list.sort();

    let result = first_list
        .iter()
        .zip(second_list.iter())
        .fold(0, |acc, x| acc + (x.0 - x.1).abs());

    println!("part 1: {}", result);
}

fn solve_b(input: &str) {
    let lists = input
        .lines()
        .map(|line| {
            let pairs = line
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (pairs[0], pairs[1])
        })
        .collect::<Vec<_>>();

    let (first_list, second_list): (Vec<i32>, Vec<i32>) = lists.into_iter().unzip();

    let mut second_map = HashMap::new();
    for num in second_list {
        second_map.entry(num).and_modify(|a| *a += 1).or_insert(1);
    }

    let res = first_list
        .iter()
        .fold(0, |acc, x| acc + *x * second_map.get(x).unwrap_or(&0));

    println!("part 2: {}", res)
}
