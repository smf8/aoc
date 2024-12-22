use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::ops::BitXor;

fn main() {
    let example = include_str!("../../inputs/day22/example.txt");
    let main_input = include_str!("../../inputs/day22/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

struct Rnd{
    seed: usize,
}

impl Rnd {
    fn next(&mut self) -> usize{
        // first step
        let mut result = self.seed * 64;
        result ^= self.seed;
        result %= 16777216;
        // second
        let mut second_result = result.div_euclid(32);
        second_result ^= result;
        second_result %= 16777216;
        // third
        let mut third_result = second_result * 2048;
        third_result ^= second_result;
        third_result %= 16777216;

        self.seed = third_result;

        third_result
    }
}

fn solve_a(input: &str) {
    let numbers = input.lines().map(|line| line.parse::<usize>().unwrap());

    let mut result = 0;
    for number in numbers{
        let mut r = Rnd{
            seed: number,
        };

        for _ in 0..2000{
            r.next();
        }

        result += r.seed;
    }

    println!("part 1: {}", result);
}

fn solve_b(input: &str) {let numbers = input.lines().map(|line| line.parse::<usize>().unwrap());
    let mut numbers_seq = Vec::new();
    let mut all_sequences = HashSet::new();

    for number in numbers{
        let mut map = HashMap::new();
        let mut sequence = Vec::with_capacity(2000);
        let mut r = Rnd{
            seed: number,
        };

        for _ in 0..2000{
            sequence.push(r.seed % 10);
            r.next();
        }

        let diff_sequence = sequence.windows(2).map(|seq| seq[1] as i32 - seq[0] as i32).collect::<Vec<_>>();

        for (i, window) in diff_sequence.windows(4).enumerate(){
            let window_arr = [window[0], window[1], window[2], window[3]];

            map.entry(window_arr).or_insert(sequence[i + 4]);
            all_sequences.insert(window_arr);
        }

        numbers_seq.push(map);
    }

    let mut max_sequence = [0; 4];
    let mut final_bananas = 0;
    for seq in all_sequences{
        let mut maximum_bananas = 0;
        for number_seq in numbers_seq.iter(){
            maximum_bananas += number_seq.get(&seq).unwrap_or(&0);
        }

        if maximum_bananas > final_bananas{
            final_bananas = maximum_bananas;
            max_sequence = seq;
        }
    }

    println!("part 2: {:?}: {}", max_sequence, final_bananas);
}