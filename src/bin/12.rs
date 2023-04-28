use std::{
    collections::HashMap,
    io::{stdin, Read},
};

use scanf::sscanf;

fn read_input() -> (String, HashMap<String, String>) {
    let mut initial = String::new();
    let mut rules = HashMap::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if sscanf!(&line, "initial state: {}", initial).is_ok() {}

        if line.is_empty() {
            continue;
        }

        let (mut from, mut to) = (String::new(), String::new());
        if sscanf!(&line, "{} => {}", from, to).is_ok() {
            rules.insert(from, to);
        }
    }

    (initial, rules)
}

fn around(i: i32, state: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    for di in -2..=2 {
        if i + di < 0 || i + di >= (state.len() as i32) {
            result.push('.' as u8)
        } else {
            result.push(state[(i + di) as usize]);
        }
    }

    result
}

fn simulate(initial: &str, rules: &HashMap<String, String>, rounds: u64) -> (String, i64) {
    let mut state = initial.as_bytes().iter().map(|&u| u).collect::<Vec<_>>();
    let mut start = 0i64;
    let mut seen = HashMap::new();

    seen.insert(state.clone(), (0, 0i64));

    let mut r = 1;
    while r <= rounds {
        let mut new_state = Vec::new();
        let mut new_start = i64::MIN;

        for i in -2..(state.len() + 2) as i32 {
            let ctx = String::from_utf8(around(i, &state)).unwrap();
            let b_i: u8;
            if rules.contains_key(&ctx) {
                b_i = rules[&ctx].as_bytes()[0];
            } else {
                b_i = '.' as u8;
            }

            if b_i == '#' as u8 && new_start == i64::MIN {
                new_start = start + i as i64;
            }

            // Trim from the left side when constructing ..
            if b_i == '#' as u8 || !new_state.is_empty() {
                new_state.push(b_i);
            }
        }

        while *new_state.last().unwrap() == '.' as u8 {
            new_state.pop();
        }

        if seen.contains_key(&new_state) {
            let (round_seen, old_start) = seen[&new_state];
            let len = r - round_seen;
            let repeats = (rounds - r) / len;
            let start_offset = (new_start - old_start) * (repeats as i64);

            let remaining = (rounds - r) % len;

            r = rounds - remaining + 1;
            new_start = new_start + start_offset;
        } else {
            seen.insert(new_state.clone(), (r, new_start));
            r += 1;
        }

        state = new_state;
        start = new_start;
    }

    (String::from_utf8(state).unwrap(), start)
}

fn count_score(final_state: &str, start: i64) -> i64 {
    let mut result = 0;
    for (i, c) in final_state.chars().enumerate() {
        if c == '#' {
            result += start + i as i64;
        }
    }

    result
}

fn part_one() {
    let (initial, rules) = read_input();

    let (final_state, start) = simulate(&initial, &rules, 20);
    let result = count_score(&final_state, start);

    println!("{result}");
}

fn part_two() {
    let (initial, rules) = read_input();

    let (final_state, start) = simulate(&initial, &rules, 50_000_000_000);
    let result = count_score(&final_state, start);

    println!("{result}");
}

fn main() {
    part_two();
}
