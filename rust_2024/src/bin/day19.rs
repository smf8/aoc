use std::collections::HashMap;

fn main() {
    let example = include_str!("../../inputs/day19/example.txt");
    let main_input = include_str!("../../inputs/day19/main.txt");

    solve_a(example);
    solve_a(main_input);
    //
    solve_b(example);
    solve_b(main_input);
}

fn comb_possible_count(stripes: &[&str], pattern: &str, dp: &mut HashMap<String, u64>) -> u64 {
    let mut result = 0;
    if pattern.is_empty() {
        return 1;
    }

    for stripe in stripes.iter() {
        if let Some(trimmed_pattern) = pattern.strip_prefix(stripe) {
            if let Some(dp_value) = dp.get(trimmed_pattern) {
                result += *dp_value;
            } else {
                let count = comb_possible_count(stripes, trimmed_pattern, dp);
                dp.insert(trimmed_pattern.to_string(), count);
                result += count;
            }
        }
    }

    result
}

fn comb_possible(stripes: &[&str], pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }

    for stripe in stripes.iter() {
        if let Some(trimmed_pattern) = pattern.strip_prefix(stripe) {
            if comb_possible(stripes, trimmed_pattern) {
                return true;
            }
        }
    }

    false
}

fn solve_a(input: &str) {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let stripes = input[0].split(", ").collect::<Vec<_>>();
    let cases = input[1].lines().collect::<Vec<_>>();
    let mut possibles = 0;

    for case in cases {
        let possibilities = comb_possible(&stripes, case);
        if possibilities {
            possibles += 1;
        }
    }

    println!("part 1: {}", possibles);
}

fn solve_b(input: &str) {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let stripes = input[0].split(", ").collect::<Vec<_>>();

    let cases = input[1].lines().collect::<Vec<_>>();

    let mut possibles = 0;

    for case in cases {
        let possibilities = comb_possible_count(&stripes, case, &mut HashMap::new());
        possibles += possibilities;
    }

    println!("part 2: {}", possibles);
}
