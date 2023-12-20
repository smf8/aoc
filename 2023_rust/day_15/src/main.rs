use std::collections::{HashMap, VecDeque};

fn main() {
    solve_a();
    solve_b();
}

fn solve_b() {
    let mut words = read_input();

    for word in words.iter_mut() {
        word.compute_hash_part_b();
    }

    let mut boxes = HashMap::new();

    for word in words.iter() {
        let box_vec: &mut Vec<(&str, usize)> = boxes
            .entry(word.hash)
            .or_insert(Vec::<(&str, usize)>::new());

        if word.operation == '-' {
            box_vec.retain(|i: &(&str, usize)| i.0 != word.label);
        } else if word.operation == '=' {
            if let Some(pos) = box_vec.iter().position(|i| i.0 == word.label) {
                box_vec[pos].1 = word.focal_length;
            } else {
                box_vec.push((&word.label, word.focal_length));
            }
        }
    }

    let sum = boxes
        .iter()
        .fold(0, |a: usize, x: (&usize, &Vec<(&str, usize)>)| {
            a + (x.0 + 1)
                * x.1
                    .iter()
                    .enumerate()
                    .fold(0, |product: usize, lens: (usize, &(&str, usize))| {
                        product + ((lens.0 + 1) * lens.1 .1)
                    })
        });

    println!("{}", sum);
}

fn solve_a() {
    let mut words = read_input();

    for word in words.iter_mut() {
        word.compute_hash_part_a();
    }

    let sum = words.iter().map(|w| w.hash).sum::<usize>();

    println!("{}", sum);
}

struct Word {
    word: String,
    label: String,
    focal_length: usize,
    operation: char,
    hash: usize,
}

impl Word {
    fn new(str: String) -> Word {
        let split = str.split(['-', '=']).collect::<Vec<_>>();

        let (label, focal_length, operation) = if str.contains('-') {
            (split[0].to_string(), 0, '-')
        } else {
            // =
            (
                split[0].to_string(),
                split[1].parse::<usize>().unwrap(),
                '=',
            )
        };

        Word {
            word: str,
            hash: 0,
            label,
            focal_length,
            operation,
        }
    }

    fn compute_hash_part_a(&mut self) {
        self.hash = self
            .word
            .chars()
            .fold(0usize, |a: usize, x| ((a + x as usize) * 17) % 256);
    }

    fn compute_hash_part_b(&mut self) {
        self.hash = self
            .label
            .chars()
            .fold(0usize, |a: usize, x| ((a + x as usize) * 17) % 256);
    }
}

fn read_input() -> Vec<Word> {
    let input = include_str!("input")
        .trim()
        .split(',')
        .map(|w| Word::new(w.to_string()))
        .collect::<Vec<_>>();

    input
}
