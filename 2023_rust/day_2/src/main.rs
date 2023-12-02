use std::cmp::max;

fn main() {
    solve_a();
    solve_b();
}

fn solve_a(){
    let input = include_str!("input");

    let mut sum = 0;
    for line in input.lines(){
        let (game_id, game_picks) = parse_game(line);

        if !game_picks.iter().any(|a| a.0 > 12 || a.1 > 13 || a.2 > 14){
            sum += game_id;
        }
    }

    println!("{}", sum);
}

fn solve_b(){
    let input = include_str!("input");

    let mut sum = 0;
    for line in input.lines(){
        let (game_id, game_picks) = parse_game(line);

        let possible_rgb = game_picks.iter().fold(Rgb(0,0,0), |a, b| Rgb(max(a.0, b.0), max(a.1, b.1), max(a.2, b.2)));
        sum += possible_rgb.0 * possible_rgb.1 * possible_rgb.2;
    }

    println!("{}", sum);
}

struct Rgb(u32, u32, u32);

fn parse_game(line: &str)-> (u32, Vec<Rgb>){
    let line_split = line.split([':', ';']).collect::<Vec<_>>();
    let game_id: u32 = line_split[0].split_whitespace().last().unwrap().parse().unwrap();

    let res = line_split[1..].iter().map(|set| {
        let pick = set.split(',').fold(Rgb(0, 0, 0), |mut i, x| {
            let x_split = x.split_whitespace().collect::<Vec<_>>();
            let n = x_split.first().unwrap().parse::<u32>().unwrap();
            match *x_split.last().unwrap() {
                "red" =>{
                    i.0 = n;
                },
                "green" =>{
                    i.1 = n;
                },
                "blue" => {
                    i.2 = n;
                },
                _ => {}
            }

            i
        });

        pick
    }).collect::<Vec<_>>();

    (game_id, res)
}