use grid::Grid;
use regex::Regex;
use std::cmp::Ordering;

fn main() {
    // let example = include_str!("../../inputs/day14/example.txt");
    let main_input = include_str!("../../inputs/day14/main.txt");

    // solve_a(example);
    solve_a(main_input);

    // solve_b(example);
    solve_b(main_input);
}

#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_map(input: &str) -> (Grid<i32>, Vec<Robot>) {
    // for example size is 7 x 11
    // let map = Grid::init(103, 101, None);
    let mut map = Grid::init(103, 101, 0);
    let mut robots = Vec::new();
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let robot = Robot {
            position: (
                captures[2].parse::<i32>().unwrap(),
                captures[1].parse::<i32>().unwrap(),
            ),
            velocity: (
                captures[4].parse::<i32>().unwrap(),
                captures[3].parse::<i32>().unwrap(),
            ),
        };

        let (row, col) = (robot.position.0 as usize, robot.position.1 as usize);

        map[(row, col)] += 1;
        robots.push(robot);
    }

    (map, robots)
}

fn advance_grid(map: &mut Grid<i32>, robots: &mut Vec<Robot>) {
    let (rows, cols) = (map.rows() as i32, map.cols() as i32);
    for robot in robots {
        let (new_row, new_col) = (
            (robot.position.0 + robot.velocity.0).rem_euclid(rows),
            (robot.position.1 + robot.velocity.1).rem_euclid(cols),
        );
        let (row, col) = (robot.position.0, robot.position.1);
        robot.position = (new_row, new_col);

        map[(row as usize, col as usize)] -= 1;
        map[(new_row as usize, new_col as usize)] += 1;
    }
}

fn solve_a(input: &str) {
    let (mut map, mut robots) = parse_map(input);

    for _ in 0..100 {
        advance_grid(&mut map, &mut robots);
    }

    // a:
    // 0 | 1
    // -----
    // 2 | 3
    let mut a = (0, 0, 0, 0);
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            let len = map[(row, col)];
            if len != 0 {
                match (row.cmp(&(map.rows() / 2)), col.cmp(&(map.cols() / 2))) {
                    (Ordering::Equal, _) | (_, Ordering::Equal) => {}
                    (Ordering::Less, Ordering::Less) => {
                        a.0 += len;
                    }
                    (Ordering::Less, Ordering::Greater) => {
                        a.1 += len;
                    }
                    (Ordering::Greater, Ordering::Less) => {
                        a.2 += len;
                    }
                    (Ordering::Greater, Ordering::Greater) => {
                        a.3 += len;
                    }
                }
            }
        }
    }

    let result = a.0 * a.1 * a.2 * a.3;

    println!("part 1: {:?}", result);
}

fn solve_b(input: &str) {
    let (mut map, mut robots) = parse_map(input);
    for i in 0..10000 {
        advance_grid(&mut map, &mut robots);
        draw_grid(&map, i);
    }
}

fn is_tree_head(map: &Grid<i32>, (row, col): (usize, usize)) -> bool {
    let is_non_zero = |map: &Grid<i32>, row: i32, col: i32| -> bool {
        if let Some(num) = map.get(row, col) {
            *num != 0
        } else {
            false
        }
    };

    let mut result = true;
    for i in 0..4 {
        result &= is_non_zero(map, (row + i) as i32, (col + i) as i32)
            && is_non_zero(map, (row + i) as i32, (col - i) as i32)
    }

    result
}

fn draw_grid(map: &Grid<i32>, i: i32) {
    let mut line = String::new();
    let mut should_draw = false;
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            //...#...
            //..#.#..
            //.#...#.
            //#.....#
            if is_tree_head(map, (row, col)) {
                should_draw = true;
            }

            if map[(row, col)] == 0 {
                line.push('.')
            } else {
                line.push('#')
            }
        }
        line.push('\n');
    }

    if should_draw {
        println!("after {}s", i + 1);
        println!("{}", line);
        println!("\n");
    }
}
