use std::collections::HashMap;
use grid::{grid, Grid};

fn main() {
    let example = include_str!("../../inputs/day8/example.txt");
    let main_input = include_str!("../../inputs/day8/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

#[allow(clippy::type_complexity)]
fn parse_map(input: &str) -> (Grid<char>, HashMap<char, Vec<(i32, i32)>>){
    let mut map = grid![];
    let mut antennas = HashMap::new();

    for (row, line) in input.lines().enumerate(){
        let mut chars = Vec::with_capacity(line.len());
        for (col, ch) in line.chars().enumerate(){
            chars.push(ch);

            if ch != '.' {
                antennas.entry(ch)
                    .and_modify(|r: &mut Vec<(i32, i32)>| r.push((row as i32,col as i32)))
                    .or_insert(vec![(row as i32, col as i32)]);
            }
        }

        map.push_row(chars);
    }

    (map, antennas)
}

fn solve_a(input: &str) {
    let (mut map, antennas) = parse_map(input);

    for positions in antennas.values(){
        for (i, first) in positions.iter().enumerate(){
            for second in positions[i+1..].iter(){
                let (row_diff, col_diff) = (first.0 - second.0, first.1 - second.1);

                let (first_antinode, second_antinode) = ((first.0 + row_diff, first.1 + col_diff), (second.0 + -row_diff, second.1 + -col_diff));

                if let Some(antinode) = map.get_mut(first_antinode.0, first_antinode.1){
                    *antinode = '#'
                }

                if let Some(antinode) = map.get_mut(second_antinode.0, second_antinode.1){
                    *antinode = '#'
                }
            }
        }
    }

    let antinode_count = map.iter().filter(|ch| **ch == '#').count();

    println!("part 1: {}", antinode_count)
}

fn solve_b(input: &str) {
    let (mut map, antennas) = parse_map(input);

    for positions in antennas.values(){
        for (i, first) in positions.iter().enumerate(){
            for second in positions[i+1..].iter() {
                let (row_diff, col_diff) = (first.0 - second.0, first.1 - second.1);

                let mut new_pos = (first.0, first.1);

                while let Some(char) = map.get_mut(new_pos.0, new_pos.1) {
                    *char = '#';
                    new_pos = (new_pos.0 + row_diff, new_pos.1 + col_diff);
                }

                let mut new_pos = (second.0, second.1);

                while let Some(char) = map.get_mut(new_pos.0, new_pos.1) {
                    *char = '#';
                    new_pos = (new_pos.0 - row_diff, new_pos.1 - col_diff);
                }
            }
        }
    }

    let antinode_count = map.iter().filter(|ch| **ch == '#').count();

    println!("part 2 {}", antinode_count)
}