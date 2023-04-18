use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    io::stdin,
};

use scanf::sscanf;
use statrs::{distribution, statistics::Distribution};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn zero() -> Vec2 {
    return Vec2 { x: 0, y: 0 };
}

fn min(v1: Vec2, v2: Vec2) -> Vec2 {
    Vec2 {
        x: std::cmp::min(v1.x, v2.x),
        y: std::cmp::min(v1.y, v2.y),
    }
}

fn max(v1: Vec2, v2: Vec2) -> Vec2 {
    Vec2 {
        x: std::cmp::max(v1.x, v2.x),
        y: std::cmp::max(v1.y, v2.y),
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Point {
    pos: Vec2,
    v: Vec2,
}

fn read_points() -> Vec<Point> {
    let mut points = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut x, mut y, mut dx, mut dy) = (0, 0, 0, 0);
        if sscanf!(&line, "position=<{}, {}> velocity=<{}, {}>", x, y, dx, dy).is_ok() {
            points.push(Point {
                pos: Vec2 { x: x, y: y },
                v: Vec2 { x: dx, y: dy },
            });
        }
    }

    points
}

fn tick(pts: &mut Vec<Point>) {
    for mut pt in pts {
        pt.pos.x += pt.v.x;
        pt.pos.y += pt.v.y;
    }
}

fn bounding_rect(pts: &Vec<Point>) -> (Vec2, Vec2) {
    let (mut top_left, mut bottom_right) = (
        Vec2 {
            x: i32::MAX,
            y: i32::MAX,
        },
        Vec2 {
            x: i32::MIN,
            y: i32::MIN,
        },
    );
    for &pt in pts {
        top_left = min(top_left, pt.pos);
        bottom_right = max(bottom_right, pt.pos);
    }

    return (top_left, bottom_right);
}

fn display(pts: &Vec<Point>) {
    let (top_left, bottom_right) = bounding_rect(pts);
    let pos_set: HashSet<Vec2> = HashSet::from_iter(pts.iter().map(|pt| pt.pos));

    for y in top_left.y - 1..=bottom_right.y + 1 {
        for x in top_left.x - 1..=bottom_right.x + 1 {
            if pos_set.contains(&Vec2 { x: x, y: y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn connected_points(pts: &HashSet<Vec2>, pt: Vec2) -> Vec<Vec2> {
    let mut q = VecDeque::new();
    q.push_back(pt);

    let mut visited = HashSet::new();
    visited.insert(pt);

    let mut result = Vec::new();
    result.push(pt);

    while !q.is_empty() {
        let cur = q.pop_front().unwrap();

        for (dx, dy) in [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ] {
            let next = Vec2 {
                x: cur.x + dx,
                y: cur.y + dy,
            };
            if pts.contains(&next) && !visited.contains(&next) {
                q.push_back(next);
                result.push(next);
                visited.insert(next);
            }
        }
    }

    result
}

/*
 * The approach is to identify the "connected components" of points in the current state, and take the
 * number of points for each component.
 * If the points spell out a message, then the number of points in each components should have a fairly high mean
 * (since each component should represent a letter), unlike a more random distribution, where you have a lot of "small" components
 * with e.g. sizes below 5 (with which you can't make a letter).
 */
fn show_message(points: &Vec<Point>) {
    let mut pts = points.clone();
    for i in 0.. {
        let mut visited = HashSet::new();
        let mut components = Vec::new();
        let positions = pts.iter().map(|p| p.pos).collect::<HashSet<_>>();

        for &pos in &positions {
            if !visited.contains(&pos) {
                let connected = connected_points(&positions, pos);
                components.push(connected.clone());

                for &pt2 in &connected {
                    visited.insert(pt2);
                }
            }
        }

        let counts = components
            .iter()
            .map(|pts| pts.len() as f64)
            .collect::<Vec<_>>();

        let distribution = statrs::statistics::Data::new(counts.clone());
        let (mean, stddev) = (
            distribution.mean().unwrap(),
            distribution.std_dev().unwrap(),
        );

        if mean >= 10.0 && stddev < 10.0 {
            display(&pts);
            println!("\n{i} iterations");
            break;
        }

        tick(&mut pts);
    }
}

fn main() {
    let points = read_points();
    show_message(&points);
}
