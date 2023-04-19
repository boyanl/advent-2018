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

fn region_fuel_sum_2(fuel_field: &Vec<Vec<i32>>, x: usize, y: usize, side: usize) -> i32 {
    let mut sum = 0;
    for dy in 0..side {
        for dx in 0..side {
            sum += fuel_field[y + dy - 1][x + dx - 1];
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

    for size in 2..=n {
        for x in 1..=n - size + 1 {
            for y in 1..=n - size + 1 {
                let sum = region_fuel_sum_2(&fuel_field, x, y, size);
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
