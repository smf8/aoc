use std::collections::{BTreeMap, HashMap, HashSet};
use std::os::unix::raw::time_t;
use std::{cmp, vec};
use std::cmp::Ordering;

fn main() {
    let example = include_str!("../../inputs/day9/example.txt");
    let main_input = include_str!("../../inputs/day9/main.txt");

    solve_a(example);
    solve_a(main_input);

    solve_b(example);
    solve_b(main_input);
}

//3360 too low
fn solve_a(input: &str) {
    let seats = input
        .lines()
        .map(|l| {
            let numbers = l
                .split(',')
                .map(|m| m.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            (numbers[0], numbers[1])
        })
        .collect::<Vec<_>>();

    let mut areas = Vec::new();

    for (i, s1) in seats[..seats.len() - 1].iter().enumerate() {
        for s2 in seats[i..].iter() {
            areas.push((
                ((s1.0 - s2.0).abs() + 1) * ((s1.1 - s2.1).abs() + 1),
                (s1, s2),
            ));
        }
    }

    areas.sort();

    println!("Part1: {}", areas.last().unwrap().0);
}

fn solve_b(input: &str) {
    let mut seats = input
        .lines()
        .map(|l| {
            let numbers = l
                .split(',')
                .map(|m| m.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            (numbers[0], numbers[1])
        })
        .collect::<Vec<_>>();

    let mut areas = Vec::new();

    for (i, s1) in seats[..seats.len() - 1].iter().enumerate() {
        for s2 in seats[i..].iter() {
            areas.push((
                ((s1.0 - s2.0).abs() + 1) * ((s1.1 - s2.1).abs() + 1),
                (*s1, *s2),
            ));
        }
    }

    areas.sort();

    // we need to determine if the vertices are in clockwise or counter-clockwise order
    let corner = seats
        .iter()
        .enumerate()
        .min_by(|a, b| match a.1 .0.cmp(&b.1 .0) {
            cmp::Ordering::Equal => a.1 .1.cmp(&b.1 .1),
            x => x,
        })
        .unwrap();

    let mut is_clockwise = false;
    // next vertex is down -> counter-clockwise
    if corner.1 .0 == seats[corner.0 + 1].0 {
        is_clockwise = false;
    } else {
        is_clockwise = true;
    }

    let mut edges_lines = Vec::new();

    seats.push(*seats.first().unwrap());

    for tuple in seats.windows(2) {
        let mut line = Line::new(tuple[0], tuple[1]);

        let in_dir = match (tuple[0].0 - tuple[1].0, tuple[0].1 - tuple[1].1) {
            (0, y) if y > 0 => {
                // up going edge
                if is_clockwise {
                    Dir::RIGHT
                } else {
                    Dir::LEFT
                }
            }
            (0, y) if y < 0 => {
                // down going edge
                if is_clockwise {
                    Dir::LEFT
                } else {
                    Dir::RIGHT
                }
            }
            (x, 0) if x < 0 => {
                // right going edge
                if is_clockwise {
                    Dir::DOWN
                } else {
                    Dir::UP
                }
            }
            (x, 0) if x > 0 => {
                // left going edge
                if is_clockwise {
                    Dir::UP
                } else {
                    Dir::DOWN
                }
            }
            _ => {
                panic!("fasdkjhfsdkjhdf")
            }
        };

        line.Inside = in_dir;

        edges_lines.push(line);
    }

    for area in areas.iter().rev() {
        let p1 = (area.1 .0 .0, area.1 .1 .1);
        let p2 = (area.1 .1 .0, area.1 .0 .1);
        let l1 = Line::new(area.1.0, p1);
        let l2 = Line::new(area.1.0, p2);
        let l3 = Line::new(area.1.1, p1);
        let l4 = Line::new(area.1.1, p2);

        if validate_line(&edges_lines, &l1)
            && validate_line(&edges_lines, &l2)
            && validate_line(&edges_lines, &l3) && validate_line(&edges_lines, &l4) {
            println!("Part2: {:?}", area);

            return;
        }
    }
}

#[derive(PartialEq)]
enum Dir{
    RIGHT,
    LEFT,
    DOWN,
    UP,
}

struct Line {
    start: (i64, i64),
    len: i64,
    dir: Dir,
    Inside: Dir,
}

impl Line {
    fn new(start: (i64, i64), end: (i64, i64)) -> Self {
        let dir = match (start.0 - end.0, start.1 - end.1) {
            (0, y) if y > 0 => (Dir::UP, -y),
            (0, y) if y < 0 => (Dir::DOWN, -y),
            (x, 0) if x > 0 => (Dir::LEFT, -x),
            (x, 0) if x < 0 => (Dir::RIGHT, -x),
            _ => panic!("fasskhjfd")
        };

        Line{
            start,
            len: dir.1,
            dir: dir.0,
            Inside: Dir::RIGHT,
        }
    }

    fn end(&self) -> (i64, i64) {
        match self.dir {
            Dir::RIGHT | Dir::LEFT => (self.start.0 + self.len, self.start.1),
            Dir::DOWN | Dir::UP => (self.start.0, self.start.1 + self.len),
        }
    }

    fn does_cross(&self, line: &Line) -> Option<(i64, i64)> {
        let is_in_between = match line.dir {
            Dir::RIGHT | Dir::LEFT => is_between(line.start.1, self.start.1, self.end().1),
            Dir::UP | Dir::DOWN => is_between(line.start.0, self.start.0, self.end().0),
        };

        match line.dir {
            Dir::RIGHT if line.start.0 <= self.start.0 => {
                // the line reaches the edge
                if line.end().0 >= self.start.0 && is_in_between{
                    return Some((self.start.0, line.start.1))
                }
            }
            Dir::LEFT if line.start.0 >= self.start.0 => {
                if line.end().0 <= self.start.0 && is_in_between{
                    return Some((self.start.0, line.start.1))
                }
            }
            Dir::DOWN if line.start.1 <= self.start.1 => {
                if line.end().1 >= self.start.1 && is_in_between {
                    return Some((line.start.0, self.start.1))
                }
            }
            Dir::UP if line.start.1 >= self.start.1 => {
                if line.end().1 <= self.start.1 && is_in_between {
                    return Some((line.start.0, self.start.1))
                }
            }
            _ => {}
        }

        None
    }
}

fn validate_line(edges: &[Line], line: &Line) -> bool {
    for edge in edges {
        match (&edge.dir, &line.dir) {
            (Dir::RIGHT | Dir::LEFT, Dir::RIGHT | Dir::LEFT) => continue,
            (Dir::UP | Dir::DOWN, Dir::UP | Dir::DOWN) => continue,
            (_, _) => {}
        }


        if let Some(intersection) = edge.does_cross(line) {
            match line.dir {
                Dir::RIGHT if intersection.0 + 1 <= line.end().0 => {
                    if !point_in_polygon(edges, (intersection.0 + 1, line.start.1)) {
                        return false
                    }
                }

                Dir::LEFT if intersection.0 - 1 >= line.end().0 => {
                    if !point_in_polygon(edges, (intersection.0 - 1, line.start.1)) {
                        return false
                    }
                }
                Dir::DOWN if intersection.1 + 1 <= line.end().1 => {
                    if !point_in_polygon(edges, (line.start.0, intersection.1 + 1)) {
                        return false
                    }
                }
                Dir::UP if intersection.1 - 1 >= line.end().1 => {
                    if !point_in_polygon(edges, (line.start.0, intersection.1 - 1)) {
                        return false
                    }
                }
                _ => ()
            }
        }

    }

    point_in_polygon(edges, line.start)
}

fn point_in_polygon(edges: &[Line], p: (i64, i64)) -> bool {
    let mut nearest_right_edge = &Line{
        start: (i64::MAX, 0),
        len: 0,
        dir: Dir::RIGHT,
        Inside: Dir::RIGHT,
    };

    for edge in edges {
        // check if it's overlapping the edges
        match edge.dir {
            Dir::RIGHT | Dir::LEFT => {
                if edge.start.1 == p.1 && is_between(p.0, edge.start.0, edge.end().0){
                    return true
                }
            }
            Dir::DOWN | Dir::UP => {
                if is_between(p.1, edge.start.1, edge.end().1){
                    if edge.start.0 == p.0{
                        return true
                    }

                    if edge.start.0 > p.0 && edge.start.0 < nearest_right_edge.start.0{
                        nearest_right_edge = edge;
                    }
                }
            }
        }
    }

    if nearest_right_edge.Inside == Dir::LEFT {
        true
    }else{
        false
    }
}

fn is_between(a: i64, b1: i64, b2: i64) -> bool {
    a >= cmp::min(b1, b2) && a <= cmp::max(b1, b2)
}