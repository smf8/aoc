fn main() {
    solve_a();
    solve_b();
}

fn solve_a() {
    solve(2);
}

fn solve_b() {
    solve(1_000_000);
}

fn solve(scale: u64) {
    let map = read_input();
    let galaxies = find_galaxies(&map);

    let mut galaxy_combination: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            galaxy_combination.push((galaxies[i], galaxies[j]));
        }
    }

    let mut sum = 0u64;

    for (a, b) in galaxy_combination {
        let mut distance = 0u64;

        for x in a.0.min(b.0) + 1..=a.0.max(b.0) {
            if map[x][b.1] == '*' {
                distance += scale;
            } else {
                distance += 1;
            }
        }

        for y in a.1.min(b.1) + 1..=a.1.max(b.1) {
            if map[b.0][y] == '*' {
                distance += scale;
            } else {
                distance += 1;
            }
        }

        sum += distance;
    }

    println!("{:?}", sum);
}

fn find_galaxies(m: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (x, row) in m.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char == '#' {
                galaxies.push((x, y));
            }
        }
    }

    galaxies
}

fn read_input() -> Vec<Vec<char>> {
    let mut input = include_str!("input")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for row in input.iter_mut() {
        if row.iter().all(|ch| *ch == '.') {
            *row = vec!['*'; row.len()];
        }
    }

    'outer: for y in 0..input[0].len() {
        for (x, _) in input.iter().enumerate() {
            if input[x][y] == '#' {
                continue 'outer;
            }
        }

        for char in input.iter_mut() {
            char[y] = '*';
        }
    }

    input
}
