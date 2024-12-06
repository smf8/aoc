use grid::{grid, Grid};
use std::collections::HashSet;

fn main() {
    let example = include_str!("../../inputs/day6/example.txt");
    let main_input = include_str!("../../inputs/day6/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn parse_map(input: &str) -> (Grid<char>, (i32, i32)) {
    let mut map = grid![];
    let (mut row, mut col) = (0, 0);
    for (x, line) in input.lines().enumerate() {
        map.push_row(line.chars().collect());

        if let Some(y) = line.find("^") {
            (row, col) = (x, y)
        }
    }

    (map, (row as i32, col as i32))
}

fn traverse(map: &mut Grid<char>, (mut row, mut col): (i32, i32)) {
    let mut dir = (-1, 0);
    loop {
        if row < 0 || row >= map.rows() as i32 || col < 0 || col >= map.cols() as i32 {
            break;
        }

        if map[(row as usize, col as usize)] == '#' {
            // revert index change
            (row, col) = (row - dir.0, col - dir.1);

            // turn 90 degrees
            dir = (dir.1, -dir.0);
        }

        map[(row as usize, col as usize)] = 'X';

        (row, col) = (row + dir.0, col + dir.1);
    }
}

fn solve_a(input: &str) {
    let (mut map, (row, col)) = parse_map(input);

    traverse(&mut map, (row, col));

    let visited = map.iter().filter(|char| **char == 'X').count();

    println!("part 1: {}", visited);
}

fn solve_b(input: &str) {
    let (mut map, (mut row, mut col)) = parse_map(input);
    let mut dir = (-1, 0);
    let mut loops = 0;
    loop {
        if row < 0 || row >= map.rows() as i32 || col < 0 || col >= map.cols() as i32 {
            break;
        }

        let char = map[(row as usize, col as usize)];

        if char == '.' {
            map[(row as usize, col as usize)] = '#';

            if detect_loop(&mut map, (row - dir.0, col - dir.1), dir) {
                loops += 1;
            }

            map[(row as usize, col as usize)] = '.';
        }

        if char == '#' {
            // revert index change
            (row, col) = (row - dir.0, col - dir.1);

            // turn 90 degrees
            dir = (dir.1, -dir.0);
        }

        map[(row as usize, col as usize)] = 'X';

        (row, col) = (row + dir.0, col + dir.1);
    }

    println!("part 2: {}", loops)
}

// detect_loop detects if moving at (row, col) with 90 degree of dir will cause a loop
fn detect_loop(map: &mut Grid<char>, (mut row, mut col): (i32, i32), mut dir: (i32, i32)) -> bool {
    let mut visited = HashSet::new();
    // rotate
    dir = (dir.1, -dir.0);
    loop {
        if visited.contains(&((row, col), dir)) {
            return true;
        }

        visited.insert(((row, col), dir));
        if map[(row as usize, col as usize)] == '#' {
            // revert index change
            (row, col) = (row - dir.0, col - dir.1);

            // turn 90 degrees
            dir = (dir.1, -dir.0);

            continue;
        }

        (row, col) = (row + dir.0, col + dir.1);

        if row < 0 || row >= map.rows() as i32 || col < 0 || col >= map.cols() as i32 {
            return false;
        }
    }
}
