use grid::grid;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Dfs, IntoNodeIdentifiers, NodeFiltered};
use petgraph::{algo, Graph};

fn main() {
    let example = include_str!("../../inputs/day10/example.txt");
    let main_input = include_str!("../../inputs/day10/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

#[derive(Clone)]
struct HikeNode {
    height: i32,
}

fn parse_map(input: &str) -> (Graph<HikeNode, u32>, Vec<NodeIndex>) {
    let mut node_indices = grid![];

    let mut graph: Graph<HikeNode, u32> = Graph::new();
    let mut starting_points = Vec::new();

    for line in input.lines() {
        let mut row_vec = Vec::new();

        for num_str in line.chars().map(|c| c.to_digit(10).unwrap_or(99)) {
            let n = HikeNode {
                height: num_str as i32,
            };

            let node_index = graph.add_node(n.clone());
            row_vec.push(node_index);

            if num_str == 0 {
                starting_points.push(node_index);
            }
        }

        node_indices.push_row(row_vec);
    }

    let directions = [
        (-1, 0), //up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    for row in 0..node_indices.rows() {
        for col in 0..node_indices.cols() {
            let node = graph[node_indices[(row, col)]].clone();
            for dir in directions {
                if let Some(neighbor_index) =
                    node_indices.get(row as i32 + dir.0, col as i32 + dir.1)
                {
                    let neighbor = graph[*neighbor_index].clone();
                    if neighbor.height - node.height == 1 {
                        graph.add_edge(
                            node_indices[(row, col)],
                            *node_indices
                                .get(row as i32 + dir.0, col as i32 + dir.1)
                                .unwrap(),
                            1,
                        );
                    }
                }
            }
        }
    }

    (graph, starting_points)
}

fn solve_a(input: &str) {
    let (graph, starting_points) = parse_map(input);

    let mut result = 0;
    for point in starting_points {
        let mut dfs = Dfs::new(&graph, point);
        while let Some(nx) = dfs.next(&graph) {
            let node = &graph[nx];
            if node.height == 9 {
                result += 1;
            }
        }
    }

    println!("part 1:{}", result)
}

fn solve_b(input: &str) {
    let (graph, starting_points) = parse_map(input);

    let ends = NodeFiltered::from_fn(&graph, |f| graph[f].height == 9)
        .node_identifiers()
        .collect::<Vec<_>>();
    let mut result = 0;
    for start_point in starting_points {
        for end_point in ends.iter() {
            let ways =
                algo::all_simple_paths::<Vec<_>, _>(&graph, start_point, *end_point, 8, None)
                    .count();
            result += ways;
        }
    }

    println!("part 2:{}", result)
}
