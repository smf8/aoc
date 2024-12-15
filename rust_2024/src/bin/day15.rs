use grid::{grid, Grid};
use std::collections::HashSet;

fn main() {
    let example = include_str!("../../inputs/day15/example.txt");
    let main_input = include_str!("../../inputs/day15/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn new(ch: char) -> Self {
        match ch {
            '>' => Dir::Right,
            '<' => Dir::Left,
            '^' => Dir::Up,
            'v' => Dir::Down,
            _ => panic!("here we go again!"),
        }
    }

    fn delta(&self) -> (i32, i32) {
        match *self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
}

fn parse_map(input: &str, part_two: bool) -> (Grid<char>, Vec<Dir>, (i32, i32)) {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let mut map = grid![];
    let mut starting_point = (0, 0);
    let mut movements = Vec::new();

    for (row, line) in input[0].lines().enumerate() {
        let mut row_vec = vec![];
        for (col, ch) in line.chars().enumerate() {
            if part_two {
                match ch {
                    'O' => row_vec.append(&mut vec!['[', ']']),
                    '#' => row_vec.append(&mut vec!['#', '#']),
                    '.' => row_vec.append(&mut vec!['.', '.']),
                    '@' => {
                        row_vec.append(&mut vec!['@', '.']);
                        starting_point = (row as i32, (col * 2) as i32);
                    }
                    _ => panic!("🙀"),
                }
            } else {
                row_vec.push(ch);
                if ch == '@' {
                    starting_point = (row as i32, col as i32);
                }
            }
        }

        map.push_row(row_vec);
    }

    for line in input[1].lines() {
        for ch in line.chars() {
            movements.push(Dir::new(ch))
        }
    }

    (map, movements, starting_point)
}

fn solve_a(input: &str) {
    let (mut map, movements, mut start_pos) = parse_map(input, false);

    for movement in movements {
        start_pos = move_bot(&mut map, start_pos, movement, false);
    }

    let mut result = 0;
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if map[(row, col)] == 'O' {
                result += row * 100 + col;
            }
        }
    }

    print_map(&map);
    println!("{:?}", result);
}

fn move_bot(map: &mut Grid<char>, pos: (i32, i32), dir: Dir, part2: bool) -> (i32, i32) {
    let delta = dir.delta();
    // tiles_to_move are positions of O or [ tiles.
    let mut tiles_to_move = vec![];
    let mut tiles_to_check = vec![(pos.0, pos.1)];
    let mut checked_tiles = HashSet::new();
    while let Some(tile) = tiles_to_check.pop() {
        if checked_tiles.contains(&tile) {
            continue;
        }

        checked_tiles.insert(tile);
        match (dir, map[(tile.0 as usize, tile.1 as usize)]) {
            (Dir::Up | Dir::Down, '[') => {
                tiles_to_move.push(tile);
                tiles_to_check.append(&mut vec![
                    (tile.0 + delta.0, tile.1 + delta.1),
                    (tile.0, tile.1 + 1),
                ]);
            }
            (Dir::Up | Dir::Down, ']') => {
                tiles_to_check.append(&mut vec![
                    (tile.0 + delta.0, tile.1 + delta.1),
                    (tile.0, tile.1 - 1),
                ]);
            }
            // for part 1
            (_, 'O') => {
                tiles_to_move.push(tile);
                tiles_to_check.push((tile.0 + delta.0, tile.1 + delta.1));
            }
            (_, '[') => {
                tiles_to_move.push(tile);
                tiles_to_check.push((tile.0 + delta.0, tile.1 + delta.1));
            }
            (_, ']' | '@') => {
                tiles_to_check.push((tile.0 + delta.0, tile.1 + delta.1));
            }
            (_, '#') => {
                return pos;
            }
            (_, '.') => {}
            _ => panic!("oh no"),
        }
    }

    for tile in tiles_to_move.iter() {
        map[(tile.0 as usize, tile.1 as usize)] = '.';
        if part2 {
            map[(tile.0 as usize, (tile.1 + 1) as usize)] = '.';
        }
    }

    for tile in tiles_to_move.iter() {
        let new_pos = (tile.0 + delta.0, tile.1 + delta.1);
        if part2 {
            map[(new_pos.0 as usize, new_pos.1 as usize)] = '[';
            map[(new_pos.0 as usize, (new_pos.1 + 1) as usize)] = ']';
        } else {
            map[(new_pos.0 as usize, new_pos.1 as usize)] = 'O';
        }
    }

    map[((pos.0 + delta.0) as usize, (pos.1 + delta.1) as usize)] = '@';
    map[(pos.0 as usize, pos.1 as usize)] = '.';

    (pos.0 + delta.0, pos.1 + delta.1)
}
// 1428367 too low
fn solve_b(input: &str) {
    let (mut map, movements, mut start_pos) = parse_map(input, true);

    for movement in movements.iter() {
        start_pos = move_bot(&mut map, start_pos, *movement, true);
    }

    let mut result = 0;
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if map[(row, col)] == '[' {
                result += row * 100 + col;
            }
        }
    }

    print_map(&map);
    println!("{:?}", result);
}

fn print_map(map: &Grid<char>) {
    let mut str = String::new();
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            str.push(map[(row, col)]);
        }
        str.push('\n');
    }

    println!("{}", str);
}
