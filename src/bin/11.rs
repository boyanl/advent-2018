use std::vec;

fn fuel_value(x: i32, y: i32, serial_number: i32) -> i32 {
    let (x, y, serial_number) = (x as i64, y as i64, serial_number as i64);
    let rack_id = x + 10;
    let power1 = (rack_id * y + serial_number) * rack_id;
    let hundreds = ((power1 % 1000) / 100) as i32;
    return hundreds - 5;
}

fn region_fuel_sum(x: i32, y: i32, side: usize, n: usize, serial_number: i32) -> i32 {
    let mut sum = 0;
    for dx in 0..side {
        for dy in 0..side {
            sum += fuel_value(x + dx as i32, y + dy as i32, serial_number);
        }
    }

    sum
}

fn biggest_3x3_fuel_region_top(N: usize, serial_number: i32) -> (usize, usize) {
    let mut result = i32::MIN;
    let mut top = (0, 0);
    for x in 1..=N {
        for y in 1..=N {
            let sum = region_fuel_sum(x as i32, y as i32, 3, N, serial_number);
            if sum > result {
                result = sum;
                top = (x, y);
            }
        }
    }

    top
}

fn part_one() {
    let serial_number = 7403;
    let N = 300;

    let result = biggest_3x3_fuel_region_top(N, serial_number);
    println!("{:?}", result);
}

fn fuel_values(n: usize, serial_number: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for y in 1..=n {
        let line = (1..=n)
            .map(|x| fuel_value(x as i32, y as i32, serial_number))
            .collect::<Vec<_>>();
        result.push(line);
    }

    result
}

fn biggest_fuel_region(fuel_field: Vec<Vec<i32>>) -> (usize, usize, usize) {
    let mut result = i32::MIN;
    let mut top = (0, 0);
    let mut side = 0;
    let n = fuel_field.len();

    let mut partial_sums = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        partial_sums[0][i] = 0;
        partial_sums[i][0] = 0;
    }
    partial_sums[1][1] = fuel_field[0][0];
    for x in 1..n {
        partial_sums[1][x + 1] = partial_sums[1][x] + fuel_field[0][x];
    }

    for y in 1..n {
        partial_sums[y + 1][1] = partial_sums[y][1] + fuel_field[y][0];
    }

    for y in 1..n {
        for x in 1..n {
            partial_sums[y + 1][x + 1] =
                fuel_field[y][x] + partial_sums[y][x + 1] + partial_sums[y + 1][x]
                    - partial_sums[y][x];
        }
    }

    for size in 2..=n {
        for x in 1..=n - size + 1 {
            for y in 1..=n - size + 1 {
                let dsize = size - 1;
                let sum = partial_sums[y + dsize][x + dsize]
                    - partial_sums[y + dsize][x - 1]
                    - partial_sums[y - 1][x + dsize]
                    + partial_sums[y - 1][x - 1];
                if sum > result {
                    result = sum;
                    top = (x, y);
                    side = size;
                }
            }
        }
    }

    (top.0, top.1, side)
}

fn part_two() {
    let serial_number = 7403;
    let N = 300;
    let fuel_field = fuel_values(N, serial_number);

    let result = biggest_fuel_region(fuel_field);
    println!("{:?}", result)
}

fn main() {
    part_two()
}
