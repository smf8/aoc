fn main() {
    let example = include_str!("../../inputs/day7/example.txt");
    let main_input = include_str!("../../inputs/day7/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let mut cases = input
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

    cases.retain(|case| is_combination_possible(case.0, &case.1, false).is_some());

    let result = cases.iter().fold(0, |acc, x| acc + x.0);

    println!("part 1: {}", result);
}

fn is_combination_possible(
    final_num: u64,
    numbers: &[u64],
    consider_concat: bool,
) -> Option<String> {
    let last_number_len = numbers.last().unwrap().ilog10() + 1;

    if numbers.len() == 2 {
        if final_num == numbers[0] * numbers[1] {
            return Some(format!("{} * {}", numbers[0], numbers[1]));
        }

        if final_num == numbers[0] + numbers[1] {
            return Some(format!("{} + {}", numbers[0], numbers[1]));
        }

        if final_num == numbers[0] * 10u64.pow(last_number_len) + numbers[1] {
            return Some(format!("{} || {}", numbers[0], numbers[1]));
        }

        return None;
    }

    // since we're using u64, this is to avoid negative value panics
    if final_num < *numbers.last().unwrap() {
        return None;
    }

    // check reverse effect of concat
    if consider_concat && final_num % 10u64.pow(last_number_len) == *numbers.last().unwrap() {
        // last n digit of final number is the same as our case. we can do ||
        if let Some(res) = is_combination_possible(
            final_num / 10u64.pow(last_number_len),
            &numbers[..numbers.len() - 1],
            consider_concat,
        ) {
            return Some(format!("{} || {}", res, numbers.last().unwrap()));
        }
    }

    // reverse of * is / but only with 0 remainder
    if final_num % numbers.last().unwrap() != 0 {
        if let Some(res) = is_combination_possible(
            final_num - numbers.last().unwrap(),
            &numbers[..numbers.len() - 1],
            consider_concat,
        ) {
            return Some(format!("{} + {}", res, numbers.last().unwrap()));
        }
    } else {
        if let Some(res) = is_combination_possible(
            final_num / numbers.last().unwrap(),
            &numbers[..numbers.len() - 1],
            consider_concat,
        ) {
            return Some(format!("{} * {}", res, numbers.last().unwrap()));
        }

        if let Some(res) = is_combination_possible(
            final_num - numbers.last().unwrap(),
            &numbers[..numbers.len() - 1],
            consider_concat,
        ) {
            return Some(format!("{} + {}", res, numbers.last().unwrap()));
        }
    }

    None
}

fn solve_b(input: &str) {
    let mut cases = input
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

    cases.retain(|case| {
        let res = is_combination_possible(case.0, &case.1, true);
        if let Some(res_str) = &res {
            println!("{}: {}", case.0, res_str)
        }

        res.is_some()
    });

    let result = cases.iter().fold(0, |acc, x| acc + x.0);

    println!("part 2: {}", result);
}
