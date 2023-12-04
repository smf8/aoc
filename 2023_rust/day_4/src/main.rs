use std::collections::{HashMap, HashSet};

fn main() {
    solve_a();
    solve_b();
}

fn solve_a() {
    let input = include_str!("input");

    let mut sum = 0;
    for line in input.lines() {
        let (_, matching_cards) = card_score(line);

        if matching_cards != 0 {
            let card_score = 2u32.pow((matching_cards - 1) as u32);
            sum += card_score;
        }
    }

    println!("{}", sum);
}

fn solve_b() {
    let input = include_str!("input");

    let mut card_copies = HashMap::<u32, u32>::new();
    for line in input.lines() {
        let (card_id, matching_cards) = card_score(line);

        // count the original card
        card_copies
            .entry(card_id as u32)
            .and_modify(|a| *a += 1u32)
            .or_insert(1);

        // handle previously acquired copies
        for _ in 0..*card_copies.get(&(card_id as u32)).unwrap_or(&1) {
            for i in card_id + 1..=card_id + matching_cards {
                card_copies
                    .entry(i as u32)
                    .and_modify(|a| *a += 1u32)
                    .or_insert(1);
            }
        }
    }

    let sum = card_copies.values().sum::<u32>();
    println!("{}", sum);
}

fn card_score(card_line: &str) -> (usize, usize) {
    let card_content = card_line.split([':', '|']).collect::<Vec<_>>();
    let card_id = card_content[0]
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let winning_cards = card_content[1]
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    let matching_cards = card_content[2]
        .split_whitespace()
        .filter(|n| winning_cards.contains(&n.parse::<u32>().unwrap()))
        .count();

    (card_id, matching_cards)
}
