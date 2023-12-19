fn main() {
    solve_a();
    solve_b();
}

fn solve_b() {
    let patterns = read_input();

    let mut sum = 0;
    for pattern in patterns {
        if let Some(horizontal_mirror) = duplicate_vec(&pattern.rows, true) {
            sum += horizontal_mirror * 100;
            continue;
        }

        if let Some(vertical_mirror) = duplicate_vec(&pattern.columns, true) {
            sum += vertical_mirror;
            continue;
        }
    }

    println!("{}", sum);
}

fn solve_a() {
    let patterns = read_input();

    let mut sum = 0;
    for pattern in patterns {
        if let Some(horizontal_mirror) = duplicate_vec(&pattern.rows, false) {
            sum += horizontal_mirror * 100;
            continue;
        }

        if let Some(vertical_mirror) = duplicate_vec(&pattern.columns, false) {
            sum += vertical_mirror;
            continue;
        }
    }

    println!("{}", sum);
}

struct Pattern {
    rows: Vec<String>,
    columns: Vec<String>,
}

fn diff(str_one: &str, str_two: &str) -> usize {
    str_one
        .chars()
        .zip(str_two.chars())
        .filter(|comparison| comparison.0 != comparison.1)
        .count()
}

fn dup(v: &[String], part_b: bool) -> Option<usize> {
    let max_diff = if part_b { 1 } else { 0 };

    let first_duplicates = v
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, r)| {
            let diff = diff(r.as_str(), v.first().unwrap().as_str());

            diff <= max_diff
        })
        .collect::<Vec<_>>();

    // return none for no duplicate
    if first_duplicates.is_empty() {
        return None;
    }

    // we skip the first record (v[0]) and only consume even duplications
    for (duplicate_index, _) in first_duplicates.iter().step_by(2) {
        // zip the slice from center
        // -> ##.##.#. will become => (##.# , .#.#)
        let differences = v
            .iter()
            .take(duplicate_index / 2 + 1)
            .zip(
                v.iter()
                    .skip(duplicate_index / 2 + 1)
                    .take(duplicate_index / 2 + 1)
                    .rev(),
            )
            .filter(|record| {
                let diff = diff(record.0, record.1);

                if part_b {
                    diff >= 1
                } else {
                    diff != 0
                }
            })
            .collect::<Vec<_>>();

        if !part_b && !differences.is_empty() {
            continue;
        }

        // we don't want the original answer in part b
        if part_b && differences.is_empty() {
            return None;
        }

        // any answer with more than 1 smudge is invalid
        if differences.len() > 1
            || (differences.len() == 1 && diff(differences[0].0, differences[0].1) > 1)
        {
            continue;
        }

        return Some(duplicate_index / 2 + 1);
    }

    None
}

fn duplicate_vec(v: &[String], part_b: bool) -> Option<usize> {
    if let Some(result) = dup(v, part_b) {
        return Some(result);
    }

    if let Some(result) = dup(&v.iter().cloned().rev().collect::<Vec<_>>(), part_b) {
        return Some(v.len() - result);
    }

    None
}

fn read_input() -> Vec<Pattern> {
    let input = include_str!("input")
        .split("\n\n")
        .map(|p| {
            let rows = p.lines().map(|line| line.to_string()).collect::<Vec<_>>();
            let mut columns = Vec::new();
            for i in 0..rows[0].len() {
                let mut column = String::new();
                for row in rows.iter() {
                    column.push(row.chars().nth(i).unwrap());
                }

                columns.push(column);
            }

            Pattern { rows, columns }
        })
        .collect::<Vec<_>>();

    input
}
