fn main() {
    let example = include_str!("../../inputs/day2/example.txt");
    let main_input = include_str!("../../inputs/day2/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

fn solve_a(input: &str) {
    let mut lists = input.lines().map(|line| line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    lists = lists.into_iter().filter(|list| {
        is_safe(list)
    }).collect();

    println!("part1: {}", lists.len())
}

fn solve_b(input: &str) {
    let mut lists = input.lines().map(|line| line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let result = lists.into_iter().filter_map(|mut list| {
        let is_ascending = list.windows(2).map(|window| if window[1] > window[0] {
            1
        } else{
            -1
        }).sum::<i32>() > 0;


        let invalid_pos = list.windows(2).position(|window|
            (is_ascending && window[1] - window[0] <= 0 || window[1] - window[0] > 3)
                || (!is_ascending && 0 >= window[0] - window[1] || window[0] - window[1] > 3));

        // when we have an invalid window, either values in window can be the cause
        // so we have to test correctness for both numbers in the window
        if let Some(invalid_pos) = invalid_pos {
            // first element is faulty
            let removed_element = list.remove(invalid_pos);

            if is_safe(&list){
                Some(true)
            }else{
                // second element might be faulty
                list.insert(invalid_pos, removed_element);
                list.remove(invalid_pos + 1);
                if is_safe(&list) {
                    Some(true)
                }else{
                    None
                }
            }
        }else{
            Some(true)
        }
    }).collect::<Vec<_>>();

    println!("part2: {}", result.len())
}

fn is_safe(list: &[i32]) -> bool{
    let is_ascending = list[0] < list[1];

    list.windows(2).all(|window|
        (is_ascending && 0 < window[1] - window[0] && window[1] - window[0] <= 3)
            || (!is_ascending && 0 < window[0] - window[1] && window[0] - window[1] <= 3))
}