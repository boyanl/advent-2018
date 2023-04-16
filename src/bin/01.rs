use std::{collections::HashSet, io::stdin};

fn part_one() {
    let answer: i32 = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let (op, &amount) = (&line[..1], &line[1..].parse::<i32>().unwrap());
            return match op {
                "+" => amount,
                "-" => -amount,
                _ => todo!("unexpected op"),
            };
        })
        .sum();

    println!("{answer}");
}

fn part_two() {
    let input_instrs = stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut current = 0;
    let mut seen = HashSet::new();

    'outer: loop {
        for line in &input_instrs {
            let (op, &amount) = (&line[..1], &line[1..].parse::<i32>().unwrap());
            current += match op {
                "+" => amount,
                "-" => -amount,
                _ => todo!("unexpected op"),
            };
            if seen.contains(&current) {
                println!("{current}");
                break 'outer;
            }
            seen.insert(current);
        }
    }
}

fn main() {
    part_two();
}
