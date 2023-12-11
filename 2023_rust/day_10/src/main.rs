use std::{collections::HashSet, ops::Add};

fn main() {
    solve_a();
    solve_b();
}

fn solve_b() {
    let (s_index, mut map) = read_input();

    let loop_nodes = find_loop(&mut map, &s_index);
    // find J with maximum Y and X (right, bottom corner)
    let (mut x, mut y) = (0, 0);
    let mut corner_j_index = 0;
    for (idx, node) in loop_nodes.iter().enumerate() {
        if node.ch == 'J' && (node.x > x || node.x == x && node.y > y) {
            (x, y) = (node.x, node.y);
            corner_j_index = idx;
        }
    }

    // from corner_j node
    // all tiles to the up and left are considered enclosed
    //  ....|
    //  ....|
    //  .---J
    // traversal is anticlockwise
    let mut normal_vector: (i32, i32);
    if loop_nodes[corner_j_index].dir == 'r' {
        //    |
        // <- |
        //    |
        normal_vector = (0, -1);
    } else {
        //  ^
        //  |
        // ----
        // traversal is clockwise
        normal_vector = (-1, 0);
    }

    let mut enclosed_tiles = HashSet::<(usize, usize)>::new();

    // we start traverse from corner_j
    for i in corner_j_index + 1..corner_j_index + loop_nodes.len() {
        let node = loop_nodes[i % loop_nodes.len()];
        let (mut new_x, mut new_y) = (
            (node.x as i32).add(normal_vector.0) as usize,
            (node.y as i32).add(normal_vector.1) as usize,
        );

        if map[new_x][new_y].ch != '#' {
            enclosed_tiles.insert((new_x, new_y));
        }

        match node.ch {
            'L' | '7' => {
                normal_vector = (-normal_vector.1, -normal_vector.0);
            }
            'F' | 'J' => {
                normal_vector = (normal_vector.1, normal_vector.0);
            }
            _ => continue,
        }
        // we need to consider two tiles in bends
        (new_x, new_y) = (
            (node.x as i32).add(normal_vector.0) as usize,
            (node.y as i32).add(normal_vector.1) as usize,
        );

        if map[new_x][new_y].ch != '#' {
            enclosed_tiles.insert((new_x, new_y));
        }
    }

    flood_fill(&map, &mut enclosed_tiles);

    println!("{:?}", enclosed_tiles.len());
}

fn flood_fill(map: &[Vec<Index>], tiles: &mut HashSet<(usize, usize)>) {
    let mut stack: Vec<(usize, usize)> = tiles.iter().cloned().collect();

    let neibours = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((x, y)) = stack.pop() {
        for neibour in neibours.iter() {
            let tmp_index = (
                (x as i32 + neibour.0) as usize,
                (y as i32 + neibour.1) as usize,
            );
            if map[tmp_index.0][tmp_index.1].ch != '#' && !tiles.contains(&tmp_index) {
                tiles.insert(tmp_index);
                stack.push(tmp_index);
            }
        }
    }
}

fn solve_a() {
    let (s_index, mut map) = read_input();

    let loop_nodes = find_loop(&mut map, &s_index);
    println!("{}", loop_nodes.len().div_ceil(2));
}

fn find_loop(map: &mut [Vec<Index>], s_index: &Index) -> Vec<Index> {
    let mut loop_nodes = Vec::<Index>::new();

    if let ch_var @ ('|' | 'F' | '7') = map[s_index.x - 1][s_index.y].ch {
        loop_nodes.push(Index::new(s_index.x - 1, s_index.y, ch_var, 'u'));
    }

    if let ch_var @ ('-' | '7' | 'J') = map[s_index.x][s_index.y + 1].ch {
        loop_nodes.push(Index::new(s_index.x, s_index.y + 1, ch_var, 'r'));
    }

    if let ch_var @ ('|' | 'J' | 'L') = map[s_index.x + 1][s_index.y].ch {
        loop_nodes.push(Index::new(s_index.x + 1, s_index.y, ch_var, 'd'));
    }

    if let ch_var @ ('-' | 'F' | 'L') = map[s_index.x][s_index.y - 1].ch {
        loop_nodes.push(Index::new(s_index.x, s_index.y - 1, ch_var, 'l'));
    }

    loop_nodes.pop();

    let mut current_node = &loop_nodes[0];
    loop {
        let (dir, x, y) = match (current_node.dir, current_node.ch) {
            ('r', '-') | ('d', 'L') | ('u', 'F') => ('r', current_node.x, current_node.y + 1), // right
            ('r', 'J') | ('u', '|') | ('l', 'L') => ('u', current_node.x - 1, current_node.y), // up
            ('r', '7') | ('d', '|') | ('l', 'F') => ('d', current_node.x + 1, current_node.y), // down
            ('d', 'J') | ('u', '7') | ('l', '-') => ('l', current_node.x, current_node.y - 1), // left
            _ => ('.', 0, 0),
        };

        map[current_node.x][current_node.y].ch = '#';

        if map[x][y].ch == 'S' {
            // we need to detect and replace S for part2
            let s_char = match (dir, loop_nodes[0].dir) {
                ('r', 'l') | ('l', 'r') => '-',
                ('r', 'u') | ('d', 'l') => 'J',
                ('r', 'd') | ('u', 'l') => '7',
                ('l', 'u') | ('d', 'r') => 'L',
                ('l', 'd') | ('u', 'r') => 'F',
                ('u', 'd') | ('d', 'u') => '|',
                _ => '.',
            };

            let new_index = Index::new(x, y, s_char, dir);
            loop_nodes.push(new_index);

            map[x][y].ch = '#';

            break;
        }

        let new_index = Index::new(x, y, map[x][y].ch, dir);
        loop_nodes.push(new_index);

        current_node = &loop_nodes.last().unwrap();
    }

    loop_nodes
}

#[derive(Clone, Copy, Debug)]
struct Index {
    x: usize,
    y: usize,
    ch: char,
    dir: char,
}

impl Index {
    fn new(x: usize, y: usize, ch: char, dir: char) -> Self {
        Index { x, y, ch, dir }
    }
}

fn read_input() -> (Index, Vec<Vec<Index>>) {
    let mut s_location: Index = Index {
        x: 0,
        y: 0,
        ch: 'S',
        dir: '.',
    };

    let tmp_index = Index::new(0, 0, '.', '.');

    let mut input = include_str!("input")
        .lines()
        .enumerate()
        .map(|(x, line)| {
            let mut chars = line
                .chars()
                .enumerate()
                .map(|(y, ch)| {
                    if ch == 'S' {
                        s_location.x = x + 1;
                        s_location.y = y + 1;
                    }

                    Index {
                        x: x + 1,
                        y: y + 1,
                        ch,
                        dir: '.',
                    }
                })
                .collect::<Vec<_>>();

            chars.insert(0, tmp_index);
            chars.push(tmp_index);
            chars
        })
        .collect::<Vec<_>>();

    input.push(vec![tmp_index; input[0].len()]);
    input.insert(0, vec![tmp_index; input[0].len()]);

    (s_location, input)
}
