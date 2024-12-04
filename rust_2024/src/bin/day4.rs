fn main() {
    let example = include_str!("../../inputs/day4/example.txt");
    let main_input = include_str!("../../inputs/day4/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let input_map = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let possible_orders = [
        (1, 0),   // vertical
        (-1, 0),  // vertical reverse
        (0, 1),   // horizontal
        (0, -1),  // horizontal reverse
        (1, 1),   //diagonal -> ^
        (1, -1),  // diagonal -> \/
        (-1, -1), // diagonal <- \/
        (-1, 1),  // diagonal <- ^
    ];

    let mut result = 0;

    for x in 0..input_map.len() {
        for y in 0..input_map[0].len() {
            for order in possible_orders.iter() {
                if xmas_in_order(&input_map, (x, y), *order) {
                    // skip if char isn't X
                    if input_map[x][y] != b'X' {
                        continue;
                    }

                    result += 1;
                }
            }
        }
    }

    println!("part 1: {}", result);
}

fn xmas_in_order(map: &[&[u8]], index: (usize, usize), order: (i32, i32)) -> bool {
    let xmas = [b'X', b'M', b'A', b'S'];
    for (i, char) in xmas.iter().enumerate() {
        let (new_x, new_y) = (
            index.0 as i32 + order.0 * i as i32,
            index.1 as i32 + order.1 * i as i32,
        );

        // out of bound
        if new_y < 0 || new_x < 0 || new_y >= map.len() as i32 || new_x >= map[0].len() as i32 {
            return false;
        }

        // not in XMAS
        if map[new_x as usize][new_y as usize] != *char {
            return false;
        }
    }

    true
}

fn solve_b(input: &str) {
    let input_map = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut result = 0;

    for x in 0..input_map.len() {
        for y in 0..input_map[0].len() {
            // skip if char isn't A
            if input_map[x][y] != b'A' {
                continue;
            }

            if mas_in_order(&input_map, (x, y)) {
                result += 1;
            }
        }
    }

    println!("part 2: {}", result);
}

fn mas_in_order(map: &[&[u8]], (x, y): (usize, usize)) -> bool {
    // skip corners
    if x < 1 || x > map.len() - 2 || y < 1 || y > map[0].len() - 2 {
        return false;
    }

    let first_mas = matches!(
        (map[x - 1][y - 1], map[x][y], map[x + 1][y + 1]),
        (b'M', b'A', b'S') | (b'S', b'A', b'M')
    );

    let second_mas = matches!(
        (map[x + 1][y - 1], map[x][y], map[x - 1][y + 1]),
        (b'M', b'A', b'S') | (b'S', b'A', b'M')
    );

    first_mas && second_mas
}
