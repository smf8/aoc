use grid::grid;
use petgraph::graph::NodeIndex;
use petgraph::{algo, Graph, Undirected};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    let example = include_str!("../../inputs/day16/example.txt");
    let main_input = include_str!("../../inputs/day16/main.txt");

    solve(example);
    solve(main_input);
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct MapNode {
    char: char,
    row: i32,
    col: i32,
    dir: Dir,
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

fn parse_map(input: &str) {
    let mut map = grid![];

    let mut graph: Graph<MapNode, i32, Undirected> = Graph::new_undirected();
    let mut indices: HashMap<(i32, i32, Dir), NodeIndex> = HashMap::new();

    let (mut start, mut end) = ((0, 0), (0, 0));

    for (row, line) in input.lines().enumerate() {
        let mut row_vec2 = Vec::new();

        for (col, ch) in line.chars().enumerate() {
            row_vec2.push(ch);
            if ch != '#' {
                let all_dirs = Dir::all();
                for dir in all_dirs {
                    let node = MapNode {
                        char: ch,
                        row: row as i32,
                        col: col as i32,
                        dir: *dir,
                    };

                    let index = graph.add_node(node);
                    indices.insert((row as i32, col as i32, *dir), index);
                }

                // add rotate edges
                for dir1 in all_dirs {
                    for dir2 in all_dirs {
                        if dir1 != dir2 {
                            let (delta1, delta2) = (dir1.delta(), dir2.delta());
                            if (delta1.0 + delta2.0, delta1.1 + delta2.1) != (0, 0) {
                                graph.update_edge(
                                    indices[&(row as i32, col as i32, *dir1)],
                                    indices[&(row as i32, col as i32, *dir2)],
                                    1000,
                                );
                            }
                        }
                    }
                }
            }
            if ch == 'S' {
                start = (row as i32, col as i32);
            } else if ch == 'E' {
                end = (row as i32, col as i32);
            }
        }

        map.push_row(row_vec2);
    }

    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if map[(row, col)] != '#' {
                for dir in Dir::all() {
                    let delta = dir.delta();
                    if let Some(next) = map.get(row as i32 + delta.0, col as i32 + delta.1) {
                        if *next != '#' {
                            graph.add_edge(
                                indices[&(row as i32, col as i32, *dir)],
                                indices[&(row as i32 + delta.0, col as i32 + delta.1, *dir)],
                                1,
                            );
                        }
                    }
                }
            }
        }
    }

    let start_dijkstra = algo::dijkstra(
        &graph,
        indices[&(start.0, start.1, Dir::Right)],
        None,
        |e| *e.weight(),
    );

    let mut min_path = i32::MAX;
    let mut result_dir = Dir::Right;
    for dir in Dir::all() {
        if min_path > start_dijkstra[&indices[&(end.0, end.1, *dir)]] {
            result_dir = *dir;
            min_path = start_dijkstra[&indices[&(end.0, end.1, *dir)]];
        }
    }

    let end_node_index = indices[&(end.0, end.1, result_dir)];

    let end_dijkstra = algo::dijkstra(&graph, end_node_index, None, |e| *e.weight());

    println!("part 1: {}", min_path);

    let mut res = HashSet::new();

    for index in indices.iter() {
        let start_to_point = start_dijkstra[index.1];
        let end_to_point = end_dijkstra[index.1];
        let (row, col) = (index.0 .0, index.0 .1);
        if start_to_point < min_path && start_to_point + end_to_point == min_path {
            res.insert((row, col));
        }
    }

    println!("part 2: {:?}", res.len() + 1);
}

fn solve(input: &str) {
    parse_map(input);
}
