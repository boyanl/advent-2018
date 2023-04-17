use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::stdin;

use scanf::sscanf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn pt_min(p1: Point, p2: Point) -> Point {
    Point {
        x: min(p1.x, p2.x),
        y: min(p1.y, p2.y),
    }
}

fn pt_max(p1: Point, p2: Point) -> Point {
    Point {
        x: max(p1.x, p2.x),
        y: max(p1.y, p2.y),
    }
}

fn read_points() -> Vec<Point> {
    let mut points = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut x, mut y) = (0, 0);
        if sscanf!(&line, "{}, {}", x, y).is_ok() {
            points.push(Point { x: x, y: y });
        }
    }

    points
}

fn bounding_rect(points: &Vec<Point>) -> (Point, Point) {
    let (mut min, mut max) = (
        Point {
            x: i32::MAX,
            y: i32::MAX,
        },
        Point {
            x: i32::MIN,
            y: i32::MIN,
        },
    );

    for &point in points {
        min = pt_min(min, point);
        max = pt_max(max, point);
    }
    return (min, max);
}

fn points_in_rect(top_left: Point, bottom_right: Point) -> Vec<Point> {
    let mut points = Vec::new();
    for y in top_left.y..=bottom_right.y {
        for x in top_left.x..=bottom_right.x {
            points.push(Point { x: x, y: y });
        }
    }

    points
}

fn distance(p1: Point, p2: Point) -> i32 {
    return (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
}

fn most_closest_points_in_bounding_rect(points: &Vec<Point>) -> i32 {
    let (top_left, bottom_right) = bounding_rect(points);
    let mut closest = HashMap::new();

    for pt in points_in_rect(top_left, bottom_right) {
        let mut closest_dist = i32::MAX;
        let mut closest_place = -1;
        for (i, &place) in points.iter().enumerate() {
            let dist = distance(place, pt);
            if dist < closest_dist {
                closest_dist = dist;
                closest_place = i as i32;
            } else if dist == closest_dist {
                closest_place = -1;
            }
        }
        if closest_place != -1 {
            closest.insert(pt, closest_place);
        }
    }

    let mut result = i32::MIN;
    for i in 0..points.len() {
        let mut cnt = 0;
        let mut infinite = false;
        for (&pt, &closest_place) in &closest {
            if closest_place == (i as i32) {
                cnt += 1;
                if pt.x == top_left.x
                    || pt.y == top_left.y
                    || pt.x == bottom_right.x
                    || pt.y == bottom_right.y
                {
                    infinite = true;
                }
            }
        }

        if !infinite {
            result = max(result, cnt);
        }
    }

    result
}

fn part_one() {
    let places = read_points();
    let result = most_closest_points_in_bounding_rect(&places);
    println!("{result}");
}

fn part_two() {
    let places = read_points();

    let (top_left, bottom_right) = bounding_rect(&places);
    let threshold = 10000;

    let result = points_in_rect(top_left, bottom_right)
        .iter()
        .filter(|&pt| {
            places
                .iter()
                .map(|place| distance(*pt, *place))
                .sum::<i32>()
                < threshold
        })
        .count();

    println!("{result}");
}

fn main() {
    part_two()
}
