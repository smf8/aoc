use std::vec;

fn main() {
    solve_a();
    solve_b();
}

fn solve_b() {
    let map = read_input();
    let mut max = 0;
    let directions = vec![
        ((0, 1), ((0..map.len()).collect::<Vec<usize>>(), vec![0])),
        (
            (0, -1),
            ((0..map.len()).collect::<Vec<usize>>(), vec![map.len() - 1]),
        ),
        ((1, 0), (vec![0], (0..map.len()).collect::<Vec<usize>>())),
        (
            (-1, 0),
            (vec![map.len() - 1], (0..map.len()).collect::<Vec<usize>>()),
        ),
    ];

    for dir in directions {
        for x in dir.1 .0.iter() {
            for y in dir.1 .1.iter() {
                let mut map = read_input();

                let mut visited = vec![vec![0; map[0].len()]; map.len()];

                move_beam((*x as i32, *y as i32), dir.0, &mut map, &mut visited);

                let count = visited.iter().flatten().filter(|cnt| **cnt != 0).count();

                if count > max {
                    max = count;
                }
            }
        }
    }

    println!("{}", max);
}

fn solve_a() {
    let mut map = read_input();

    let mut visited = vec![vec![0; map[0].len()]; map.len()];

    move_beam((0, 0), (0, 1), &mut map, &mut visited);

    let count = visited.iter().flatten().filter(|cnt| **cnt != 0).count();

    println!("{}", count);
}

fn move_beam(
    mut idx: (i32, i32),
    mut direction: (i32, i32),
    map: &mut [Vec<char>],
    visited: &mut [Vec<usize>],
) {
    loop {
        if map.get(idx.0 as usize).is_none() || map.get(0).unwrap().get(idx.1 as usize).is_none() {
            break;
        }

        visited[idx.0 as usize][idx.1 as usize] += 1;

        match (map[idx.0 as usize][idx.1 as usize], direction) {
            // inter from sides
            ('|', (0, 1 | -1)) => {
                move_beam((idx.0 + 1, idx.1), (1, 0), map, visited);
                move_beam((idx.0 - 1, idx.1), (-1, 0), map, visited);
                break;
            }
            ('|', _) => {
                idx = (idx.0 + direction.0, idx.1);
            }
            ('-', (1 | -1, 0)) => {
                move_beam((idx.0, idx.1 + 1), (0, 1), map, visited);
                move_beam((idx.0, idx.1 - 1), (0, -1), map, visited);
                break;
            }
            ('-', _) => {
                idx = (idx.0, idx.1 + direction.1);
            }
            ('/', _) => {
                idx = (idx.0 + -direction.1, idx.1 + -direction.0);
                direction = (-direction.1, -direction.0);
            }
            ('\\', _) => {
                idx = (idx.0 + direction.1, idx.1 + direction.0);
                direction = (direction.1, direction.0);
            }
            ('v', (0, -1 | 1)) | ('h', (-1 | 1, 0)) => {
                // we have reached a loop
                break;
            }
            (_, _) => {
                // note if the point is visited vertically or horizontally
                if direction.0 != 0 {
                    // horizontally
                    map[idx.0 as usize][idx.1 as usize] = 'h';
                } else {
                    map[idx.0 as usize][idx.1 as usize] = 'v';
                }
                // map[idx.0 as usize][idx.1 as usize] = '#';
                idx = (idx.0 + direction.0, idx.1 + direction.1);
            }
        }
    }
}

fn read_input() -> Vec<Vec<char>> {
    let input = include_str!("input")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    input
}
