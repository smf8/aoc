use petgraph::graph::{EdgeIndex, Node, NodeIndex};
use petgraph::{algo, Graph, Undirected};
use std::collections::HashMap;
use std::hash::Hash;
use petgraph::visit::NodeRef;

fn main() {
    let example = include_str!("../../inputs/day21/example.txt");
    let main_input = include_str!("../../inputs/day21/main.txt");

    solve_a(example);
    solve_a(main_input);
    // // //
    // solve_b(example);
    // solve_b(main_input);
}

fn parse_map(number: &str) -> usize {
    let number_dirs = number_all_paths(number.to_string());

    let mut all_paths = number_dirs;
    for _ in 0..2 {
        let mut tmp_paths = Vec::new();
        for path in all_paths {
            let new_paths = dir_all_paths(path);
            tmp_paths.extend(new_paths);
        }

        // for path in tmp_paths.iter(){
        //     println!("{}", path);
        // }

        all_paths = tmp_paths;
    }

    let min_path = all_paths.iter().map(|m| m.len()).min().unwrap();

    // for path in all_paths{
    //     println!("{}", path);
    // }

    min_path
}

fn dir_all_paths(direction: String) -> Vec<String>{

    let mut keypad_graph: Graph<(i32, i32), char> = Graph::new();

    let keyUp = keypad_graph.add_node((0, 1));
    let keyADir = keypad_graph.add_node((0, 2));
    let keyDown = keypad_graph.add_node((1, 1));
    let keyRight = keypad_graph.add_node((1, 2));
    let keyLeft = keypad_graph.add_node((1, 0));

    keypad_graph.extend_with_edges([
        (keyUp, keyADir, '>'), (keyUp, keyDown, 'v'),
        (keyADir, keyUp, '<'), (keyADir, keyRight, 'v'),
        (keyLeft, keyDown, '>'),
        (keyRight, keyDown, '<'), (keyRight, keyADir, '^'),
        (keyDown, keyUp, '^'), (keyDown, keyRight, '>'), (keyDown, keyLeft, '<'),
    ]);

    let key_index_map = HashMap::from(
        [
            ('<', keyLeft),
            ('^', keyUp),
            ('>', keyRight),
            ('v', keyDown),
            ('A', keyADir)
        ]
    );

    let dir_paths = find_all_shortest_paths(&keypad_graph);

    let mut paths: Vec<String> = vec!["".to_string()];

    let mut current_node = keyADir;

    for ch in direction.chars(){
        let mut temp_paths = Vec::new();

        let new_paths = dir_paths.get(&(current_node, key_index_map[&ch])).unwrap_or_else(|| panic!("{:?} {:?}", current_node, key_index_map[&ch])).iter().map(|v| path_string(&keypad_graph, v)).collect::<Vec<_>>();

        for path in paths.iter(){
            for new_path in new_paths.iter(){
                let mut p = path.clone();
                p.push_str(new_path);
                p.push('A');
                temp_paths.push(p);
            }
        }

        paths = temp_paths;
        current_node = key_index_map[&ch];
    }

    paths
}

