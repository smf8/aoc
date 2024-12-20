use petgraph::graph::NodeIndex;
use petgraph::{algo, Graph, Undirected};
use std::collections::HashMap;

fn main() {
    let example = include_str!("../../inputs/day20/example.txt");
    let main_input = include_str!("../../inputs/day20/main.txt");

    solve_a(example);
    solve_a(main_input);
    // //
    solve_b(example);
    solve_b(main_input);
}

#[derive(Clone)]
struct Map {
    graph: Graph<(i32, i32), (), Undirected>,
    graph_indices: HashMap<(i32, i32), NodeIndex>,
    start: (i32, i32),
    end: (i32, i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn all() -> &'static [Dir] {
        &[Dir::Up, Dir::Down, Dir::Left, Dir::Right]
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

fn parse_map(input: &str) -> Map {
    let mut graph: Graph<(i32, i32), (), Undirected> = Graph::new_undirected();
    let mut indices: HashMap<(i32, i32), NodeIndex> = HashMap::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '#' {
                let index = graph.add_node((row as i32, col as i32));
                indices.insert((row as i32, col as i32), index);

                if ch == 'S' {
                    start = (row as i32, col as i32)
                }

                if ch == 'E' {
                    end = (row as i32, col as i32)
                }
            }
        }
    }

    for n in indices.keys() {
        for &dir in Dir::all() {
            let delta = dir.delta();
            let new_pos = (n.0 + delta.0, n.1 + delta.1);

            if indices.contains_key(&new_pos) {
                graph.update_edge(indices[n], indices[&new_pos], ());
            }
        }
    }

    Map {
        graph,
        graph_indices: indices,
        start,
        end,
    }
}

// too low
fn solve(input: &str, cheat_len: i32) {
    let map = parse_map(input);

    let mut paths = algo::all_simple_paths::<Vec<_>, _>(
        &map.graph,
        map.graph_indices[&map.start],
        map.graph_indices[&map.end],
        0,
        None,
    )
    .collect::<Vec<_>>();

    let path = paths.remove(0);
    let mut shortcut = HashMap::new();
    for (i, first_node) in path.iter().enumerate() {
        for (j, second_node) in path[i..].iter().enumerate() {
            let j = j + i;
            let first_pos = *map.graph.node_weight(*first_node).unwrap();
            let second_pos = *map.graph.node_weight(*second_node).unwrap();
            let (row_diff, col_diff) = (
                (first_pos.0 - second_pos.0).abs(),
                (first_pos.1 - second_pos.1).abs(),
            );

            if row_diff + col_diff <= cheat_len {
                let new_path_len = j - i - (row_diff + col_diff) as usize;
                shortcut
                    .entry(new_path_len)
                    .or_insert(Vec::new())
                    .push((i, j));
            }
        }
    }

    let result = shortcut
        .iter()
        .filter_map(|(len, vec)| {
            if *len >= 100 {
                return Some(vec.len());
            }
            None
        })
        .sum::<usize>();

    println!("{:?}", result);
}

fn solve_a(input: &str) {
    solve(input, 2);
}

fn solve_b(input: &str) {
    solve(input, 20);
}
