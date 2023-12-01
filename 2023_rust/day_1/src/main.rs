use std::collections::HashMap;
use fancy_regex::Regex;

fn main() {
    solve_a();
    solve_b();
}


fn solve_a(){
    let input = include_str!("input.txt");


    let is_char_predicate = |c: char| c.to_digit(10);

    let sum = input.lines().map(|line| {
        let chars = line.chars();
        let numbers = chars.filter_map(&is_char_predicate).collect::<Vec<u32>>();
        numbers.first().unwrap() * 10 + numbers.last().unwrap()
    }).sum::<u32>();

    println!("{}", sum);
}

fn solve_b(){
    let input = include_str!("input.txt");

    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|[1-9]))").unwrap();
    let numbers: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven",7),
        ("eight", 8),
        ("nine", 9),
    ].into_iter().collect();

    let sum = input.lines().map(|line| {
        let tokens = re.captures_iter(line).map(|a| {
            let number = a.unwrap().get(1).unwrap();
            if number.as_str().len() == 1{
                number.as_str().parse::<u32>().unwrap()
            }else{
                *numbers.get(number.as_str()).unwrap()
            }
        }).collect::<Vec<_>>();

        tokens.first().unwrap() * 10 + tokens.last().unwrap()
    }).sum::<u32>();

    println!("{}", sum);
}