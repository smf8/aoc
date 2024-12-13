use regex::Regex;

fn main() {
    let example = include_str!("../../inputs/day13/example.txt");
    let main_input = include_str!("../../inputs/day13/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

struct Puzzle {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Puzzle {
    fn solve(&self, part2: bool) -> Option<(i64, i64)> {
        let y = (self.a.0 * self.prize.1 - self.prize.0 * self.a.1) as f64
            / (self.a.0 * self.b.1 - self.b.0 * self.a.1) as f64;
        if y.floor() != y || y < 0.0 {
            return None;
        }

        let x = (self.prize.0 - (y as i64 * self.b.0)) as f64 / self.a.0 as f64;

        if x.floor() != x || x < 0.0 {
            return None;
        }

        if !part2 && (x > 100.0 || y > 100.0) {
            return None;
        }

        Some((x as i64, y as i64))
    }
}
fn parse(input: &str) -> Vec<Puzzle> {
    let re = Regex::new(r"Button\sa:\sX\+(\d+),\sY\+(\d+)\nButton\sb:\sX\+(\d+),\sY\+(\d+)\nprize:\sX=(\d+),\sY=(\d+)").unwrap();

    let result = input
        .split("\n\n")
        .map(|puzzle_str| {
            let captures = re.captures(puzzle_str).unwrap();

            Puzzle {
                a: (
                    captures[1].parse::<i64>().unwrap(),
                    captures[2].parse::<i64>().unwrap(),
                ),
                b: (
                    captures[3].parse::<i64>().unwrap(),
                    captures[4].parse::<i64>().unwrap(),
                ),
                prize: (
                    captures[5].parse::<i64>().unwrap(),
                    captures[6].parse::<i64>().unwrap(),
                ),
            }
        })
        .collect::<Vec<_>>();

    result
}

fn solve_a(input: &str) {
    let puzzles = parse(input);

    let mut result = 0;
    for puzzle in puzzles.iter() {
        if let Some((a, b)) = puzzle.solve(false) {
            result += a * 3 + b;
        }
    }

    println!("part 1: {}", result)
}

fn solve_b(input: &str) {
    let mut puzzles = parse(input);

    let mut result = 0;
    for puzzle in puzzles.iter_mut() {
        puzzle.prize = (
            puzzle.prize.0 + 10000000000000,
            puzzle.prize.1 + 10000000000000,
        );
        if let Some((a, b)) = puzzle.solve(true) {
            result += a * 3 + b;
        }
    }

    println!("part 2: {}", result)
}
