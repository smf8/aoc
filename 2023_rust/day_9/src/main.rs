fn main() {
    solve_a_b();
}

fn solve_a_b() {
    let numbers = read_input();
    let sum = numbers
        .iter()
        .map(|series| yield_number(series))
        .fold((0, 0), |a, x| (a.0 + x.0, a.1 + x.1));

    println!("a: {}\nb: {}", sum.1, sum.0);
}

// yield_number will produce the missing numbers recursively.
// performance: since it will recurse 2 times in each call.
// it's not very performant.
fn yield_number(series: &[i32]) -> (i32, i32) {
    if series.iter().all(|a| *a == 0) {
        return (0, 0);
    }

    (
        series.first().unwrap()
            - yield_number(&series.windows(2).map(|a| a[1] - a[0]).collect::<Vec<_>>()).0,
        series.last().unwrap()
            + yield_number(&series.windows(2).map(|a| a[1] - a[0]).collect::<Vec<_>>()).1,
    )
}

fn read_input() -> Vec<Vec<i32>> {
    let input = include_str!("input")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    input
}
