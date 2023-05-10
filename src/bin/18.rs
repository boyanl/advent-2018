use std::{collections::HashMap, io::stdin};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Pos {
    y: i32,
    x: i32,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum AcreState {
    TREES,
    LUMBERYARD,
    EMPTY,
}

type State = Vec<Vec<AcreState>>;

fn neighbours(state: &State, x: usize, y: usize) -> Vec<AcreState> {
    let mut result = Vec::new();
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if ny >= 0
                    && (ny as usize) < state.len()
                    && nx >= 0
                    && (nx as usize) < state[ny as usize].len()
                {
                    result.push(state[ny as usize][nx as usize]);
                }
            }
        }
    }

    result
}

fn next(state: &State) -> State {
    let mut result = vec![vec![AcreState::EMPTY; state[0].len()]; state.len()];

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            let s = state[y][x];
            result[y][x] = s;

            let ns = neighbours(state, x, y);
            if s == AcreState::EMPTY {
                if ns.iter().filter(|&&v| v == AcreState::TREES).count() >= 3 {
                    result[y][x] = AcreState::TREES;
                }
            } else if s == AcreState::TREES {
                if ns.iter().filter(|&&v| v == AcreState::LUMBERYARD).count() >= 3 {
                    result[y][x] = AcreState::LUMBERYARD;
                }
            } else {
                let lumberyards = ns.iter().filter(|&&v| v == AcreState::LUMBERYARD).count();
                let trees = ns.iter().filter(|&&v| v == AcreState::TREES).count();

                if lumberyards < 1 || trees < 1 {
                    result[y][x] = AcreState::EMPTY;
                }
            }
        }
    }

    result
}

fn count(s: &State, val: AcreState) -> usize {
    let mut result = 0;
    for row in s {
        for &el in row {
            if el == val {
                result += 1;
            }
        }
    }

    result
}

fn read_state() -> State {
    let mut state = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut row = Vec::new();
        for c in line.chars() {
            let s = match c {
                '|' => AcreState::TREES,
                '#' => AcreState::LUMBERYARD,
                _ => AcreState::EMPTY,
            };
            row.push(s);
        }
        state.push(row);
    }

    state
}

fn resource_value(s: &State) -> usize {
    count(s, AcreState::LUMBERYARD) * count(s, AcreState::TREES)
}

fn part_one() {
    let mut state = read_state();

    let rounds = 10;
    for _ in 1..=rounds {
        state = next(&state);
    }

    println!("{}", resource_value(&state));
}

fn part_two() {
    let mut state = read_state();

    let rounds = 1_000_000_000;

    let mut seen: HashMap<State, usize> = HashMap::new();
    seen.insert(state.clone(), 0);

    let mut r = 1;
    while r <= rounds {
        state = next(&state);

        if seen.contains_key(&state) {
            let cycle_len = r - seen[&state];
            let times = (rounds - r) / cycle_len;

            r += times * cycle_len;
        }
        seen.insert(state.clone(), r);

        r += 1;
    }

    let (lumberyards, trees) = (
        count(&state, AcreState::LUMBERYARD),
        count(&state, AcreState::TREES),
    );
    println!("{}", lumberyards * trees);
}

fn main() {
    part_two();
}
