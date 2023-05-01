use std::{
    collections::HashSet,
    fmt::{Display, Result},
    io::stdin,
    ops,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
const UP: Dir = Dir { x: 0, y: -1 };
const DOWN: Dir = Dir { x: 0, y: 1 };
const LEFT: Dir = Dir { x: -1, y: 0 };
const RIGHT: Dir = Dir { x: 1, y: 0 };

const INTERSECTION_DIRECTIONS: [Dir; 3] = [LEFT, UP, RIGHT];
const DIRECTIONS: [Dir; 4] = [RIGHT, UP, LEFT, DOWN];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Cart {
    pos: Pos,
    direction: Dir,
    intersection_dir_idx: usize,
}

fn is_cart(c: char) -> bool {
    return "^<>v".contains(c);
}

fn rotate(d1: Dir, d2: Dir) -> Dir {
    let idx = DIRECTIONS.iter().position(|&r| r == d1).unwrap();
    return match d2 {
        LEFT => DIRECTIONS[(idx + 1) % 4],
        RIGHT => DIRECTIONS[(idx + 3) % 4],
        UP => d1,
        _ => todo!("unexpected direction"),
    };
}

fn find_carts(tracks: &Vec<Vec<char>>) -> Vec<Cart> {
    let mut result = Vec::new();
    for i in 0..tracks.len() {
        for j in 0..tracks[i].len() {
            if !is_cart(tracks[i][j]) {
                continue;
            }
            let pos = Pos {
                x: j as i32,
                y: i as i32,
            };
            let dir = match tracks[i][j] {
                '^' => UP,
                '>' => RIGHT,
                '<' => LEFT,
                'v' => DOWN,
                _ => todo!(),
            };

            result.push(Cart {
                pos: pos,
                direction: dir,
                intersection_dir_idx: 0,
            });
        }
    }

    result
}

fn move_cart(tracks: &Vec<Vec<char>>, cart: Cart) -> Cart {
    let next_pos = cart.pos + cart.direction;
    let mut next_dir = cart.direction;

    let next_track = tracks[next_pos.y as usize][next_pos.x as usize];
    let mut result = cart;

    if next_track == '/' || next_track == '\\' {
        next_dir = match cart.direction {
            LEFT => {
                if next_track == '/' {
                    DOWN
                } else {
                    UP
                }
            }
            RIGHT => {
                if next_track == '/' {
                    UP
                } else {
                    DOWN
                }
            }
            UP => {
                if next_track == '/' {
                    RIGHT
                } else {
                    LEFT
                }
            }
            DOWN => {
                if next_track == '/' {
                    LEFT
                } else {
                    RIGHT
                }
            }
            _ => todo!(),
        }
    } else if next_track == '+' {
        // handle intersections
        next_dir = rotate(
            cart.direction,
            INTERSECTION_DIRECTIONS[cart.intersection_dir_idx],
        );
        result.intersection_dir_idx =
            (cart.intersection_dir_idx + 1) % INTERSECTION_DIRECTIONS.len();
    }

    result.pos = next_pos;
    result.direction = next_dir;
    result
}

fn move_carts_once(tracks: &Vec<Vec<char>>, carts: &Vec<Cart>) -> (Vec<Cart>, Option<Pos>) {
    let mut carts = carts.clone();
    carts.sort_by(|c1, c2| {
        let cmp_y = c1.pos.y.cmp(&c2.pos.y);
        return if cmp_y != std::cmp::Ordering::Equal {
            cmp_y
        } else {
            c1.pos.x.cmp(&c2.pos.x)
        };
    });

    for i in 0..carts.len() {
        let cart_next = move_cart(tracks, carts[i]);

        for j in 0..carts.len() {
            if j != i && carts[j].pos == cart_next.pos {
                return (carts, Some(cart_next.pos));
            }
        }
        carts[i] = cart_next;
    }

    (carts, None)
}

fn move_until_collision(tracks: &Vec<Vec<char>>, carts: &Vec<Cart>) -> Pos {
    let mut carts = carts.clone();
    loop {
        let (new_carts, collision) = move_carts_once(tracks, &carts);
        if let Some(pos) = collision {
            return pos;
        }

        carts = new_carts;
    }
}

fn read_tracks() -> Vec<Vec<char>> {
    stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part_one() {
    let tracks = read_tracks();

    let carts = find_carts(&tracks);
    let collision_pos = move_until_collision(&tracks, &carts);

    println!("{collision_pos}");
}

fn move_carts_once_and_remove_colliding(tracks: &Vec<Vec<char>>, carts: &Vec<Cart>) -> Vec<Cart> {
    let mut carts = carts.clone();
    carts.sort_by(|c1, c2| {
        let cmp_y = c1.pos.y.cmp(&c2.pos.y);
        return if cmp_y != std::cmp::Ordering::Equal {
            cmp_y
        } else {
            c1.pos.x.cmp(&c2.pos.x)
        };
    });

    let mut to_remove = HashSet::new();
    for i in 0..carts.len() {
        if !to_remove.contains(&i) {
            let cart_next = move_cart(tracks, carts[i]);

            for j in 0..carts.len() {
                if j != i && carts[j].pos == cart_next.pos {
                    to_remove.insert(j);
                    to_remove.insert(i);
                }
            }
            carts[i] = cart_next;
        }
    }

    let mut result = Vec::new();
    for i in 0..carts.len() {
        if !to_remove.contains(&i) {
            result.push(carts[i]);
        }
    }

    result
}

fn move_until_one_remains(tracks: &Vec<Vec<char>>, carts: &Vec<Cart>) -> Pos {
    let mut carts = carts.clone();

    while carts.len() > 1 {
        carts = move_carts_once_and_remove_colliding(tracks, &carts);
    }

    return carts[0].pos;
}

fn part_two() {
    let tracks = read_tracks();
    let carts = find_carts(&tracks);

    let result = move_until_one_remains(&tracks, &carts);
    println!("{result}");
}

fn main() {
    part_two();
}
