use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    solve_a();
    solve_b();
}

fn get_map() -> Vec<Vec<char>>{
    let input = include_str!("input");
    let mut map = input.lines().map(|line| {
        let mut chars = vec!['.'];
        chars.append(&mut line.chars().collect::<Vec<_>>());
        chars.push('.');

        chars
    }).collect::<Vec<_>>();

    map.insert(0, vec!['.'; map[0].len()]);
    map.push(vec!['.'; map[0].len()]);

    map
}
fn solve_a(){
    let map = get_map();

    let mut res = 0;
    for i in 1..map.len(){
        for j in 1..map[0].len(){
            if map[i][j-1].is_ascii_digit() || !map[i][j].is_ascii_digit(){
                continue;
            }

            let mut end = j;
            for k in j+1..{
                if map[i][k].is_ascii_digit(){
                    end += 1;
                }else{
                    break;
                }
            }

            if check_symbol(i, j, end, &map){
                let num = i32::from_str(map[i][j..=end].iter().collect::<String>().as_str()).unwrap();
                res += num;
            }
        }
    }

    println!("{}", res);
}

fn solve_b(){
    let map = get_map();

    let mut gear_map: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for i in 1..map.len(){
        for j in 1..map[0].len(){
            if map[i][j-1].is_ascii_digit() || !map[i][j].is_ascii_digit(){
                continue;
            }

            let mut end = j;
            for k in j+1..{
                if map[i][k].is_ascii_digit(){
                    end += 1;
                }else{
                    break;
                }
            }

            let (valid, x, y) = check_symbol_b(i, j, end, &map);
            if valid{
                let num = i32::from_str(map[i][j..=end].iter().collect::<String>().as_str()).unwrap();
                gear_map.entry((x, y)).and_modify(|e| e.push(num)).or_insert(vec![num]);
            }
        }
    }

    let res = gear_map.values().fold(0, |acc, x|{
        if x.len() == 2{
            return acc + x[0] * x[1];
        }

        acc
    });

    println!("{}", res);
}


fn check_symbol(x: usize, y1: usize, y2: usize, map :&[Vec<char>]) -> bool{
    for i in x-1..=x+1 {
        for j in y1-1..=y2+1 {
            match map[i][j] {
                '@' | '#' | '$' | '%' | '^' | '&' | '*' | '/' | '-' | '+' | '=' => {
                    return true;
                }
                _ => {}
            }
        }
    }

    false
}

fn check_symbol_b(x: usize, y1: usize, y2: usize, map :&[Vec<char>]) -> (bool, usize,usize){
    for i in x-1..=x+1 {
        for j in y1-1..=y2+1 {
            if map[i][j] == '*'{
                return (true, i,j);
            }
        }
    }

    (false, 0,0)
}