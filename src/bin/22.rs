use std::{
    cell,
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

use binary_heap_plus::BinaryHeap;
use scanf::sscanf;
use CellType::*;
use Tool::*;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Ord, PartialOrd)]
struct Pos {
    y: usize,
    x: usize,
}

fn read_input() -> (usize, Pos) {
    let mut depth: usize = 0;
    let mut pos = Pos { x: 0, y: 0 };
    for line in stdin().lines().map(|l| l.unwrap()) {
        if sscanf!(&line, "depth: {}", depth).is_ok() {}
        let (mut x, mut y) = (0, 0);
        if sscanf!(&line, "target: {},{}", x, y).is_ok() {
            pos = Pos { x: x, y: y };
        }
    }

    (depth, pos)
}

#[derive(Debug, Clone, Copy)]
enum CellType {
    Wet,
    Rocky,
    Narrow,
}

fn part_one() {
    let (depth, target) = read_input();
    let (tx, ty) = (target.x as usize, target.y as usize);

    let mut erosion_levels = HashMap::new();
    erosion_levels.insert(Pos { x: 0, y: 0 }, 0);

    let mut total_risk = 0;
    for y in 0..=ty {
        for x in 0..=tx {
            let pos = Pos { x: x, y: y };
            let erosion_lvl = get_erosion_level(pos, target, depth as usize, &mut erosion_levels);

            let cell_type = cell_type(erosion_lvl);
            let cell_risk = match cell_type {
                Rocky => 0,
                Wet => 1,
                Narrow => 2,
            };

            total_risk += cell_risk;
        }
    }

    println!("{total_risk}");
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Tool {
    ClimbingGear,
    Torch,
}

fn get_erosion_level(
    p: Pos,
    target: Pos,
    depth: usize,
    erosion_levels: &mut HashMap<Pos, i32>,
) -> i32 {
    if erosion_levels.contains_key(&p) {
        return erosion_levels[&p];
    }
    let geologic_idx;
    let m = 20183;

    let start = Pos { x: 0, y: 0 };

    if p == start || p == target {
        geologic_idx = 0
    } else if p.y == 0 {
        geologic_idx = (p.x * 16807) % m;
    } else if p.x == 0 {
        geologic_idx = (p.y * 48271) % m;
    } else {
        let l1 = get_erosion_level(Pos { x: p.x - 1, y: p.y }, target, depth, erosion_levels);
        let l2 = get_erosion_level(Pos { x: p.x, y: p.y - 1 }, target, depth, erosion_levels);

        geologic_idx = (((l1 as i64) * (l2 as i64)) as usize) % m;
    }

    let result = ((geologic_idx + depth) % m) as i32;
    erosion_levels.insert(p, result);

    result
}

fn cell_type(erosion_lvl: i32) -> CellType {
    match erosion_lvl % 3 {
        0 => Rocky,
        1 => Wet,
        2 => Narrow,
        _ => todo!(),
    }
}

fn is_acceptable_for(t: CellType, is_target: bool, tool: Option<Tool>) -> bool {
    if is_target {
        return tool == Some(Torch);
    }
    match t {
        Rocky => tool != None,
        Wet => tool == None || tool == Some(ClimbingGear),
        Narrow => tool == None || tool == Some(Torch),
    }
}

fn part_two() {
    let (depth, target) = read_input();

    let state: (i32, Pos, Option<Tool>) = (0, Pos { x: 0, y: 0 }, Some(Torch));

    let mut q = BinaryHeap::new_min();
    q.push(state);

    let mut seen = HashSet::new();
    let mut erosion_levels = HashMap::new();

    let start = Pos { x: 0, y: 0 };
    erosion_levels.insert(start, 0);

    let mut result = 0;
    while !q.is_empty() {
        let (time, pos, tool) = q.pop().unwrap();
        if pos == target && tool == Some(Torch) {
            result = time;
            break;
        }
        let key = (pos, tool);
        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);

        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let (nx, ny) = (pos.x as i32 + dx, pos.y as i32 + dy);
            if nx >= 0 && ny >= 0 {
                let new_pos = Pos {
                    x: nx as usize,
                    y: ny as usize,
                };

                let erosion_lvl = get_erosion_level(new_pos, target, depth, &mut erosion_levels);
                let new_type = cell_type(erosion_lvl);

                let current_type =
                    cell_type(get_erosion_level(pos, target, depth, &mut erosion_levels));

                let mut next_states = Vec::new();
                if is_acceptable_for(new_type, new_pos == target, tool) {
                    next_states.push((time + 1, new_pos, tool));
                } else {
                    for new_tool in [None, Some(ClimbingGear), Some(Torch)] {
                        if is_acceptable_for(new_type, new_pos == target, new_tool)
                            && is_acceptable_for(current_type, false, new_tool)
                        {
                            next_states.push((time + 8, new_pos, new_tool));
                        }
                    }
                }

                for state in next_states {
                    let seen_key = (state.1, state.2);
                    if !seen.contains(&seen_key) {
                        q.push(state);
                    }
                }
            }
        }
    }

    println!("{result}");
}

fn main() {
    part_two();
}
