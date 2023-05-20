use statrs::distribution;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};
use Path::*;

#[derive(Clone, Debug)]
enum Path {
    Simple(String),
    Consecutive(Vec<Path>),
    Branch(Box<Path>, Box<Path>),
}

fn matching_closing_brace(s: &str, brace_idx: usize) -> usize {
    let mut balance = 0;
    for (i, c) in s[brace_idx..].chars().enumerate() {
        if c == ')' {
            balance -= 1;
            if balance == 0 {
                return brace_idx + i;
            }
        } else if c == '(' {
            balance += 1;
        }
    }
    return 0;
}

fn parse(path_regex: &str) -> Path {
    let mut paths: Vec<Path> = Vec::new();
    let mut current = String::new();

    let mut i = 0;
    while i < path_regex.len() {
        let c = path_regex.as_bytes()[i] as char;

        match c {
            '(' => {
                if current.len() > 0 {
                    paths.push(Path::Simple(current.clone()));

                    current = String::new();
                }
                let closing_idx = matching_closing_brace(path_regex, i);
                let inside = parse(&path_regex[i + 1..closing_idx]);
                paths.push(inside);

                i = closing_idx;
            }
            '|' => {
                if current.len() > 0 {
                    paths.push(Path::Simple(current));
                }
                let current_path = if paths.len() == 1 {
                    paths[0].clone()
                } else {
                    Path::Consecutive(paths.clone())
                };
                let right = parse(&path_regex[i + 1..]);
                return Path::Branch(Box::new(current_path), Box::new(right));
            }
            _ => {
                current.push(c);
            }
        }

        i += 1;
    }

    if current.len() > 0 {
        paths.push(Path::Simple(current.clone()));
    }

    if paths.len() == 1 {
        return paths[0].clone();
    }
    return Path::Consecutive(paths);
}

type Pos = (i32, i32);

fn follow(pos: Pos, dir: char) -> Pos {
    match dir {
        'N' => (pos.0, pos.1 + 1),
        'E' => (pos.0 + 1, pos.1),
        'W' => (pos.0 - 1, pos.1),
        'S' => (pos.0, pos.1 - 1),
        _ => todo!(),
    }
}

fn build_maze(
    path: &Path,
    distance_map: &mut HashMap<Pos, usize>,
    starts: &HashSet<Pos>,
) -> HashSet<Pos> {
    match path {
        Simple(p) => {
            let mut ends = starts.into_iter().map(|&x| x).collect::<Vec<_>>();

            for c in p.chars() {
                for e in &mut ends {
                    let prev = *e;
                    let next = follow(*e, c);

                    if !distance_map.contains_key(&next) {
                        distance_map.insert(next, distance_map[&prev] + 1);
                    } else {
                        let dist_prev = distance_map[&prev];
                        distance_map
                            .entry(next)
                            .and_modify(|d| *d = std::cmp::min(*d, dist_prev + 1));
                    }
                    *e = next;
                }
            }

            ends.into_iter().collect::<HashSet<_>>()
        }
        Branch(b1, b2) => {
            let (ends1, ends2) = (
                build_maze(&*b1, distance_map, starts),
                build_maze(&*b2, distance_map, starts),
            );
            ends1.union(&ends2).map(|&u| u).collect::<HashSet<_>>()
        }
        Consecutive(paths) => {
            let mut ends = starts.clone();
            for p in paths {
                ends = build_maze(p, distance_map, &ends);
            }
            ends
        }
    }
}

fn build_distance_map(p: &Path) -> HashMap<(i32, i32), usize> {
    let mut distance_map = HashMap::new();
    let start = (0, 0);
    distance_map.insert(start, 0);

    build_maze(p, &mut distance_map, &HashSet::from([start]));

    distance_map
}

fn part_one() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let parsed = parse(&line[1..line.len() - 1]);

        let distance_map = build_distance_map(&parsed);
        let longest = distance_map.iter().max_by_key(|&(_, d)| d).unwrap().1;
        println!("{longest}");
    }
}

fn part_two() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let parsed = parse(&line[1..line.len() - 1]);

        let distance_map = build_distance_map(&parsed);
        let paths_cnt = distance_map.iter().filter(|&(_, d)| *d >= 1000).count();
        println!("{paths_cnt}");
    }
}

fn main() {
    part_two();
}
