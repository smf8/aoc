use itertools::Itertools;
use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    solve_a();
    solve_b();
}

fn solve_a() {
    let mut hands = read_input();
    hands.sort_by(sort_hand(false));

    let sum = hands
        .iter()
        .enumerate()
        .fold(0, |a, x| a + (hands.len() - x.0) * x.1 .1 as usize);

    println!("{:?}", sum);
}
fn solve_b() {
    let mut hands = read_input();
    hands.sort_by(sort_hand(true));

    let sum = hands
        .iter()
        .enumerate()
        .fold(0, |a, x| a + (hands.len() - x.0) * x.1 .1 as usize);

    println!("{:?}", sum);
}

fn sort_hand(part_b: bool) -> impl Fn(&Hand, &Hand) -> Ordering {
    let closure = move |a: &Hand, b: &Hand| {
        let cmp = a.rank(part_b).cmp(&b.rank(part_b));
        if cmp == Ordering::Equal {
            // might not be the best solution :D
            for i in 0..a.0.len() {
                if a.0[i] != b.0[i] {
                    // change it to LABELS_B/A
                    let labels = if part_b { LABELS_B } else { LABELS_A };

                    for l in labels {
                        match (l == a.0[i], l == b.0[i]) {
                            (true, false) => return Ordering::Less,
                            (false, true) => return Ordering::Greater,
                            _ => continue,
                        }
                    }
                }
            }
        }

        cmp
    };

    closure
}

#[derive(Debug)]
struct Hand(Vec<char>, u32);

static LABELS_A: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

static LABELS_B: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

impl Hand {
    fn rank(&self, part_b: bool) -> u32 {
        let mut counts = self.0.iter().counts();

        if part_b {
            let &max = counts
                .iter()
                .max_by(|a, b| {
                    if **a.0 == 'J' {
                        return Ordering::Less;
                    }

                    if **b.0 == 'J' {
                        return Ordering::Greater;
                    }

                    a.1.cmp(b.1)
                })
                .unwrap()
                .0;

            let j_count = counts.remove(&'J').unwrap_or(0);

            counts
                .entry(max)
                .and_modify(|a| *a += j_count)
                .or_insert(j_count);
        }

        for count in counts.values() {
            return match (counts.len(), count) {
                (5, _) => 7,     // high card
                (4, _) => 6,     // one pair
                (1, _) => 1,     // five of a kind
                (2, 1 | 4) => 2, // four of a kind
                (2, 2 | 3) => 3, //full house
                (3, 3) => 4,     // three of a kind
                (3, 2) => 5,     // two pair
                _ => continue,
            };
        }

        0
    }
}

impl FromStr for Hand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.split_whitespace().collect::<Vec<_>>();
        let hand = line[0].chars().collect::<Vec<_>>();
        let bid = line[1].parse::<u32>()?;

        Ok(Hand(hand, bid))
    }
}

fn read_input() -> Vec<Hand> {
    let input = include_str!("input");

    let hands = input
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect::<Vec<_>>();

    hands
}
