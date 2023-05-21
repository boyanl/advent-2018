use std::{
    collections::{HashMap, HashSet},
    io::stdin,
    vec,
};

use scanf::sscanf;
use OpCode::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn parse_opcode(s: &str) -> OpCode {
    match s {
        "addr" => Addr,
        "addi" => Addi,
        "mulr" => Mulr,
        "muli" => Muli,
        "banr" => Banr,
        "bani" => Bani,
        "borr" => Borr,
        "bori" => Bori,
        "setr" => Setr,
        "seti" => Seti,
        "gtir" => Gtir,
        "gtri" => Gtri,
        "gtrr" => Gtrr,
        "eqir" => Eqir,
        "eqri" => Eqri,
        "eqrr" => Eqrr,
        _ => todo!(),
    }
}

#[derive(Debug, Clone, Copy)]
struct Instr {
    code: OpCode,
    op1: i32,
    op2: i32,
    op3: i32,
}
fn do_instr(instr: Instr, rs: &mut Vec<i64>, current_cycle: usize, seen: &mut HashMap<i64, usize>) {
    let Instr {
        code,
        op1: a,
        op2: b,
        op3: c,
    } = instr;
    match code {
        Addr => rs[c as usize] = rs[a as usize] + rs[b as usize],
        Addi => rs[c as usize] = rs[a as usize] + b as i64,
        Mulr => rs[c as usize] = rs[a as usize] * rs[b as usize],
        Muli => rs[c as usize] = rs[a as usize] * b as i64,
        Banr => rs[c as usize] = rs[a as usize] & rs[b as usize],
        Bani => rs[c as usize] = rs[a as usize] & b as i64,
        Borr => rs[c as usize] = rs[a as usize] | rs[b as usize],
        Bori => rs[c as usize] = rs[a as usize] | b as i64,
        Setr => rs[c as usize] = rs[a as usize],
        Seti => rs[c as usize] = a as i64,
        Gtir => rs[c as usize] = if a as i64 > rs[b as usize] { 1 } else { 0 },
        Gtri => rs[c as usize] = if rs[a as usize] > b as i64 { 1 } else { 0 },
        Gtrr => {
            rs[c as usize] = if rs[a as usize] > rs[b as usize] {
                1
            } else {
                0
            }
        }
        Eqir => rs[c as usize] = if a as i64 == rs[b as usize] { 1 } else { 0 },
        Eqri => rs[c as usize] = if rs[a as usize] == b as i64 { 1 } else { 0 },
        Eqrr => {
            if a == 3 && b == 0 {
                if !seen.contains_key(&rs[3]) {
                    seen.insert(rs[3], current_cycle);
                }
            }
            rs[c as usize] = if rs[a as usize] == rs[b as usize] {
                1
            } else {
                0
            }
        }
    }
}

const REGISTERS_CNT: usize = 6;
fn execute(
    program: &Vec<Instr>,
    ip_reg: usize,
    start_state: &Vec<i64>,
    instructions_limit: Option<usize>,
) -> (Vec<i64>, usize, HashMap<i64, usize>) {
    let mut registers = start_state.clone();
    let mut ip = 0;

    let mut cycles = 0;
    let mut seen_for_reg_3 = HashMap::new();

    loop {
        if ip as usize >= program.len()
            || (instructions_limit.is_some() && cycles >= instructions_limit.unwrap())
        {
            break;
        }
        let instr = program[ip as usize];
        registers[ip_reg] = ip;

        do_instr(instr, &mut registers, cycles, &mut seen_for_reg_3);
        ip = registers[ip_reg];
        ip += 1;

        cycles += 1;
    }

    (registers, cycles, seen_for_reg_3)
}

fn read_program() -> (Vec<Instr>, usize) {
    let mut ip_register = 0;
    let mut instructions = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut ip_reg = 0;
        let (mut instr_type, mut op1, mut op2, mut op3) = (String::new(), 0, 0, 0);
        if sscanf!(&line, "ip {}", ip_reg).is_ok() {
            ip_register = ip_reg;
        } else if sscanf!(&line, "{} {} {} {}", instr_type, op1, op2, op3).is_ok() {
            let instr_type = parse_opcode(&instr_type);
            instructions.push(Instr {
                code: instr_type,
                op1: op1,
                op2: op2,
                op3: op3,
            });
        }
    }

    (instructions, ip_register as usize)
}

fn part_one() {
    let (program, ip_reg) = read_program();

    let mut state = vec![0; REGISTERS_CNT];
    let (_, _, seen_values) = execute(&program, ip_reg, &state, Some(1000000));

    let earliest_reg_3_value = seen_values
        .iter()
        .min_by_key(|(_, cycle)| **cycle)
        .unwrap()
        .0;

    println!("{earliest_reg_3_value}");
}

// Generates the last value of register 3 which is unique; translated from the puzzle input and optimized some manual division; way faster, obviously
fn reg_3_last_unique_value() -> i64 {
    let (mut r3, mut r4) = (0i64, 0i64);
    let mut last_r3 = r3;
    let mut skip = false;

    let m = 16777215;

    let mut seen = HashSet::new();

    loop {
        if !skip {
            r4 = r3 | 65536;
            r3 = 7041048;
        }
        r3 += r4 & 255;
        r3 &= m;
        r3 *= 65899;
        r3 &= m;

        if r4 < 256 {
            if seen.contains(&r3) {
                return last_r3;
            }
            seen.insert(r3);
            last_r3 = r3;
            skip = false;
        } else {
            r4 /= 256;
            skip = true;
        }
    }
}

fn part_two() {
    let latest_reg_3_value = reg_3_last_unique_value();

    // Also works, but is a lot slower
    // let (program, ip_reg) = read_program();

    // let mut state = vec![0; REGISTERS_CNT];
    // let (_, _, seen_values) = execute(&program, ip_reg, &state, Some(6_000_000_000));

    // let latest_reg_3_value = seen_values
    //     .iter()
    //     .max_by_key(|(_, cycle)| **cycle)
    //     .unwrap()
    //     .0;

    println!("{latest_reg_3_value}");
}

fn main() {
    part_two();
}
