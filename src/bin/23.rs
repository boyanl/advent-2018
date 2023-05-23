use std::{
    cmp::{max, min, Ordering},
    io::stdin,
    ops::{Add, Mul},
};

use binary_heap_plus::BinaryHeap;
use scanf::sscanf;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn from(t: (i64, i64, i64)) -> Pos {
        return Pos {
            x: t.0,
            y: t.1,
            z: t.2,
        };
    }
}

impl std::ops::Index<usize> for Pos {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => todo!(),
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Pos;
    fn mul(self, rhs: i32) -> Self::Output {
        let c = rhs as i64;
        return Pos {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
        };
    }
}

fn dist(p1: Pos, p2: Pos) -> i64 {
    return (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs();
}

type PosTuple = (i64, i64, i64);

impl std::ops::Add<PosTuple> for Pos {
    type Output = Pos;
    fn add(self, rhs: PosTuple) -> Self::Output {
        Pos {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
        }
    }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Clone, Copy)]
struct Nanobot {
    pos: Pos,
    radius: i64,
}

fn read_input() -> Vec<Nanobot> {
    let mut result = Vec::new();

    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut x, mut y, mut z, mut r) = (0, 0, 0, 0);
        if sscanf!(&line, "pos=<{},{},{}>, r={}", x, y, z, r).is_ok() {
            result.push(Nanobot {
                pos: Pos { x: x, y: y, z: z },
                radius: r,
            });
        }
    }

    result
}

fn part_one() {
    let bots = read_input();

    let max_range_bot = bots.iter().max_by_key(|bot| bot.radius).unwrap();
    let in_range = bots
        .iter()
        .filter(|b| dist(b.pos, max_range_bot.pos) <= max_range_bot.radius)
        .count();

    println!("{in_range}");
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Box {
    min: Pos,
    max: Pos,
}

fn intersects(b: Box, bot: Nanobot) -> bool {
    let mut total_d = 0;
    for i in 0..3 {
        let (l, h) = (b.min[i], b.max[i] - 1);
        let d = if l <= bot.pos[i] && bot.pos[i] <= h {
            0
        } else {
            std::cmp::min((l - bot.pos[i]).abs(), (h - bot.pos[i]).abs())
        };
        total_d += d;
    }

    total_d <= bot.radius
}

fn bots_in_range(bots: &Vec<Nanobot>, b: Box) -> usize {
    bots.iter().filter(|&bot| intersects(b, *bot)).count()
}

type QueueEl = (usize, i64, i64, Box);

fn part_two() {
    let bots = read_input();

    let origin = Pos { x: 0, y: 0, z: 0 };

    let mut radius = [0i64, 0i64, 0i64];
    for i in 0..3 {
        let min_c = bots.iter().map(|b| b.pos[i]).min().unwrap();
        let max_c = bots.iter().map(|b| b.pos[i]).max().unwrap();
        radius[i] = max(min_c.abs(), max_c.abs());
    }

    let mut r = 1;
    for _ in 1.. {
        r *= 2;
        if r >= radius[0] && r >= radius[1] && r >= radius[2] {
            break;
        }
    }

    let b = Box {
        min: Pos {
            x: -r,
            y: -r,
            z: -r,
        },
        max: Pos { x: r, y: r, z: r },
    };

    let mut q = BinaryHeap::new_by(|el1: &QueueEl, el2: &QueueEl| {
        let (count1, dist1, size1, box1) = *el1;
        let (count2, dist2, size2, box2) = *el2;

        let count_cmp = count1.cmp(&count2);
        if count_cmp != Ordering::Equal {
            return count_cmp;
        }

        let dist_cmp = dist2.cmp(&dist1);
        if dist_cmp != Ordering::Equal {
            return dist_cmp;
        }

        let size_cmp = size1.cmp(&size2);
        if size_cmp != Ordering::Equal {
            return size_cmp;
        }

        box1.cmp(&box2)
    });
    let in_range = bots_in_range(&bots, b);
    q.push((in_range, dist(b.min, origin), 2 * r, b));
    while !q.is_empty() {
        let (_, distance, size, b) = q.pop().unwrap();

        if size == 1 {
            println!("{distance}");
            break;
        }

        let new_size = size / 2;
        for oct in [
            (0, 0, 0),
            (0, 0, 1),
            (0, 1, 0),
            (0, 1, 1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, 0),
            (1, 1, 1),
        ] {
            let new_min = b.min + Pos::from(oct) * (new_size as i32);
            let new_max = new_min + (new_size, new_size, new_size);
            let new_box = Box {
                min: new_min,
                max: new_max,
            };
            let in_range = bots_in_range(&bots, new_box);
            q.push((in_range, dist(new_box.min, origin), new_size, new_box));
        }
    }
}

fn main() {
    part_two();
}
