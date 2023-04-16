use std::io::stdin;

fn codes_to_str(c: &Vec<u8>) -> &str {
    return std::str::from_utf8(&c[0..c.len()]).unwrap();
}

fn simulate_reactions(start: &str) -> String {
    let mut codes = start.chars().map(|c| c as u8).collect::<Vec<_>>();

    loop {
        let mut to_remove = Vec::new();
        let mut i = 0;
        while i < (codes.len() as i32) - 1 {
            let (c1, c2) = (codes[i as usize] as i32, codes[(i + 1) as usize] as i32);
            if (c1 - c2).abs() == 32 {
                to_remove.push(i);
                to_remove.push(i + 1);
                i += 1;
            }
            i += 1;
        }

        if to_remove.is_empty() {
            break;
        }
        for i in (0..to_remove.len()).rev() {
            codes.remove(to_remove[i] as usize);
        }
    }

    return codes_to_str(&codes).to_string();
}

fn part_one() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("string expected");
    let polymer = &input[0..input.len() - 1];

    let result = simulate_reactions(&polymer);
    println!("{}", result.len());
}

fn part_two() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("string expected");
    let polymer = &input[0..input.len() - 1];

    let result = ('A'..='Z')
        .map(|c| polymer.replace(&[c, c.to_ascii_lowercase()], ""))
        .map(|polymer| simulate_reactions(&polymer).len())
        .min()
        .unwrap();

    println!("{result}");
}

fn main() {
    part_two();
}
