use std::vec;

fn main() {
    let example = include_str!("../../inputs/day4/example.txt");
    let main_input = include_str!("../../inputs/day4/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;
    for (x, row) in grid.iter().enumerate() {
        for (y, ch) in row.iter().enumerate() {
            if *ch == '@' && neighbors_count(&grid, x, y) < 4 {
                result += 1;
            }
        }
    }

    println!("Result[p1]: {}", result);
}

fn neighbors_count(grid: &[Vec<char>], x_index: usize, y_index: usize) -> u32 {
    let mut count = 0;

    let moves = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in &moves {
        let x = x_index as i32 + dx;
        let y = y_index as i32 + dy;

        if x < 0 || y < 0 {
            continue;
        }

        if grid
            .get(x as usize)
            .is_some_and(|a| a.get(y as usize).is_some_and(|b| *b == '@'))
        {
            count += 1;
        }
    }

    count
}

fn solve_b(input: &str) {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;

    loop {
        let mut grid_copy = grid.clone();
        let mut removed = 0;

        for (x, row) in grid.iter().enumerate() {
            for (y, ch) in row.iter().enumerate() {
                if *ch == '@' && neighbors_count(&grid_copy, x, y) < 4 {
                    removed += 1;
                    grid_copy[x][y] = '.';
                }
            }
        }

        if removed == 0 {
            break;
        }

        result += removed;
        grid = grid_copy;
    }

    println!("Result[p2]: {}", result);
}
