fn main() {
    let example = include_str!("../../inputs/day9/example.txt");
    let main_input = include_str!("../../inputs/day9/main.txt");

    solve_a(example);
    solve_a(main_input);
    //
    solve_b(example);
    solve_b(main_input);
}

fn read_latest_n(n: usize, blocks: &mut [u8]) -> Vec<usize> {
    let mut res = Vec::new();
    let len = blocks.len();

    let mut written = 0;
    for (j, latest_num) in blocks.iter_mut().rev().enumerate() {
        if written == n {
            break;
        }

        while *latest_num > 0 && written != n {
            res.push(len - j - 1);
            *latest_num -= 1;
            written += 1;
        }
    }

    res
}

fn solve_a(input: &str) {
    // 12345
    let numbers = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    // 1 3 5
    // (1, 0) (3, 1), (5, 2)
    let mut blocks = numbers.iter().cloned().step_by(2).collect::<Vec<_>>();

    // 2, 4
    let free_spaces = numbers
        .iter()
        .skip(1)
        .cloned()
        .step_by(2)
        .collect::<Vec<_>>();

    let mut final_res = vec![];

    let mut free_index = 0;
    let mut block_index = 0;
    loop {
        for _ in 0..blocks[block_index] {
            final_res.push(block_index);
            blocks[block_index] -= 1;
        }

        let values = read_latest_n(free_spaces[free_index] as usize, &mut blocks);

        for value in values {
            final_res.push(value);
        }

        free_index += 1;
        block_index += 1;

        if blocks[block_index] == 0 {
            break;
        }
    }

    let result = final_res
        .iter()
        .enumerate()
        .fold(0, |acc, x| acc + x.0 * (*x.1));

    println!("part 1: {:?}", result);
}

#[allow(clippy::same_item_push)]
fn solve_b(input: &str) {
    let numbers = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    let mut blocks = numbers.iter().cloned().step_by(2).collect::<Vec<_>>();
    let free_spaces = numbers
        .iter()
        .skip(1)
        .cloned()
        .step_by(2)
        .collect::<Vec<_>>();

    let zero_count = blocks.remove(0);
    let mut final_res = vec![];

    for (i, (block, free)) in blocks.iter().zip(free_spaces.iter()).enumerate() {
        for _ in 0..*free {
            final_res.push(0);
        }

        for _ in 0..*block {
            final_res.push(i + 1);
        }
    }

    let mut block_index = final_res.len();

    while let Some((block, block_pos)) = next_block(&final_res[..block_index]) {
        if let Some(free_pos) =
            next_free_position(&final_res[..=block_pos.0], block_pos.1 - block_pos.0 + 1)
        {
            for num in final_res[free_pos.0..=free_pos.1].iter_mut() {
                *num = block;
            }

            for num in final_res[block_pos.0..=block_pos.1].iter_mut() {
                *num = 0;
            }
        }

        block_index = block_pos.0;
    }

    let result = final_res
        .iter()
        .enumerate()
        .fold(0, |acc, x| acc + (x.0 + zero_count as usize) * (*x.1));

    println!("part 2: {:?}", result);
}

fn next_block(list: &[usize]) -> Option<(usize, (usize, usize))> {
    let mut number = 0;
    for (i, num) in list.iter().enumerate().rev() {
        if *num != 0 && number == 0 {
            number = *num;
            let pos = list[..i].iter().rposition(|p| *p != number);

            return pos.map(|pos| (*num, (pos + 1, i)));
        }
    }
    None
}

fn next_free_position(list: &[usize], len: usize) -> Option<(usize, usize)> {
    for (i, num) in list.iter().enumerate() {
        if i + len >= list.len() {
            break;
        }

        if *num == 0 && list[i..i + len].iter().all(|num| *num == 0) {
            return Some((i, i + len - 1));
        }
    }

    None
}
