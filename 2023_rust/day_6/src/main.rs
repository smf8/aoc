fn main() {
    solve_a();
    solve_b();
}

fn solve_a() {
    let (times, records) = read_input();

    let result = times
        .iter()
        .zip(records.iter())
        .map(|(time, record)| {
            (0..=*time)
                .filter(|hold_time| hold_time * (*time - hold_time) > *record)
                .count()
        })
        .product::<usize>();

    println!("{}", result);
}
fn solve_b() {
    let (times, records) = read_input();

    let (time, record) = times.iter().zip(records.iter()).fold((0, 0), |a, x| {
        (
            a.0 * 10u64.pow(x.0.ilog10() + 1) + *x.0 as u64,
            a.1 * 10u64.pow(x.1.ilog10() + 1) + *x.1 as u64,
        )
    });

    // solutions to x . ( time - x) > record
    let x = ((time as f64 - ((time.pow(2) - 4 * record) as f64).sqrt()) / 2.0).ceil() as u64;

    println!("{}", time - x * 2 + 1);
}

fn read_input() -> (Vec<u32>, Vec<u32>) {
    let input = include_str!("input");

    let times = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|a| a.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let records = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .filter_map(|a| a.parse::<u32>().ok())
        .collect::<Vec<_>>();

    (times, records)
}
