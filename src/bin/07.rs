use binary_heap_plus::BinaryHeap;
use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

use scanf::sscanf;

fn find_roots(steps: &Steps) -> Vec<String> {
    let with_parent: HashSet<String> = steps.parents_map.keys().cloned().collect();
    return steps
        .all_steps
        .difference(&with_parent)
        .map(|k| k.clone())
        .collect::<Vec<_>>();
}

fn traverse(steps: &Steps, roots: &Vec<String>) -> String {
    let mut q = BinaryHeap::new_min();
    for root in roots {
        q.push(root);
    }
    let mut completed = HashSet::new();
    let mut result = String::new();

    while !q.is_empty() {
        let current = q.pop().unwrap();
        result.push_str(current);
        completed.insert(current);

        if steps.next_map.contains_key(current) {
            for next in &steps.next_map[current] {
                let all_completed = steps.parents_map[next]
                    .iter()
                    .all(|k| completed.contains(k));
                if all_completed {
                    q.push(next);
                }
            }
        }
    }

    return result;
}

struct Steps {
    parents_map: HashMap<String, Vec<String>>,
    next_map: HashMap<String, Vec<String>>,
    all_steps: HashSet<String>,
}

fn read_input() -> Steps {
    let mut parent: HashMap<String, Vec<String>> = HashMap::new();
    let mut next: HashMap<String, Vec<String>> = HashMap::new();
    let mut steps = HashSet::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut from, mut to) = (String::new(), String::new());
        if sscanf!(
            &line,
            "Step {} must be finished before step {} can begin.",
            from,
            to
        )
        .is_ok()
        {
            parent
                .entry(to.clone())
                .or_insert(Vec::new())
                .push(from.clone());
            steps.insert(from.clone());
            steps.insert(to.clone());
            next.entry(from).or_insert(Vec::new()).push(to);
        }
    }

    return Steps {
        parents_map: parent,
        next_map: next,
        all_steps: steps,
    };
}

fn part_one() {
    let steps = read_input();
    let roots = find_roots(&steps);

    let traversal_string = traverse(&steps, &roots);
    println!("{traversal_string}");
}

fn completion_time(steps: &Steps, num_workers: u8) -> i32 {
    let roots = find_roots(steps);

    let mut q = BinaryHeap::new_min();
    for root in &roots {
        q.push((0, root));
    }
    let mut completed = HashMap::new();
    let mut workers_time = BinaryHeap::new_min();
    for _ in 0..num_workers {
        workers_time.push(0);
    }

    while !q.is_empty() {
        let (parents_t, current) = q.pop().unwrap();

        let free_worker_t = max(workers_time.pop().unwrap(), parents_t);
        let work = current.chars().nth(0).unwrap();
        let cost = (work as i32) - ('A' as i32) + 1 + 60;
        let end = free_worker_t + cost;

        workers_time.push(end);
        completed.insert(current, end);

        if steps.next_map.contains_key(current) {
            for next in &steps.next_map[current] {
                let all_completed = steps.parents_map[next]
                    .iter()
                    .all(|k| completed.contains_key(k));
                if all_completed {
                    q.push((end, next));
                }
            }
        }
    }

    let mut result = 0;
    while !workers_time.is_empty() {
        result = workers_time.pop().unwrap();
    }

    return result;
}

fn part_two() {
    let steps = read_input();
    let result = completion_time(&steps, 5);

    println!("{result}");
}

fn main() {
    part_two()
}
