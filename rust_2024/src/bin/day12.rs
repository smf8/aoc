use grid::{grid, Grid};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

fn main() {
    let example = include_str!("../../inputs/day12/example.txt");
    let main_input = include_str!("../../inputs/day12/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

struct Area {
    _char: char,
    plots_map: HashSet<(i32, i32)>,
    plot_sides: RefCell<HashMap<(i32, i32), Vec<Dir>>>,
    plots: HashMap<usize, Vec<usize>>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn all() -> &'static [Dir] {
        &[Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    }

    fn delta(&self) -> (i32, i32) {
        match *self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
}

impl Area {
    fn new(ch: char) -> Self {
        Area {
            _char: ch,
            plots_map: HashSet::new(),
            plot_sides: RefCell::new(HashMap::new()),
            plots: HashMap::new(),
        }
    }

    fn area(&self) -> usize {
        self.plots.values().fold(0, |acc, x| acc + x.len())
    }

    fn perimeter(&self) -> usize {
        self.plots_map
            .iter()
            .map(|a| {
                let mut perimeter = 0;
                for dir in Dir::all() {
                    let delta = dir.delta();
                    let (row, col) = (a.0 + delta.0, a.1 + delta.1);
                    if !self.plots_map.contains(&(row, col)) {
                        perimeter += 1;
                    }
                }

                perimeter
            })
            .sum()
    }

    fn mark_neighbor(&self, mut plot: (i32, i32), move_dir: Dir, check_dir: Dir) {
        let delta = move_dir.delta();
        while self.plots_map.contains(&(plot.0, plot.1)) {
            let check_delta = check_dir.delta();
            if self
                .plots_map
                .contains(&(plot.0 + check_delta.0, plot.1 + check_delta.1))
            {
                break;
            }

            self.plot_sides
                .borrow_mut()
                .entry(plot)
                .or_default()
                .push(check_dir.clone());
            plot = (plot.0 + delta.0, plot.1 + delta.1);
        }
    }

    fn process_sides(&mut self) -> usize {
        let mut result = 0;

        // for vertical sides we need to check RIGHT/LEFT and go UP/DOWN
        // for horizontal sides we need to check UP/DOWN and go RIGHT/LEFT
        let check_cases = [
            ([Dir::Right, Dir::Left], [Dir::Up, Dir::Down]),
            ([Dir::Up, Dir::Down], [Dir::Right, Dir::Left]),
        ];

        for plot in self.plots_map.iter() {
            for case in check_cases.iter() {
                for check_dir in case.0.iter() {
                    // check this can become part of a side
                    let delta = check_dir.delta();
                    if !self
                        .plots_map
                        .contains(&(plot.0 + delta.0, plot.1 + delta.1))
                    {
                        let mut mark = false;
                        if let Some(plot_sides) = self.plot_sides.borrow().get(plot).cloned() {
                            if !plot_sides.contains(check_dir) {
                                result += 1;
                                mark = true;
                            }
                        } else {
                            result += 1;
                            mark = true;
                        }

                        if mark {
                            for move_dir in case.1.iter() {
                                self.mark_neighbor(*plot, move_dir.clone(), check_dir.clone());
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

fn parse_map(input: &str) -> Vec<Area> {
    let mut map = grid![];

    for line in input.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }

        map.push_row(row);
    }

    let mut areas = Vec::new();
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if map[(row, col)] != '-' {
                areas.push(find_region(&mut map, (row as i32, col as i32)))
            }
        }
    }

    areas
}

fn find_region(map: &mut Grid<char>, (row, col): (i32, i32)) -> Area {
    let start_char = map[(row as usize, col as usize)];
    let mut area = Area::new(start_char);

    // good ol' BFS
    let mut queue = vec![(row, col)];
    let mut visited = HashMap::new();
    visited.insert((row, col), true);

    while let Some((r, c)) = queue.pop() {
        area.plots.entry(r as usize).or_default().push(c as usize);
        area.plots_map.insert((r, c));

        for (dr, dc) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nr = r + dr;
            let nc = c + dc;
            if let Some(c) = map.get(nr, nc) {
                if *c == start_char && !visited.contains_key(&(nr, nc)) {
                    queue.push((nr, nc));
                    visited.insert((nr, nc), true);
                }
            }
        }

        map[(r as usize, c as usize)] = '-';
    }

    area
}

fn solve_a(input: &str) {
    let areas = parse_map(input);

    let result = areas
        .iter()
        .map(|area| {
            let (area, perimeter) = (area.area(), area.perimeter());

            area * perimeter
        })
        .sum::<usize>();

    println!("part 1: {}", result)
}

fn solve_b(input: &str) {
    let mut areas = parse_map(input);

    let result = areas
        .iter_mut()
        .map(|area| {
            let sides = area.process_sides();
            let area = area.area();

            sides * area
        })
        .sum::<usize>();

    println!("part 2: {}", result)
}
