fn main() {
    let example = include_str!("../../inputs/day1/example.txt");
    let main_input = include_str!("../../inputs/day1/main.txt");

    // solve_a(example);
    // solve_a(main_input);
    //
    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let lists = input
        .lines()
        .map(|line| {
            let parts = line.split_at(1);
            let is_neg = parts.0.starts_with('L');
            let number: i32 = parts.1.parse().unwrap();

            (is_neg, number)
        })
        .collect::<Vec<_>>();

    let mut start_index = 50;
    let mut zeros = 0;

    for (is_neg, number) in lists {
        start_index += ((is_neg as i32) * -2 + 1) * number;
        start_index %= 100;

        if start_index == 0 {
            zeros += 1;
        }
    }

    println!("zeros: {}", zeros);
}

fn solve_b(input: &str) {
    let lists = input
        .lines()
        .map(|line| {
            let parts = line.split_at(1);
            let is_neg = parts.0.starts_with('L');
            let number: i32 = parts.1.parse().unwrap();

            (is_neg, number)
        })
        .collect::<Vec<_>>();

    let mut start_index = 50;
    let mut zeros = 0;

    for (is_neg, mut number) in lists {
        if number > 100 {
            zeros += number / 100;
            number = number % 100;
        }

        let new_number = (start_index + (((is_neg as i32) * -2 + 1) * number)).rem_euclid(100);
        // println!("index:{}, rotations:{}, zeros:{}", new_number, number, zeros);
        if (start_index != 0 && (new_number > start_index && is_neg)
            || (new_number < start_index && !is_neg)
            || new_number == 0)
        {
            zeros += 1;
        }

        start_index = new_number;
    }

    println!("b zeros: {}", zeros);
}
