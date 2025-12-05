fn main() {
    let example = include_str!("../../inputs/day3/example.txt");
    let main_input = include_str!("../../inputs/day3/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let ranges = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let mut total_voltage = 0;
    for pack in ranges {
        let (mut b1, mut b2) = (pack[0], 0);
        for (idx, battery) in pack.iter().enumerate() {
            if idx < pack.len() - 1 {
                if *battery > b1 {
                    b1 = *battery;
                    b2 = 0;
                    continue;
                }
            }

            if idx > 0 {
                b2 = b2.max(*battery);
            }
        }

        total_voltage += b1 * 10 + b2;
    }

    println!("total_voltage: {}", total_voltage);
}

fn solve_b(input: &str) {
    let ranges = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let mut total_voltage = 0;
    for pack in ranges {
        let mut max_batteries = vec![0; 12];
        let (mut start_index, mut end_index) = (0, 0);
        for idx in 0..12 {
            let end_index = pack.len() - (12 - idx);
            let (max_index, max) = max_available(&pack[start_index..=end_index]);
            start_index += max_index + 1;
            max_batteries[idx] = max;
        }

        let mut joltage: usize = 0;
        for (i, bat) in max_batteries.iter().enumerate() {
            joltage += (*bat as usize) * (10_usize.pow(12 - i as u32 - 1))
        }

        total_voltage += joltage
    }

    println!("total_joltage p2: {}", total_voltage);
}

fn max_available(input: &[u32]) -> (usize, u32) {
    let (mut max_idx, mut max) = (0, 0);

    for (idx, num) in input.iter().enumerate() {
        if *num > max {
            max = *num;
            max_idx = idx;
        }
    }

    (max_idx, max)
}
