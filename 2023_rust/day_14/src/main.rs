use std::collections::{HashMap, VecDeque};

use itertools::Either;
use num::integer::gcd;

fn main() {
    solve_a();
    solve_b();
}

fn solve_b() {
    let mut map = read_input();

    let mut loads_count = HashMap::new();
    let mut cycle_len = 0;
    for i in 0..1000 {
        cycle(&mut map);

        // skip first elements to reach a state that all states are repeated
        if i > 250 {
            let sum = map
                .iter()
                .rev()
                .enumerate()
                .map(|(idx, row)| {
                    let c = row.iter().filter(|ch| **ch == 'O').count() * (idx + 1);

                    c
                })
                .sum::<usize>();
            
            // add the detected loads to find the cycle
            loads_count.entry(sum).and_modify(|e| *e += 1).or_insert(1);

            if cycle_len != 0 && (1_000_000_000 - (i + 1)) % cycle_len == 0 {
                println!("{}", sum);
                break;
            }

            // we detect a cycle when enough loads are captured 
            // in the map. to calculate the cycle length
            // we calculate GCD of load counts. when we reach a state that the gcd
            // equals the minimum value in the counts. then we can count
            // the unique numbers and their accurances in the cycle to get cycle length.
            if i > 500 {
                let gcd = loads_count
                    .values()
                    .filter(|v| **v != 1)
                    .fold(*loads_count.values().min().unwrap(), |a, x| gcd(*x, a));

                // loop until all values gcd become the maximum;
                if gcd != 1 && gcd == *loads_count.values().min().unwrap_or(&0) {
                    cycle_len = loads_count.values().fold(0, |a, x| a + (x / gcd));
                }
            }
        }
    }
}
fn solve_a() {
    let mut map = read_input();

    tilt_vertical(&mut map, false);

    let sum = map
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| {
            let c = row.iter().filter(|ch| **ch == 'O').count() * (idx + 1);

            c
        })
        .sum::<usize>();

    println!("{}", sum);
}
fn cycle(v: &mut [Vec<char>]) {
    // north
    tilt_vertical(v, false);
    // west
    tilt_horizontal(v, false);
    // south
    tilt_vertical(v, true);
    // east
    tilt_horizontal(v, true);
}

fn tilt_horizontal(v: &mut [Vec<char>], rev: bool) {
    for i in 0..v.len() {
        let mut empty_spaces = VecDeque::<usize>::new();
        let iter = if rev {
            Either::Left((0..v.len()).rev())
        } else {
            Either::Right(0..v.len())
        };

        for j in iter {
            match v[i][j] {
                '.' => empty_spaces.push_back(j),
                '#' => empty_spaces.clear(),
                'O' => {
                    if let Some(front) = empty_spaces.pop_front() {
                        (v[i][front], v[i][j]) = ('O', '.');
                        empty_spaces.push_back(j);
                    }
                }
                _ => {}
            }
        }
    }
}

fn tilt_vertical(v: &mut [Vec<char>], rev: bool) {
    for j in 0..v[0].len() {
        // what are we living for :D
        let mut empty_spaces = VecDeque::<usize>::new();
        let iter = if rev {
            Either::Left((0..v.len()).rev())
        } else {
            Either::Right(0..v.len())
        };

        for i in iter {
            match v[i][j] {
                '.' => empty_spaces.push_back(i),
                '#' => empty_spaces.clear(),
                'O' => {
                    if let Some(front) = empty_spaces.pop_front() {
                        (v[front][j], v[i][j]) = ('O', '.');
                        empty_spaces.push_back(i);
                    }
                }
                _ => {}
            }
        }
    }
}

fn read_input() -> Vec<Vec<char>> {
    let input = include_str!("input")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    input
}