fn number_all_paths(number: String) -> Vec<String>{
    let mut numpad_graph: Graph<(i32, i32), char> = Graph::new();

    let key0 = numpad_graph.add_node((3,1));
    let keyA = numpad_graph.add_node((3,2));
    let key1 = numpad_graph.add_node((2,0));
    let key2 = numpad_graph.add_node((2,1));
    let key3 = numpad_graph.add_node((2,2));
    let key4 = numpad_graph.add_node((1,0));
    let key5 = numpad_graph.add_node((1,1));
    let key6 = numpad_graph.add_node((1,2));
    let key7 = numpad_graph.add_node((0,0));
    let key8 = numpad_graph.add_node((0,1));
    let key9 = numpad_graph.add_node((0,2));

    let num_index_map = HashMap::from(
        [
            ('0', key0),
('1', key1),
('2', key2),
('3', key3),
('4', key4),
('5', key5),
('6', key6),
('7', key7),
('8', key8),
('9', key9),
('A', keyA),
        ]
    );

    numpad_graph.extend_with_edges([
        (key0, keyA, '>'), (key0, key2, '^'),
        (keyA, key0, '<'), (keyA, key3, '^'),
        (key1, key2, '>'), (key1, key4, '^'),
        (key2, key3, '>'), (key2, key5, '^'), (key2, key0, 'v'), (key2, key1, '<'),
        (key5, key6, '>'), (key5, key8, '^'), (key5, key2, 'v'), (key5, key4, '<'),
        (key3, key2, '<'), (key3, key6, '^'), (key3, keyA, 'v'),
        (key4, key5, '>'), (key4, key7, '^'), (key4, key1, 'v'),
        (key6, key5, '<'), (key6, key9, '^'), (key6, key3, 'v'),
        (key7, key8, '>'), (key7, key4, 'v'),
        (key8, key7, '<'), (key8, key9, '>'), (key8, key5, 'v'),
        (key9, key8, '<'), (key9, key6, 'v'),
    ]);

    let mut current_node = keyA;

    let number_paths = find_all_shortest_paths(&numpad_graph);

    let mut paths: Vec<String> = vec!["".to_string()];

    let passcode = number.chars().map(|c| num_index_map[&c]).collect::<Vec<_>>();

    for next_number in passcode.iter(){
        let mut temp_paths = Vec::new();

        let new_paths = number_paths.get(&(current_node, *next_number)).unwrap().iter().map(|v| path_string(&numpad_graph, v)).collect::<Vec<_>>();
        for path in paths.iter_mut(){
            for new_path in new_paths.iter(){
                let mut p = path.clone();
                p.push_str(new_path);
                p.push('A');
                temp_paths.push(p);
            }
        }

        paths = temp_paths;
        current_node = *next_number;
    }

    paths
}

fn find_all_shortest_paths(
    graph: &Graph<(i32, i32), char>,
) -> HashMap<(NodeIndex, NodeIndex), Vec<Vec<EdgeIndex>>> {
    let distances = algo::floyd_warshall(&graph, |_| 1).unwrap();

    let mut paths: HashMap<(NodeIndex, NodeIndex), Vec<_>> = HashMap::new();

    for (pair, distance) in distances.iter(){
        let shortest_distance = *distances.get(pair).unwrap();
        let mut all_paths = Vec::new();
        // println!("{:?} -> {:?}",  graph[pair.0], graph[pair.1]);
        dfs_iterative(
            graph,
            pair.0,
            pair.1,
            shortest_distance,
            &distances,
            &mut all_paths,
        );

        // for p in all_paths.iter(){
        //     println!("{}", path_string(graph, p))
        // }

        paths.insert(*pair, all_paths);
    }




    paths
}

fn dfs_iterative(
    graph: &Graph<(i32, i32), char>,
    start: NodeIndex,
    end: NodeIndex,
    shortest_distance: usize,
    distances: &HashMap<(NodeIndex, NodeIndex), usize>,
    all_paths: &mut Vec<Vec<EdgeIndex>>,
) {
    let mut stack = vec![(start, vec![])];
    let mut visited = HashMap::new();

    while let Some((current, mut path)) = stack.pop() {
        if current == end {
            if path.len() == shortest_distance {
                all_paths.push(path.clone());
            }
            continue;
        }

        visited.insert(current, true);

        for neighbor in graph.neighbors(current) {
            if !visited.get(&neighbor).unwrap_or(&false) {
                if let Some(&neighbor_distance) = distances.get(&(neighbor, end)) {
                    if shortest_distance == *distances.get(&(start, current)).unwrap() + neighbor_distance + 1 {
                        let mut new_path = path.clone();
                        new_path.push(graph.find_edge(current, neighbor).unwrap());
                        stack.push((neighbor, new_path));
                    }
                }
            }
        }

        visited.insert(current, false);
    }
}

fn path_string(graph: &Graph<(i32, i32), char>, path: &Vec<EdgeIndex>) -> String{
    let mut s = String::new();

    for p in path{
        let ch = graph.edge_weight(*p).unwrap();
        s.push(*ch);
    }

    s
}

fn solve_a(input: &str) {
    let mut result = 0;
    for line in input.lines(){
        let line_res =parse_map(line);
        let number = line.strip_suffix("A").unwrap().parse::<usize>().unwrap();
        result += line_res * number;
    }

    println!("part 1: {}", result)
}

fn solve_b(input: &str) {
}
