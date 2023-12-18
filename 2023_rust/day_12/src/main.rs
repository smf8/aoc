use std::collections::HashMap;

fn main() {
    solve_a();
    solve_b();
}

fn solve_b() {
    let mut rows = read_input();

    rows = rows
        .iter()
        .map(|r| {
            let mut springs = r.springs.clone();
            let mut reports = r.reports.clone();
            for _ in 0..4 {
                springs.push('?');
                springs.extend_from_slice(&r.springs);

                reports.extend_from_slice(&r.reports);
            }

            Row { springs, reports }
        })
        .collect::<Vec<_>>();

    let mut sum = 0u64;
    for row in rows {
        let mut dp_cache = HashMap::new();
        let res = solve_row(&row.springs, &row.reports, &mut dp_cache);
        sum += res as u64;
    }

    println!("{}", sum);
}

fn solve_a() {
    let rows = read_input();

    let mut sum = 0u64;
    for row in rows {
        let mut dp_cache = HashMap::new();
        let res = solve_row(&row.springs, &row.reports, &mut dp_cache);
        // println!("{}", res);
        sum += res as u64;
    }

    println!("{}", sum);
}

struct Row {
    springs: Vec<char>,
    reports: Vec<usize>,
}

fn is_in_cache(
    h: &HashMap<(String, String), usize>,
    springs: &[char],
    report: &[usize],
) -> Option<usize> {
    let first_key = springs.iter().collect::<String>();
    let second_key = report.iter().map(|num| num.to_string()).collect::<String>();

    h.get(&(first_key, second_key)).cloned()
}

fn put_in_cache(
    h: &mut HashMap<(String, String), usize>,
    springs: &[char],
    report: &[usize],
    value: usize,
) {
    let first_key = springs.iter().collect::<String>();
    let second_key = report.iter().map(|num| num.to_string()).collect::<String>();

    h.insert((first_key, second_key), value);
}

fn solve_row(
    springs: &[char],
    report: &[usize],
    dp_cache: &mut HashMap<(String, String), usize>,
) -> usize {
    let mut result = 0usize;

    if let Some(cached_value) = is_in_cache(dp_cache, springs, report) {
        return cached_value;
    }

    // last call of recursion.
    if report.len() == 1 {
        let mut char_index = 0;

        while let Some(match_position_index) =
            check_number(&springs[char_index..], report[0], report.len() == 1)
        {
            // found a fixed location for number of #s
            // we shouldn't continue
            if springs[char_index + match_position_index] == '#' {
                if springs[char_index + match_position_index + report[0]..].contains(&'#') {
                    return result;
                }

                result += 1;
                break;
            }

            // there is a match but there is an unattended #
            if springs[char_index + match_position_index + report[0]..].contains(&'#') {
                char_index += 1;
                // char_index = char_index + match_position_index + report[0] + damaged_index;
                continue;
            }

            result += 1;
            char_index += match_position_index + 1;
        }

        put_in_cache(dp_cache, springs, report, result);
        return result;
    }

    let mut char_index = 0;

    while let Some(match_position_index) =
        check_number(&springs[char_index..], report[0], report.len() == 1)
    {
        let sub_result = solve_row(
            &springs[match_position_index + char_index + report[0] + 1..],
            &report[1..],
            dp_cache,
        );

        result += sub_result;

        // found a fixed location for number of #s
        if springs[char_index + match_position_index] == '#' {
            break;
        }

        char_index += match_position_index + 1;
    }

    put_in_cache(dp_cache, springs, report, result);
    result
}

// check_number will check the puzzle and returns the first index
// that the num number of # can be placed.
fn check_number(spring_puzzle: &[char], num: usize, is_last: bool) -> Option<usize> {
    let mut pattern = vec!['#'; num];
    if !is_last {
        pattern.push('.');
    }

    for (i, window) in spring_puzzle.windows(pattern.len()).enumerate() {
        if window.iter().enumerate().all(|(idx, ch)| {
            if ch == pattern.get(idx).unwrap() || *ch == '?' {
                return true;
            }

            false
        }) {
            return Some(i);
        }

        if spring_puzzle[i] == '#' {
            return None;
        }
    }

    None
}

fn read_input() -> Vec<Row> {
    let input = include_str!("input")
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

            Row {
                springs: parts[0].chars().collect(),
                reports: parts[1]
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect(),
            }
        })
        .collect::<Vec<_>>();

    input
}
