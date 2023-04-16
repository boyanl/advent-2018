use scanf::sscanf;
use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    thread::current,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Interval {
    min: i32,
    max: i32,
}

fn len(i: Interval) -> i32 {
    return i.max - i.min;
}

impl Interval {
    fn contains(self, x: i32) -> bool {
        return self.min <= x && x < self.max;
    }
}

fn find_guard_most_asleep(guard_sleep_intervals: &HashMap<i32, Vec<Interval>>) -> (i32, i32) {
    let guard = *guard_sleep_intervals
        .keys()
        .max_by_key(|k| {
            guard_sleep_intervals[k]
                .iter()
                .map(|&interval| len(interval))
                .sum::<i32>()
        })
        .unwrap();

    let minute = (0..60)
        .max_by_key(|minute| {
            guard_sleep_intervals[&guard]
                .iter()
                .filter(|&interval| interval.contains(*minute))
                .count()
        })
        .unwrap();

    return (guard, minute);
}

fn read_guards_data() -> HashMap<i32, Vec<Interval>> {
    let mut current_guard = 0;
    let (mut start, mut end) = (0, 0);
    let mut guard_sleep_intervals = HashMap::new();
    let mut entries = stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    entries.sort();

    for line in entries {
        let (mut date, mut time) = (String::new(), String::new());
        let mut n = 0;
        let mut minute = 0;

        if sscanf!(&line, "[{} {}] Guard #{} begins shift", date, time, n).is_ok() {
            current_guard = n;
        } else if sscanf!(&line, "[{} 00:{}] falls asleep", date, minute).is_ok() {
            start = minute;
        } else if sscanf!(&line, "[{} 00:{}] wakes up", date, minute).is_ok() {
            end = minute;
            guard_sleep_intervals
                .entry(current_guard)
                .or_insert(Vec::new())
                .push(Interval {
                    min: start,
                    max: end,
                });
        }
    }

    return guard_sleep_intervals;
}

fn part_one() {
    let guards_data = read_guards_data();
    let (guard, most_slept_minute) = find_guard_most_asleep(&guards_data);
    let result = guard * most_slept_minute;
    println!("{result}");
}

fn times_asleep_at_minute(intervals: &Vec<Interval>, minute: i32) -> usize {
    return intervals
        .iter()
        .filter(|&interval| interval.contains(minute))
        .count();
}

fn most_times_asleep_any_minute(intervals: &Vec<Interval>) -> usize {
    return (0..60)
        .map(|minute| times_asleep_at_minute(intervals, minute))
        .max()
        .unwrap();
}

fn find_guard_most_asleep_on_same_minute(
    guard_sleep_intervals: &HashMap<i32, Vec<Interval>>,
) -> (i32, i32) {
    let guard = *guard_sleep_intervals
        .keys()
        .max_by_key(|&guard| most_times_asleep_any_minute(&guard_sleep_intervals[guard]))
        .unwrap();

    let minute = (0..60)
        .max_by_key(|minute| times_asleep_at_minute(&guard_sleep_intervals[&guard], *minute))
        .unwrap();

    return (guard, minute);
}

fn part_two() {
    let guards_data = read_guards_data();
    let (guard, most_frequently_slept_minute) = find_guard_most_asleep_on_same_minute(&guards_data);
    let result = guard * most_frequently_slept_minute;
    println!("{result}");
}

fn main() {
    part_two();
}
