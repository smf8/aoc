use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::{algo, Graph};
use std::collections::HashMap;

fn main() {
    let example = include_str!("../../inputs/day21/example.txt");
    let main_input = include_str!("../../inputs/day21/main.txt");

    solve(example, 2);
    solve(main_input, 2);
    // // //
    solve(example, 25);
    solve(main_input, 25);
}

fn number_all_paths(number: String) -> Vec<String> {
    let mut numpad_graph: Graph<(i32, i32), char> = Graph::new();

    let key0 = numpad_graph.add_node((3, 1));
    let key_a = numpad_graph.add_node((3, 2));
    let key1 = numpad_graph.add_node((2, 0));
    let key2 = numpad_graph.add_node((2, 1));
    let key3 = numpad_graph.add_node((2, 2));
    let key4 = numpad_graph.add_node((1, 0));
    let key5 = numpad_graph.add_node((1, 1));
    let key6 = numpad_graph.add_node((1, 2));
    let key7 = numpad_graph.add_node((0, 0));
    let key8 = numpad_graph.add_node((0, 1));
    let key9 = numpad_graph.add_node((0, 2));

    let num_index_map = HashMap::from([
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
        ('A', key_a),
    ]);

    numpad_graph.extend_with_edges([
        (key0, key_a, '>'),
        (key0, key2, '^'),
        (key_a, key0, '<'),
        (key_a, key3, '^'),
        (key1, key2, '>'),
        (key1, key4, '^'),
        (key2, key3, '>'),
        (key2, key5, '^'),
        (key2, key0, 'v'),
        (key2, key1, '<'),
        (key5, key6, '>'),
        (key5, key8, '^'),
        (key5, key2, 'v'),
        (key5, key4, '<'),
        (key3, key2, '<'),
        (key3, key6, '^'),
        (key3, key_a, 'v'),
        (key4, key5, '>'),
        (key4, key7, '^'),
        (key4, key1, 'v'),
        (key6, key5, '<'),
        (key6, key9, '^'),
        (key6, key3, 'v'),
        (key7, key8, '>'),
        (key7, key4, 'v'),
        (key8, key7, '<'),
        (key8, key9, '>'),
        (key8, key5, 'v'),
        (key9, key8, '<'),
        (key9, key6, 'v'),
    ]);

    let mut current_node = key_a;

    let number_paths = find_all_shortest_paths(&numpad_graph);

    let mut paths: Vec<String> = vec!["".to_string()];

    let passcode = number
        .chars()
        .map(|c| num_index_map[&c])
        .collect::<Vec<_>>();

    for next_number in passcode.iter() {
        let mut temp_paths = Vec::new();

        let new_paths = number_paths
            .get(&(current_node, *next_number))
            .unwrap()
            .iter()
            .map(|v| path_string(&numpad_graph, v))
            .collect::<Vec<_>>();
        for path in paths.iter_mut() {
            for new_path in new_paths.iter() {
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

    for (pair, &shortest_distance) in distances.iter() {
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

    while let Some((current, path)) = stack.pop() {
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
                    if shortest_distance
                        == *distances.get(&(start, current)).unwrap() + neighbor_distance + 1
                    {
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

fn path_string(graph: &Graph<(i32, i32), char>, path: &Vec<EdgeIndex>) -> String {
    let mut s = String::new();

    for p in path {
        let ch = graph.edge_weight(*p).unwrap();
        s.push(*ch);
    }

    s
}

fn generate_movements() -> Vec<HashMap<&'static str, Vec<String>>> {
    let possibilites = HashMap::from([
        ("<A", vec!["A>>^A", "A>^>A"]),
        ("A<", vec!["Av<<A", "A<v<A"]),
        (">^", vec!["A^<A", "A<^A"]),
        ("^>", vec!["A>vA", "Av>A"]),
        ("vA", vec!["A>^A", "A^>A"]),
        ("Av", vec!["Av<A", "A<vA"]),
    ]);

    let pair_movements = HashMap::from([
        ("<<", vec!["AA"]),
        ("<v", vec!["A>", ">A"]),
        ("<^", vec!["A>", ">^", "^A"]),
        ("<A", vec!["A>", ">>", ">^", "^A"]),
        ("A<", vec!["Av", "v<", "<<", "<A"]),
        (">>", vec!["AA"]),
        (">v", vec!["A<", "<A"]),
        (">^", vec!["A<", "<^", "^A"]),
        (">A", vec!["A^", "^A"]),
        ("A>", vec!["Av", "vA"]),
        ("vv", vec!["AA"]),
        ("v>", vec!["A>", ">A"]),
        ("v<", vec!["A<", "<A"]),
        ("vA", vec!["A>", ">^", "^A"]),
        ("Av", vec!["A<", "<v", "vA"]),
        ("^^", vec!["AA"]),
        ("^>", vec!["Av", "v>", ">A"]),
        ("^<", vec!["Av", "v<", "<A"]),
        ("^A", vec!["A>", ">A"]),
        ("A^", vec!["A<", "<A"]),
        ("AA", vec!["AA"]),
    ]);

    let pair_movements = pair_movements
        .iter()
        .map(|(k, v)| (*k, v.iter().map(|s| s.to_string()).collect::<Vec<_>>()))
        .collect::<HashMap<_, _>>();

    let mut all_movements = vec![pair_movements];

    for (pair, possible_perm) in possibilites {
        let mut tmp_vec = Vec::new();

        for permutation in possible_perm {
            let replacement_vec = permutation
                .chars()
                .collect::<Vec<_>>()
                .windows(2)
                .map(String::from_iter)
                .collect::<Vec<_>>();

            let mut new_movements = all_movements.clone();
            for movement in new_movements.iter_mut() {
                movement.insert(pair, replacement_vec.clone());
            }

            tmp_vec.extend(new_movements);
        }

        all_movements = tmp_vec;
    }

    all_movements
}

fn movements_count(
    steps: usize,
    number: &str,
    movement_map: &HashMap<&'static str, Vec<String>>,
) -> usize {
    let mut case = number.chars().collect::<Vec<_>>();
    case.insert(0, 'A');

    let mut result = HashMap::new();
    for pair in case.windows(2) {
        let pair_str = pair.iter().collect::<String>();
        *result.entry(pair_str).or_insert(0_usize) += 1;
    }

    for _ in 0..steps {
        let mut new_result = HashMap::new();

        for (pair, count) in result.iter() {
            let new_pairs = movement_map
                .get(pair.as_str())
                .unwrap_or_else(|| panic!("{}", pair))
                .clone();
            for new_pair in new_pairs {
                *new_result.entry(new_pair.to_string()).or_insert(0) += count;
            }
        }

        result = new_result;
    }

    result.values().sum::<usize>()
}

fn solve(input: &str, steps: usize) {
    let mut result = 0;
    for line in input.lines() {
        let number_dirs = number_all_paths(line.to_string());
        let mut min_number_res = usize::MAX;
        for movement_map in generate_movements() {
            for number_str in number_dirs.iter() {
                min_number_res =
                    movements_count(steps, number_str, &movement_map).min(min_number_res);
            }
        }

        let number = line.strip_suffix("A").unwrap().parse::<usize>().unwrap();
        result += min_number_res * number;
    }

    println!("{}-steps: {}", steps, result);
}
