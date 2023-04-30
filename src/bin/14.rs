use std::io::stdin;

fn get_recipes_after(n: usize) -> Vec<usize> {
    const additional: usize = 10;

    let mut recipes = Vec::from([3, 7]);
    let (mut e1, mut e2) = (0, 1);

    while recipes.len() < n + additional {
        let next = recipes[e1] + recipes[e2];
        if next < 10 {
            recipes.push(next);
        } else {
            recipes.push(1);
            recipes.push(next % 10);
        }
        e1 = (e1 + recipes[e1] + 1) % recipes.len();
        e2 = (e2 + recipes[e2] + 1) % recipes.len();
    }

    return recipes[n..n + additional].to_vec();
}

fn part_one() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let N = line
            .parse::<usize>()
            .expect("should read number of recipes");

        let result = get_recipes_after(N);
        let stringified = result
            .iter()
            .map(|x| x.to_string())
            .fold(String::new(), |a, b| a + b.as_str());
        println!("{stringified}");
    }
}

fn digits(n: usize) -> Vec<usize> {
    let mut n = n;
    let mut result = Vec::new();

    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }

    result.reverse();
    return result;
}

fn find_first_occurrence_of(digits: &[usize]) -> usize {
    let mut recipes = Vec::from([3, 7]);
    let (mut e1, mut e2) = (0, 1);

    loop {
        let next = recipes[e1] + recipes[e2];
        if next < 10 {
            recipes.push(next);
        } else {
            recipes.push(1);
            recipes.push(next % 10);
        }

        if recipes.len() >= digits.len() {
            for i in 0..=1 {
                if recipes.len() >= digits.len() + i
                    && recipes[(recipes.len() - digits.len() - i)..(recipes.len() - i)] == *digits
                {
                    return recipes.len() - digits.len() - i;
                }
            }
        }
        e1 = (e1 + recipes[e1] + 1) % recipes.len();
        e2 = (e2 + recipes[e2] + 1) % recipes.len();
    }
}

fn part_two() {
    for line in stdin().lines().map(|l| l.unwrap()) {
        let N = line
            .parse::<usize>()
            .expect("should read number of recipes");

        let digits = digits(N);
        let result = find_first_occurrence_of(&digits);
        println!("{result}");
    }
}

fn main() {
    part_two();
}
