use std::collections::HashSet;
use std::vec;

fn main() {
    let example = include_str!("../../inputs/day6/example.txt");
    let main_input = include_str!("../../inputs/day6/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

#[derive(Debug, Clone)]
struct Question {
    numbers: Vec<usize>,
    operation: char,
}

impl Question {
    pub fn default() -> Question {
        Question {
            operation: '-',
            numbers: Vec::new(),
        }
    }

    pub fn solve(&self) -> usize {
        if self.operation == '+' {
            self.numbers.iter().fold(0, |acc, x| acc + *x)
        } else {
            self.numbers.iter().fold(1, |acc, x| acc * *x)
        }
    }

    pub fn solve_p2(&mut self) -> usize {
        let max = self.numbers.iter().max().unwrap();
        let max_digit_len = max.ilog10() as usize + 1;

        let mut result = 0;
        if self.operation == '*' {
            result = 1;
        }

        for (i, _) in (0..max_digit_len).enumerate() {
            let mut temp_number = 0;
            for num in self.numbers.iter_mut() {
                if *num != 0 {
                    if *num % 10 != 0 {
                        temp_number *= 10;
                        temp_number += *num % 10;
                    }

                    *num = *num / 10;
                }
            }

            if self.operation == '+' {
                result += temp_number;
            } else {
                result *= temp_number;
            }
        }

        result
    }
}

fn solve_a(input: &str) {
    let input_lines = input.lines().collect::<Vec<_>>();

    let mut questions: Vec<Question> = Vec::new();

    for line in input_lines[..input_lines.len() - 1].iter() {
        let numbers = line
            .split_whitespace()
            .map(|a| a.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if questions.len() == 0 {
            questions = vec![Question::default(); numbers.len()]
        }

        for (index, num) in numbers.iter().enumerate() {
            questions[index].numbers.push(*num);
        }
    }

    for (index, char) in input_lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|a| a.chars().next().unwrap())
        .enumerate()
    {
        questions[index].operation = char;
    }

    let mut result = 0;

    for question in &questions {
        result += question.solve();
    }

    println!("Part1: {}", result);
}

fn solve_b(input: &str) {
    let input_lines = input.lines().collect::<Vec<_>>();

    let max_line_length = input_lines.iter().map(|line| line.len()).max().unwrap();

    let mut widths: Vec<usize> = Vec::new();

    let mut last_op_index = 0;
    for (i, ch) in input_lines.last().unwrap().chars().enumerate() {
        if i == 0 {
            continue;
        }

        if ch == '+' || ch == '*' {
            widths.push(i - last_op_index - 1);

            last_op_index = i;
        }
    }

    widths.push(max_line_length - last_op_index);

    let mut questions = vec![Question::default(); widths.len()];

    for (index, char) in input_lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|a| a.chars().next().unwrap())
        .enumerate()
    {
        questions[index].operation = char;
    }

    for line in input_lines[..input_lines.len() - 1].iter() {
        let mut start_index = 0;
        for (q_index, width) in widths.iter().enumerate() {
            let mut num_str = &line[start_index..start_index + width];
            if num_str.starts_with(' ') {
                num_str = num_str.trim_start();
                // right aligned
                questions[q_index].numbers.push(num_str.parse().unwrap());
            } else {
                num_str = num_str.trim_end();

                let mut number = num_str.parse::<usize>().unwrap();
                let digit_diff = *width - number.ilog10() as usize - 1;
                number = number * 10_usize.pow(digit_diff as u32);

                questions[q_index].numbers.push(number);
            }

            start_index += width + 1;
        }
    }

    let mut result = 0;
    for question in questions.iter_mut() {
        result += question.solve_p2();
    }

    println!("Part2: {}", result);
}
