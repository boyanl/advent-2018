use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

use rustc_hash::FxHashSet;
use scanf::sscanf;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Pos {
    y: i32,
    x: i32,
}

struct WallRange {
    ymin: i32,
    ymax: i32,
    xmin: i32,
    xmax: i32,
}

fn read_input() -> Vec<WallRange> {
    let mut ranges = Vec::new();

    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut x, mut ymin, mut ymax) = (0, 0, 0);
        let (mut y, mut xmin, mut xmax) = (0, 0, 0);
        if sscanf!(&line, "x={i32}, y={i32}..{i32}", x, ymin, ymax).is_ok() {
            ranges.push(WallRange {
                ymin: ymin,
                ymax: ymax,
                xmin: x,
                xmax: x,
            });
        } else if sscanf!(&line, "y={i32}, x={i32}..{i32}", y, xmin, xmax).is_ok() {
            ranges.push(WallRange {
                ymin: y,
                ymax: y,
                xmin: xmin,
                xmax: xmax,
            });
        }
    }

    ranges
}

type Walls = HashSet<Pos>;

fn construct_walls(ranges: &Vec<WallRange>) -> Walls {
    let mut result = HashSet::new();

    for range in ranges {
        for x in range.xmin..=range.xmax {
            for y in range.ymin..=range.ymax {
                result.insert(Pos { x: x, y: y });
            }
        }
    }

    result
}

fn is_wall(pos: Pos, walls: &Walls) -> bool {
    return walls.contains(&pos);
}

fn min_wall_y(walls: &Walls) -> i32 {
    return walls.iter().map(|p| p.y).min().unwrap();
}

fn max_wall_y(walls: &Walls) -> i32 {
    return walls.iter().map(|p| p.y).max().unwrap();
}

fn is_free(pos: Pos, walls: &Walls, water: &FxHashSet<Pos>) -> bool {
    return !is_wall(pos, walls) && !water.contains(&pos);
}

fn left(pos: Pos) -> Pos {
    return Pos {
        y: pos.y,
        x: pos.x - 1,
    };
}

fn right(pos: Pos) -> Pos {
    return Pos {
        y: pos.y,
        x: pos.x + 1,
    };
}

fn down(pos: Pos) -> Pos {
    return Pos {
        y: pos.y + 1,
        x: pos.x,
    };
}

fn fill_left(walls: &Walls, start: Pos, water: &mut FxHashSet<Pos>, max_y: i32) -> Option<Pos> {
    let mut pos = start;
    let mut bottom = down(pos);

    while !is_free(bottom, walls, water) && is_free(left(pos), walls, water) {
        pos = left(pos);
        bottom = down(pos);
    }

    if !is_free(bottom, walls, water) {
        return Some(pos);
    }

    return fill(walls, bottom, water, max_y);
}

fn fill_right(walls: &Walls, start: Pos, water: &mut FxHashSet<Pos>, max_y: i32) -> Option<Pos> {
    let mut pos = start;
    let mut bottom = down(pos);

    while !is_free(bottom, walls, water) && is_free(right(pos), walls, water) {
        pos = right(pos);
        bottom = down(pos);
    }

    if !is_free(bottom, walls, water) {
        return Some(pos);
    }

    return fill(walls, bottom, water, max_y);
}

fn fill(walls: &Walls, start: Pos, water: &mut FxHashSet<Pos>, max_y: i32) -> Option<Pos> {
    let mut positions = VecDeque::new();
    positions.push_back(start);
    let mut pos = start;
    let mut bottom = down(pos);

    while is_free(bottom, walls, water) {
        if bottom.y > max_y {
            return None;
        }
        pos = bottom;
        bottom = down(pos);
        positions.push_back(pos);
    }

    while !positions.is_empty() {
        pos = positions.pop_back().unwrap();

        let mut left = fill_left(walls, pos, water, max_y);
        while let Some(l) = left {
            if l.y == pos.y {
                break;
            }
            left = fill_left(walls, start, water, max_y);
        }

        let mut right = fill_right(walls, pos, water, max_y);
        while let Some(r) = right {
            if r.y == pos.y {
                break;
            }
            right = fill_right(walls, start, water, max_y);
        }

        if let (Some(l), Some(r)) = (left, right) {
            if l.y == r.y {
                for x in l.x..=r.x {
                    water.insert(Pos { y: l.y, x: x });
                }
            }
        } else {
            return None;
        }
    }
    Some(pos)
}

fn water_freefall_path(walls: &Walls, start: Pos, water: &FxHashSet<Pos>) -> Vec<Pos> {
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut visited = HashSet::new();
    visited.insert(start);

    let min_y = min_wall_y(walls);
    let max_y = max_wall_y(walls);

    while !q.is_empty() {
        let c = q.pop_front().unwrap();

        if c.y >= max_y {
            continue;
        }

        let (down, left, right) = (
            Pos { x: c.x, y: c.y + 1 },
            Pos { x: c.x - 1, y: c.y },
            Pos { x: c.x + 1, y: c.y },
        );

        let (left_free, right_free) = (is_free(left, walls, water), is_free(right, walls, water));
        if is_free(down, walls, water) {
            if !visited.contains(&down) {
                q.push_back(down);
                visited.insert(down);
            }
        } else {
            if left_free && !visited.contains(&left) {
                q.push_back(left);
                visited.insert(left);
            }
            if right_free && !visited.contains(&right) {
                q.push_back(right);
                visited.insert(right);
            }
        }
    }

    return visited
        .iter()
        .filter(|&pos| pos.y >= min_y && pos.y <= max_y)
        .map(|&pos| pos)
        .collect::<Vec<_>>();
}

fn display(water: &FxHashSet<Pos>, water_path: &Vec<Pos>, walls: &Walls, source: Pos) {
    let max_y = max_wall_y(walls);
    let min_x = walls.iter().map(|p| p.x).min().unwrap();
    let max_x = walls.iter().map(|p| p.x).max().unwrap();

    for y in 0..=max_y + 1 {
        for x in min_x - 1..=max_x + 1 {
            let pos = Pos { y: y, x: x };
            if is_wall(pos, walls) {
                print!("#");
            } else if water.contains(&pos) {
                print!("~");
            } else if water_path.contains(&pos) {
                print!("|");
            } else if pos == source {
                print!("+");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn count_reachable_squares(walls: &Walls, start: Pos) -> usize {
    let mut water = FxHashSet::default();
    let max_y = max_wall_y(walls);

    fill(walls, start, &mut water, max_y);

    let water_path = water_freefall_path(walls, start, &water);

    return water.len() + water_path.len();
}

fn part_one() {
    let ranges = read_input();
    let walls = construct_walls(&ranges);
    let source = Pos { x: 500, y: 0 };

    let result = count_reachable_squares(&walls, source);
    println!("{result}");
}

fn count_water_at_rest(walls: &Walls, start: Pos) -> usize {
    let mut water = FxHashSet::default();
    let max_y = max_wall_y(walls);

    fill(walls, start, &mut water, max_y);

    return water.len();
}

fn part_two() {
    let ranges = read_input();
    let walls = construct_walls(&ranges);
    let source = Pos { x: 500, y: 0 };

    let result = count_water_at_rest(&walls, source);
    println!("{result}");
}

fn main() {
    part_two();
}
