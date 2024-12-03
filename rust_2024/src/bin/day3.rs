use regex::Regex;

fn main() {
    let example_a = include_str!("../../inputs/day3/example.txt");
    let example_b = include_str!("../../inputs/day3/example_b.txt");
    let main_input = include_str!("../../inputs/day3/main.txt");

    solve_a(example_a);
    solve_a(main_input);

    solve_b(example_b);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let result = calculate(input);
    println!("part 1: {}", result)
}

fn solve_b(input: &str) {
    let mut text = input.to_string();
    let mut is_active = true;
    let mut result = 0;
    loop {
        if is_active {
            let dont_pos = text.find("don't()");
            if let Some(dont_pos) = dont_pos {
                result += calculate(&text[..dont_pos]);
                text = text[dont_pos + 7..].to_string();
                is_active = false;
            } else {
                result += calculate(&text);
                break;
            }
        } else {
            let do_pos = text.find("do()");
            if let Some(do_pos) = do_pos {
                text = text[do_pos + 4..].to_string();
                is_active = true;
                continue;
            } else {
                break;
            }
        }
    }

    println!("part 2: {}", result);
}

fn calculate(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?<num1>\d*),(?<num2>\d*)\)").unwrap();

    let numbers = re
        .captures_iter(input)
        .map(|captures| {
            (
                captures["num1"].parse::<i32>().unwrap(),
                captures["num2"].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();

    let result = numbers.iter().fold(0, |acc, x| acc + x.0 * x.1);

    result
}
