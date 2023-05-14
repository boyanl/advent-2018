use std::{
    any::Any,
    collections::{btree_map::VacantEntry, hash_map::OccupiedEntry, HashMap},
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

fn do_instr(instr: Instr, rs: &mut Vec<i64>) {
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
            rs[c as usize] = if rs[a as usize] == rs[b as usize] {
                1
            } else {
                0
            }
        }
    }
}

fn eq_excluding(v1: &Vec<i64>, v2: &Vec<i64>, idx: usize) -> bool {
    if v1.len() != v2.len() {
        return false;
    }

    for i in 0..v1.len() {
        if i != idx && v1[i] != v2[i] {
            return false;
        }
    }

    true
}

const REGISTERS_CNT: usize = 6;
fn execute(
    program: &Vec<Instr>,
    ip_reg: usize,
    start_state: &Vec<i64>,
    instructions_limit: Option<usize>,
) -> Vec<i64> {
    let mut registers = start_state.clone();
    let mut ip = 0;

    let mut cycles = 0;

    loop {
        if ip as usize >= program.len()
            || (instructions_limit.is_some() && cycles >= instructions_limit.unwrap())
        {
            break;
        }
        let instr = program[ip as usize];
        registers[ip_reg] = ip;

        do_instr(instr, &mut registers);
        ip = registers[ip_reg];
        ip += 1;

        cycles += 1;
    }

    registers
}

fn read_input() -> (Vec<Instr>, usize) {
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
    let (instructions, ip_register) = read_input();
    let final_state = execute(&instructions, ip_register, &vec![0; REGISTERS_CNT], None);

    println!("{}", final_state[0]);
}

fn sum_divisors(n: u64) -> u64 {
    let mut result = 0;
    for i in 1..=n {
        if n % i == 0 {
            result += i;
        }
    }

    result
}

fn part_two() {
    let (instructions, ip_register) = read_input();
    let mut start_state = vec![0; REGISTERS_CNT];
    start_state[0] = 1;
    let parameters = execute(&instructions, ip_register, &start_state, Some(50));

    // Reverse-engineered what the program does and wrote a function that computes it faster ..
    let n = parameters[4];
    let result = sum_divisors(n as u64);
    println!("{result}");
}

fn main() {
    part_two();
}
