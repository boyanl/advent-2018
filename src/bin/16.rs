use std::{collections::HashMap, collections::HashSet, io::stdin};

use scanf::sscanf;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

impl OpCode {
    pub fn iterator() -> Iter<'static, OpCode> {
        static opcodes: [OpCode; 16] = [
            OpCode::Addr,
            OpCode::Addi,
            OpCode::Mulr,
            OpCode::Muli,
            OpCode::Banr,
            OpCode::Bani,
            OpCode::Borr,
            OpCode::Bori,
            OpCode::Setr,
            OpCode::Seti,
            OpCode::Gtir,
            OpCode::Gtri,
            OpCode::Gtrr,
            OpCode::Eqir,
            OpCode::Eqri,
            OpCode::Eqrr,
        ];
        return opcodes.iter();
    }
}

const registers_cnt: usize = 4;
type State = [i32; registers_cnt];

fn apply(state: State, opcode: OpCode, a: i32, b: i32, c: i32) -> State {
    let mut result = state;
    let (asz, bsz, csz) = (a as usize, b as usize, c as usize);
    match opcode {
        OpCode::Addr => result[csz] = result[asz] + result[bsz],
        OpCode::Addi => result[csz] = result[asz] + b,
        OpCode::Mulr => result[csz] = result[asz] * result[bsz],
        OpCode::Muli => result[csz] = result[asz] * b,
        OpCode::Banr => result[csz] = result[asz] & result[bsz],
        OpCode::Bani => result[csz] = result[asz] & b,
        OpCode::Borr => result[csz] = result[asz] | result[bsz],
        OpCode::Bori => result[csz] = result[asz] | b,
        OpCode::Setr => result[csz] = result[asz],
        OpCode::Seti => result[csz] = a,
        OpCode::Gtir => result[csz] = if a > result[bsz] { 1 } else { 0 },
        OpCode::Gtri => result[csz] = if result[asz] > b { 1 } else { 0 },
        OpCode::Gtrr => result[csz] = if result[asz] > result[bsz] { 1 } else { 0 },
        OpCode::Eqir => result[csz] = if a == result[bsz] { 1 } else { 0 },
        OpCode::Eqri => result[csz] = if result[asz] == b { 1 } else { 0 },
        OpCode::Eqrr => result[csz] = if result[asz] == result[bsz] { 1 } else { 0 },
    }

    result
}

type EncodedInstr = [i32; 4];

#[derive(Debug, Clone, Copy)]
struct BeforeAfterPair {
    before: State,
    instr: EncodedInstr,
    after: State,
}

fn parse_numbers(s: &str) -> Vec<i32> {
    s.replace("[", "")
        .replace("]", "")
        .replace(",", "")
        .split_ascii_whitespace()
        .map(|part| part.parse::<i32>().expect("should be able to parse int"))
        .collect::<Vec<_>>()
}

fn vec_to_state(v: Vec<i32>) -> State {
    return v.try_into().unwrap();
}

struct Input {
    pairs: Vec<BeforeAfterPair>,
    program: Vec<EncodedInstr>,
}

// TODO: Add program
fn read_input() -> Input {
    let mut pairs = Vec::new();
    let mut instructions = Vec::new();

    let lines = stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut i = 0;
    while i < lines.len() {
        let mut before = String::new();
        if sscanf!(&lines[i], "Before: {}", before).is_ok() {
            i += 1;

            let mut instr = String::new();
            sscanf!(&lines[i], "{}", instr).expect("should be able to read instr");

            i += 1;
            let mut after = String::new();
            sscanf!(&lines[i], "After: {}", after).expect("should read after state");

            let (before, instr, after) = (
                vec_to_state(parse_numbers(before.as_str())),
                parse_numbers(instr.as_str()),
                vec_to_state(parse_numbers(after.as_str())),
            );
            pairs.push(BeforeAfterPair {
                before: before,
                instr: instr
                    .try_into()
                    .unwrap_or_else(|v: Vec<i32>| panic!("Vec has unsuitable length: {}", v.len())),
                after: after,
            });
        } else if !lines[i].is_empty() {
            let mut instr_string = String::new();
            sscanf!(&lines[i], "{}", instr_string).expect("should read instructions");

            let instr: EncodedInstr = parse_numbers(instr_string.as_str()).try_into().unwrap();
            instructions.push(instr);
        }

        i += 1;
    }

    Input {
        pairs: pairs,
        program: instructions,
    }
}

fn possible_opcodes(before_after: BeforeAfterPair) -> Vec<OpCode> {
    let mut result = Vec::new();
    let (a, b, c) = (
        before_after.instr[1],
        before_after.instr[2],
        before_after.instr[3],
    );

    for opcode in OpCode::iterator() {
        let new_state = apply(before_after.before, *opcode, a, b, c);
        if new_state == before_after.after {
            result.push(*opcode);
        }
    }

    result
}

fn part_one() {
    let input = read_input();

    let result = input
        .pairs
        .iter()
        .filter(|&before_after| possible_opcodes(*before_after).len() >= 3)
        .count();

    println!("{result}");
}

fn part_two() {
    let input = read_input();
    let mut mapping: HashMap<i32, HashSet<OpCode>> = HashMap::new();
    let all_opcodes = OpCode::iterator().map(|&v| v).collect::<HashSet<_>>();

    for pair in input.pairs {
        let p = possible_opcodes(pair);
        let instr_code = pair.instr[0];

        mapping
            .entry(instr_code)
            .or_insert(all_opcodes.clone())
            .retain(|item| p.contains(item));
    }

    let mut did_elimination: HashSet<i32> = HashSet::new();

    loop {
        let determined = mapping
            .iter()
            .filter(|(&k, v)| v.len() == 1 && !did_elimination.contains(&k))
            .map(|(&u, v)| (u, v.clone()))
            .next();

        if determined.is_none() {
            break;
        }
        if let Some((k, options)) = determined {
            let option = options.iter().next().unwrap();
            let keys = mapping.keys().map(|&u| u).collect::<Vec<_>>();
            for other in keys {
                if other != k {
                    mapping.entry(other).and_modify(|v| {
                        v.remove(option);
                    });
                }
            }

            did_elimination.insert(k);
        }
    }

    let mut state = [0; registers_cnt];
    for instr in input.program {
        let opcode = *mapping[&instr[0]].iter().next().unwrap();
        state = apply(state, opcode, instr[1], instr[2], instr[3]);
    }

    println!("{}", state[0]);
}

fn main() {
    part_two();
}
