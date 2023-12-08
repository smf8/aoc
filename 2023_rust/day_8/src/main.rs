use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    solve_a();
    solve_b();
}

fn solve_a(){
    let (commands, map) = read_input();

    let mut current_node = "AAA";
    let mut cmd_index = 0;
    let mut steps = 0;
    loop {
        match (current_node, commands[cmd_index]) {
            ("ZZZ", _) => {println!("{}", steps); break;},
            (_, 'R') => current_node = map.get(current_node).unwrap().1.as_str(),
                (_, 'L') => current_node = map.get(current_node).unwrap().0.as_str(),
            _ => continue,
        }

        steps += 1;
        cmd_index = (cmd_index +1) % commands.len();
    }
}

fn solve_b(){
    let (commands, map) = read_input();

    let mut current_nodes = map.keys().filter(|k| k.ends_with('A')).map(|k| k.as_str()).collect::<Vec<_>>();
    let mut cmd_index = 0;
    let mut steps = 0;

    let mut result_map = HashMap::new();
    loop {
        current_nodes = current_nodes.iter().map(|node| {
            if commands[cmd_index] == 'R'{
                map.get(*node).unwrap().1.as_str()
            }else{
                map.get(*node).unwrap().0.as_str()
            }
        }).collect::<Vec<_>>();

        // put aside routes that already reached --Z
        current_nodes = current_nodes.iter().filter(|n| {
            if n.ends_with('Z'){
                result_map.insert(**n, steps + 1);
                return false;
            }

            true
        }).cloned().collect::<Vec<_>>();

        if current_nodes.is_empty(){
            steps = result_map.values().fold(1u64, |a, x| lcm(a, *x));
            println!("{}", steps);
            break;
        }

        steps += 1;
        cmd_index = (cmd_index +1) % commands.len();
    }
}

struct Node(String, String);

fn read_input() -> (Vec<char>, HashMap<String, Node>) {
    let input = include_str!("input").split("\n\n").collect::<Vec<_>>();
    let commands = input[0].chars().collect::<Vec<_>>();

    let mut map = HashMap::new();

    for line in input[1].lines() {
        let trimmed_line = line.replace(['=', ' ', ',', '(', ')'], "");
        let (key, commands) = trimmed_line.split_at(3);
        let (right, left) = commands.split_at(3);

        map.insert(key.to_string(), Node(right.to_string(), left.to_string()));
    }

    (commands, map)
}
