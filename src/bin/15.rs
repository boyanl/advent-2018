use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Pos {
    y: i32,
    x: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Dir {
    x: i32,
    y: i32,
}

impl ops::Add<Dir> for Pos {
    type Output = Pos;
    fn add(self, rhs: Dir) -> Self::Output {
        return Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum UnitKind {
    ELF,
    GOBLIN,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Unit {
    kind: UnitKind,
    pos: Pos,
    hp: i32,
    ap: i32,
}

const UP: Dir = Dir { x: 0, y: -1 };
const DOWN: Dir = Dir { x: 0, y: 1 };
const LEFT: Dir = Dir { x: -1, y: 0 };
const RIGHT: Dir = Dir { x: 1, y: 0 };

fn is_adjacent(p1: Pos, p2: Pos) -> bool {
    return ((p1.x - p2.x).abs() == 1 && p1.y == p2.y)
        || ((p1.y - p2.y).abs() == 1 && p1.x == p2.x);
}

fn enemies(units: &Vec<Unit>, unitKind: UnitKind) -> Vec<Unit> {
    units
        .iter()
        .filter(|&other| other.kind != unitKind)
        .map(|&u| u)
        .collect::<Vec<_>>()
}

fn next_step_to_closest_enemy(unit: Unit, units: &Vec<Unit>, walls: &HashSet<Pos>) -> Option<Pos> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    let mut occupied = HashSet::new();
    for u in units {
        occupied.insert(u.pos);
    }

    let mut prev = HashMap::new();
    let mut destinations = Vec::new();

    q.push_back((unit.pos, 0));

    while !q.is_empty() {
        let (c, dist) = q.pop_front().unwrap();

        for dir in [UP, LEFT, RIGHT, DOWN] {
            let next = c + dir;
            if walls.contains(&next) || occupied.contains(&next) || visited.contains(&next) {
                continue;
            }

            if !prev.contains_key(&next) || prev[&next] > (dist + 1, c) {
                prev.insert(next, (dist + 1, c));
            }

            for u in units {
                if is_adjacent(u.pos, next) && u.kind != unit.kind {
                    destinations.push(next);
                }
            }

            q.push_back((next, dist + 1));
            visited.insert(next);
        }
    }

    let destination_opt = destinations.iter().min_by_key(|&d| (prev[d].0, d));

    if let Some(d) = destination_opt {
        let mut result = *d;

        while prev[&result].0 > 1 {
            result = prev[&result].1;
        }
        return Some(result);
    }

    None
}

fn choose_adjacent_enemy(unit: Unit, units: &Vec<Unit>) -> Option<Unit> {
    return units
        .iter()
        .filter(|&other| is_adjacent(unit.pos, other.pos) && other.kind != unit.kind)
        .min_by_key(|&u| (u.hp, (u.pos.y, u.pos.x)))
        .map(|&u| u);
}

fn simulate(units: &Vec<Unit>, walls: &HashSet<Pos>) -> (usize, Vec<Unit>) {
    let mut units = units.clone();
    units.sort_by_key(|u| (u.pos.y, u.pos.x));

    let mut turn = 0;

    loop {
        for i in 0..units.len() {
            let mut unit = units[i];
            if units[i].hp <= 0 {
                continue;
            }

            let alive = units
                .iter()
                .filter(|&u| u.hp > 0)
                .map(|&u| u)
                .collect::<Vec<_>>();
            let enemies = enemies(&alive, unit.kind);

            if enemies.is_empty() {
                // end combat
                return (turn, alive);
            }

            if let None = choose_adjacent_enemy(units[i], &alive) {
                if let Some(new_pos) = next_step_to_closest_enemy(unit, &alive, walls) {
                    units[i].pos = new_pos;
                    unit = units[i];
                }
            }

            if let Some(adjacent) = choose_adjacent_enemy(units[i], &alive) {
                let j = units.iter().position(|u| *u == adjacent).unwrap();

                units[j].hp -= unit.ap;
            }
        }

        turn += 1;
        units.retain(|u| u.hp > 0);
        units.sort_by_key(|u| (u.pos.y, u.pos.x));
    }
}

fn read_input() -> (Vec<Unit>, HashSet<Pos>) {
    let mut y = 0;
    let mut walls = HashSet::new();
    let mut units = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert(Pos { x: x as i32, y: y });
                }
                'G' => {
                    units.push(Unit {
                        kind: UnitKind::GOBLIN,
                        pos: Pos { x: x as i32, y: y },
                        hp: 200,
                        ap: 3,
                    });
                }
                'E' => {
                    units.push(Unit {
                        kind: UnitKind::ELF,
                        pos: Pos { x: x as i32, y: y },
                        hp: 200,
                        ap: 3,
                    });
                }
                _ => {}
            };
        }
        y += 1;
    }

    return (units, walls);
}

fn battle_result(turns: usize, remaining: &Vec<Unit>) -> i32 {
    let total_hp = remaining.iter().map(|u| u.hp).sum::<i32>();
    return (turns as i32) * total_hp;
}

fn part_one() {
    let (units, walls) = read_input();
    let (turns, remaining) = simulate(&units, &walls);

    println!("{}", battle_result(turns, &remaining));
}

fn part_two() {
    let (mut units, walls) = read_input();
    let total_elves = units.iter().filter(|&u| u.kind == UnitKind::ELF).count();

    let mut result = -1;
    for power in 3.. {
        for i in 0..units.len() {
            if units[i].kind == UnitKind::ELF {
                units[i].ap = power;
            }
        }

        let (turn, remaining) = simulate(&units, &walls);
        let remaining_elves = remaining
            .iter()
            .filter(|&u| u.kind == UnitKind::ELF)
            .count();

        if remaining_elves == total_elves {
            result = battle_result(turn, &remaining);
            break;
        }
    }

    println!("{result}");
}

fn main() {
    part_two();
}
