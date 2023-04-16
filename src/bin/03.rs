use scanf::sscanf;
use std::cmp::{max, min};
use std::{collections::HashSet, io::stdin};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Rect {
    up_left: Point,
    width: i32,
    height: i32,
}

fn intersection(a: Rect, b: Rect) -> Rect {
    let max_pt_a = Point {
        x: a.up_left.x + a.width,
        y: a.up_left.y + a.height,
    };
    let max_pt_b = Point {
        x: b.up_left.x + b.width,
        y: b.up_left.y + b.height,
    };

    let min_result = Point {
        x: max(a.up_left.x, b.up_left.x),
        y: max(a.up_left.y, b.up_left.y),
    };
    let max_result = Point {
        x: min(max_pt_a.x, max_pt_b.x),
        y: min(max_pt_a.y, max_pt_b.y),
    };

    return Rect {
        up_left: min_result,
        width: max_result.x - min_result.x,
        height: max_result.y - min_result.y,
    };
}

fn points(a: Rect) -> Vec<Point> {
    let mut result = Vec::new();
    for y in a.up_left.y..a.up_left.y + a.height {
        for x in a.up_left.x..a.up_left.x + a.width {
            result.push(Point { x: x, y: y });
        }
    }
    return result;
}

fn parse_rects() -> Vec<Rect> {
    let mut rects = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut n, mut x, mut y, mut w, mut h) = (0, 0, 0, 0, 0);
        if sscanf!(&line, "#{} @ {},{}: {}x{}", n, x, y, w, h).is_ok() {
            rects.push(Rect {
                up_left: Point { x: x, y: y },
                width: w,
                height: h,
            });
        }
    }

    return rects;
}

fn find_overlapping_points_count(rects: &Vec<Rect>) -> usize {
    let mut overlapping = HashSet::new();
    for i in 0..rects.len() {
        for j in i + 1..rects.len() {
            let (rect_i, rect_j) = (rects[i], rects[j]);
            for p in points(intersection(rect_i, rect_j)) {
                overlapping.insert(p);
            }
        }
    }

    return overlapping.len();
}

fn part_one() {
    let rects = parse_rects();
    let result = find_overlapping_points_count(&rects);
    println!("{}", result);
}

fn overlaps(a: Rect, b: Rect) -> bool {
    let r = intersection(a, b);
    return r.width > 0 && r.height > 0;
}

fn part_two() {
    let rects = parse_rects();
    let mut result = 0;
    for i in 0..rects.len() {
        let rect = rects[i];
        let mut ok = true;
        for j in 0..rects.len() {
            if i != j && overlaps(rect, rects[j]) {
                ok = false;
                break;
            }
        }
        if ok {
            result = i + 1;
            break;
        }
    }
    println!("{result}");
}

fn main() {
    part_two();
}
