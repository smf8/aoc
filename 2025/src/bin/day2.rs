fn main() {
    let example = include_str!("../../inputs/day2/example.txt");
    let main_input = include_str!("../../inputs/day2/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let ranges = input
        .split(',')
        .map(|line| {
            let parts = line
                .split('-')
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    for range in ranges {
        result += process_range(range.0, range.1);
    }

    println!("total invalids: {}", result);
}

fn process_range(start: usize, end: usize) -> usize {
    let mut total_invalids = 0;
    for i in start..=end {
        if is_invalid(i) {
            total_invalids += i;
        }
    }

    total_invalids
}

fn is_invalid(num: usize) -> bool {
    let digits = num.ilog10() + 1;

    // only process even digits
    if digits % 2 != 0 {
        return false;
    }

    let first_half = num / 10_usize.pow(digits / 2);
    let second_half = num % 10_usize.pow(digits / 2);

    first_half == second_half
}

fn is_invalid_p2(num: &str) -> bool {
    let len = num.len();
    for chars_to_check in 1..=len / 2 {
        let (first_part, other_parts) = num.split_at(chars_to_check);
        if other_parts.trim_start_matches(first_part) == "" {
            return true;
        }
    }

    false
}
fn solve_b(input: &str) {
    let ranges = input
        .split(',')
        .map(|line| {
            let parts = line
                .split('-')
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<_>>();

    let mut total_invalids = 0;
    for range in ranges {
        for i in range.0..=range.1 {
            if is_invalid_p2(&i.to_string()) {
                total_invalids += i;
            }
        }
    }

    println!("total invalids p2: {}", total_invalids);
}
