use std::cmp::Ordering;
use std::collections::HashSet;

fn main() {
    let example = include_str!("../../inputs/day5/example.txt");
    let main_input = include_str!("../../inputs/day5/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

#[allow(clippy::type_complexity)]
fn split_ordered(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>, HashSet<(i32, i32)>) {
    let (rules, orders) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let rule_line = line
                .split("|")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (rule_line[0], rule_line[1])
        })
        .collect::<HashSet<(i32, i32)>>();

    let orders = orders
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (orderd, not_ordered) = orders.into_iter().partition(|order| {
        for x_i in 0..order.len() {
            for y_i in x_i + 1..order.len() {
                let (x, y) = (order[x_i], order[y_i]);
                if rules.contains(&(y, x)) {
                    return false;
                }
            }
        }

        true
    });

    (orderd, not_ordered, rules)
}
fn solve_a(input: &str) {
    let (ordered, _, _) = split_ordered(input);

    let result = ordered
        .iter()
        .fold(0, |acc, x| acc + x.get(x.len() / 2).unwrap());

    println!("part 1: {}", result);
}

// 1|2
// 2|3
// 3|4

fn solve_b(input: &str) {
    let (_, mut not_ordered, rules) = split_ordered(input);

    for order in not_ordered.iter_mut() {
        order.sort_by(|&x, &y| match rules.contains(&(y, x)) {
            true => Ordering::Greater,
            false => Ordering::Less,
        })
    }

    // alternative solution
    // for order in not_ordered.iter_mut() {
    //     for x_i in 0..order.len() {
    //         for y_i in x_i + 1..order.len() {
    //             let (x, y) = (order[x_i], order[y_i]);
    //             if rules.contains(&(y, x)) {
    //                 order.swap(x_i, y_i);
    //             }
    //         }
    //     }
    // }

    let result = not_ordered
        .iter()
        .fold(0, |acc, x| acc + x.get(x.len() / 2).unwrap());

    println!("part 2: {}", result);
}
