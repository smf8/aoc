fn main() {
    let example = include_str!("../../inputs/day7/example.txt");
    let main_input = include_str!("../../inputs/day7/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn check_list(input: &str, consider_concat: bool) -> u64 {
    let mut list = input
        .lines()
        .map(|line| {
            let split = line.split(": ").collect::<Vec<_>>();
            let final_num = split[0].parse::<u64>().unwrap();

            let numbers = split[1]
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (final_num, numbers)
        })
        .collect::<Vec<_>>();

    list.retain(|case| compute(case.0, case.1[0], &case.1[1..], consider_concat));

    let result = list.iter().fold(0, |acc, x| acc + x.0);

    result
}

fn solve_a(input: &str) {
    let result = check_list(input, false);
    println!("part 1: {}", result);
}

fn solve_b(input: &str) {
    let result = check_list(input, true);
    println!("part 2: {}", result);
}

fn compute(target: u64, value: u64, numbers: &[u64], consider_concat: bool) -> bool {
    if numbers.is_empty() {
        return target == value;
    }

    compute(target, value * numbers[0], &numbers[1..], consider_concat)
        || compute(target, value + numbers[0], &numbers[1..], consider_concat)
        || (consider_concat
            && compute(
                target,
                concat(value, numbers[0]),
                &numbers[1..],
                consider_concat,
            ))
}

fn concat(l: u64, r: u64) -> u64 {
    l * 10_u64.pow(r.ilog(10) + 1) + r
}
