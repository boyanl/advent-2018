use std::{collections::HashMap, hash::Hash, io::stdin};

fn char_freqs(s: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();
    for c in s.chars() {
        result.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    return result;
}

fn has_letter_with_freq(map: &HashMap<char, i32>, n: i32) -> bool {
    for (_, &v) in map {
        if v == n {
            return true;
        }
    }
    return false;
}

fn part_one() {
    let box_ids = stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let frequencies = box_ids.iter().map(|l| char_freqs(l)).collect::<Vec<_>>();

    let letter_twice = frequencies
        .iter()
        .filter(|&f| has_letter_with_freq(f, 2))
        .count();
    let letter_thrice = frequencies
        .iter()
        .filter(|&f| has_letter_with_freq(f, 3))
        .count();

    let result = letter_twice * letter_thrice;
    println!("{result}");
}

fn diff(s1: &str, s2: &str) -> Option<(usize, usize)> {
    let mut cnt_diff = 0;
    let mut last_diff_idx = 0;
    for i in 0..s1.len() {
        if s1.chars().nth(i) != s2.chars().nth(i) {
            cnt_diff += 1;
            last_diff_idx = i;
        }
    }
    return if cnt_diff == 0 {
        None
    } else {
        Some((cnt_diff, last_diff_idx))
    };
}

fn part_two() {
    let box_ids = stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    'outer: for id1 in &box_ids {
        for id2 in &box_ids {
            if let Some((1, pos)) = diff(id1, id2) {
                let mut answer = id1[..pos].to_string();
                answer.push_str(&id1[pos + 1..]);

                println!("{answer}");
                break 'outer;
            }
        }
    }
}

fn main() {
    part_two();
}
