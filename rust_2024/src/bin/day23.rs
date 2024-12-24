use petgraph::{Graph, Undirected};
use std::collections::HashMap;

fn main() {
    let example = include_str!("../../inputs/day23/example.txt");
    let main_input = include_str!("../../inputs/day23/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn parse_input(input: &str) -> Graph<String, (), Undirected> {
    let mut graph = Graph::new_undirected();
    let mut indices = HashMap::new();

    for line in input.lines() {
        let nodes = line.split('-').collect::<Vec<_>>();
        for node in nodes {
            if !indices.contains_key(node) {
                indices.insert(node, graph.add_node(node.to_string()));
            }
        }
    }

    for line in input.lines() {
        let nodes = line.split('-').map(|n| indices[n]).collect::<Vec<_>>();
        graph.add_edge(nodes[0], nodes[1], ());
    }

    graph
}

fn solve_a(input: &str) {
    let graph = parse_input(input);

    let cycles = find_cycles_of_length_3(&graph);

    println!("part 1: {}", cycles.len());
}

// I'm not proud of this solution myself.
fn find_cycles_of_length_3(graph: &Graph<String, (), Undirected>) -> Vec<Vec<String>> {
    let mut cycles = Vec::new();
    let nodes: Vec<_> = graph.node_indices().collect();

    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            for k in j + 1..nodes.len() {
                if graph.contains_edge(nodes[i], nodes[j])
                    && graph.contains_edge(nodes[j], nodes[k])
                    && graph.contains_edge(nodes[k], nodes[i])
                {
                    let cycle = vec![
                        graph[nodes[i]].clone(),
                        graph[nodes[j]].clone(),
                        graph[nodes[k]].clone(),
                    ];
                    let mut starts_with_t = false;
                    for node in cycle.iter() {
                        if node.starts_with("t") {
                            starts_with_t = true;
                        }
                    }

                    if starts_with_t {
                        cycles.push(cycle);
                    }
                }
            }
        }
    }

    cycles
}

fn solve_b(input: &str) {
    let graph = parse_input(input);
    let mut largest_complete_graph = Vec::new();
    let nodes = graph.node_indices().collect::<Vec<_>>();

    for first_node_index in 0..nodes.len() {
        let mut complete_graph = vec![nodes[first_node_index]];

        for node in nodes.iter() {
            let mut is_node_connected = true;
            for new_node in complete_graph.iter() {
                if !graph.contains_edge(*new_node, *node) {
                    is_node_connected = false;
                }
            }

            if is_node_connected {
                complete_graph.push(*node);
            }
        }

        if complete_graph.len() > largest_complete_graph.len() {
            largest_complete_graph = complete_graph;
        }
    }

    let mut result = largest_complete_graph
        .iter()
        .map(|idx| graph.node_weight(*idx).unwrap().clone())
        .collect::<Vec<_>>();

    result.sort();

    println!("part 2: {}", result.join(","));
}
