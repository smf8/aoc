use grid::Grid;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{algo, Graph, Undirected};
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    // let example = include_str!("../../inputs/day18/example.txt");
    let main_input = include_str!("../../inputs/day18/main.txt");

    // solve(example);
    solve_a(main_input);
    solve_b(main_input);
}

#[derive(Clone)]
struct Map {
    bytes_seq: Vec<(i32, i32)>,
    graph: Graph<(i32, i32), (), Undirected>,
    graph_indices: HashMap<(i32, i32), NodeIndex>,
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

fn drop_byte(graph: &mut Graph<(i32, i32), (), Undirected>, index: NodeIndex) {
    let edges: Vec<_> = graph.edges(index).map(|e| e.id()).collect();
    for edge in edges {
        graph.remove_edge(edge);
    }
}

fn parse_map(input: &str, dimension: usize) -> Map {
    let map: Grid<char> = Grid::new(dimension, dimension);

    let mut graph: Graph<(i32, i32), (), Undirected> = Graph::new_undirected();
    let mut indices: HashMap<(i32, i32), NodeIndex> = HashMap::new();

    let bytes_sequence = input
        .lines()
        .map(|line| {
            let pair = line
                .split(',')
                .map(|ch| ch.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (pair[1], pair[0])
        })
        .collect::<Vec<_>>();

    for row in 0..dimension {
        for col in 0..dimension {
            let index = graph.add_node((row as i32, col as i32));
            indices.insert((row as i32, col as i32), index);
        }
    }

    for row in 0..dimension {
        for col in 0..dimension {
            for &dir in Dir::all() {
                let delta = dir.delta();
                let new_pos = (row as i32 + delta.0, col as i32 + delta.1);

                if map.get(new_pos.0, new_pos.1).is_some() {
                    graph.update_edge(indices[&(row as i32, col as i32)], indices[&new_pos], ());
                }
            }
        }
    }

    Map {
        bytes_seq: bytes_sequence,
        graph,
        graph_indices: indices,
    }
}

fn solve_a(input: &str) {
    // example
    let dimension = 71;
    // let map = parse_map(input, 7);
    let mut map = parse_map(input, dimension);

    let start = map.graph_indices[&(0, 0)];
    let end = map.graph_indices[&(dimension as i32 - 1, dimension as i32 - 1)];

    // part1
    for b in map.bytes_seq[..1024].iter() {
        drop_byte(&mut map.graph, map.graph_indices[b]);
    }

    let distances = algo::dijkstra(&map.graph, start, Some(end), |_| 1);

    println!("part 1: {}", distances[&end]);
}

fn solve_b(input: &str) {
    // example
    let dimension = 71;
    // let map = parse_map(input, 7);
    let mut map = parse_map(input, dimension);

    let start = map.graph_indices[&(0, 0)];
    let end = map.graph_indices[&(dimension as i32 - 1, dimension as i32 - 1)];

    for b in map.bytes_seq.iter() {
        drop_byte(&mut map.graph, map.graph_indices[b]);

        let is_connected = algo::has_path_connecting(&map.graph, start, end, None);
        if !is_connected {
            println!("part 2: {},{}", b.1, b.0);
            break;
        }
    }
}
