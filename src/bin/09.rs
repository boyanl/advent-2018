use std::{collections::VecDeque, io::stdin};

use scanf::sscanf;

fn marbles_high_score(players: usize, marbles_cnt: usize) -> i64 {
    let mut marbles = VecDeque::new();
    marbles.push_back(0);

    let mut current_idx = 0;
    let mut scores = vec![0i64; players];

    for i in 1..marbles_cnt {
        let player = (i - 1) % players;
        if i % 23 == 0 {
            scores[player] += i as i64;

            let to_remove_idx =
                (((current_idx as i32) - 7 + marbles.len() as i32) as usize) % marbles.len();
            let el = marbles[to_remove_idx];
            scores[player] += el as i64;
            marbles.remove(to_remove_idx);

            current_idx = to_remove_idx;
            let target = marbles.len() - 2;
            if target >= current_idx {
                marbles.rotate_right(target - current_idx);
            } else {
                marbles.rotate_left(current_idx - target);
            }
        } else {
            if i <= 2 {
                let idx = (current_idx + 2) % marbles.len();
                marbles.insert(idx, i);
                current_idx = idx;

                if i == 2 {
                    marbles.rotate_right(1);
                    current_idx = 1;
                }
            } else {
                marbles.push_back(i);
                marbles.rotate_left(1);
                current_idx = marbles.len() - 2;
            }
        }
    }

    return *scores.iter().max().unwrap() as i64;
}

fn part_one() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut players, mut marbles) = (0, 0);
        if sscanf!(
            &line,
            "{} players; last marble is worth {} points",
            players,
            marbles
        )
        .is_ok()
        {
            let score = marbles_high_score(players, marbles);
            println!("{score}");
        }
    }
}

fn part_two() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut players, mut marbles) = (0, 0);
        if sscanf!(
            &line,
            "{} players; last marble is worth {} points",
            players,
            marbles
        )
        .is_ok()
        {
            let score = marbles_high_score(players, marbles * 100);
            println!("{score}");
        }
    }
}

fn main() {
    part_one()
}
