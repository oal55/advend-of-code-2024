use std::{collections::VecDeque, io::BufRead, iter};
use crate::common::io::file_reader;

#[derive(Clone, Copy, Debug)]
struct State {
    a: i64,
    b: i64,
    c: i64,
}
impl State {
    fn combo(&self, arg: i64) -> i64 {
        match arg {
            0..=3 => arg,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Bad combo: {arg}")
        }
    }
}

#[derive(Debug)]
struct Instruction {id: i64, arg: i64}

pub fn run(file_path: &str) -> (String, String) {
    let (mut state, raw_instructions) = parse_inputs(file_path);
    let instructions = raw_instructions.chunks(2)
        .map(|c|  Instruction{id: c[0], arg: c[1]})
        .collect::<Vec<_>>();

    let part1 = run_program(&mut state, &instructions).iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    
    let good_a = find_good_a(&instructions, &raw_instructions);

    (part1, good_a.to_string())
}

fn find_good_a(instructions: &[Instruction], target_out: &[i64]) -> i64 {
    let mut search_q: VecDeque<(i64, i64)> = VecDeque::from_iter(iter::once((0, 0)));
    while let Some((a_prev, i_octal)) = search_q.pop_front() {
        for i in 0..8 {
            let a = (a_prev << 3) + i;
            let cur_output = run_program(&mut State{a, b:0, c:0}, &instructions);
            let subset_out = &target_out[(target_out.len() - i_octal as usize - 1)..];
            if cur_output == subset_out {
                if cur_output.len() == target_out.len() {
                    return a;
                }
                search_q.push_back((a, i_octal + 1));
            }
        }
    }
    panic!("I cry.");
}

fn run_program(state: &mut State, instructions: &[Instruction]) -> Vec<i64> {
    let mut outputs = Vec::new();
    let mut i_pointer = 0;
    while i_pointer < instructions.len() {
        let Instruction{id, arg} = instructions[i_pointer];
        i_pointer += 1;
        match id {
            0 => state.a /= 1 << state.combo(arg),
            1 => state.b ^= arg,
            2 => state.b = state.combo(arg) % 8,
            3 => {
                if state.a != 0 {
                    i_pointer = arg as usize;
                }
            },
            4 => state.b ^= state.c,
            5 => outputs.push(state.combo(arg) % 8),
            6 => state.b = state.a / (1 << state.combo(arg)),
            7 => state.c = state.a / (1 << state.combo(arg)),
            _ => panic!("Bad instruction: id: {id}, arg: {arg}")
        }
    }
    outputs
}

fn parse_inputs(file_path: &str) -> (State, Vec<i64>) {
    let mut lines = file_reader(file_path).lines();
    let a = extract_register_value(&lines.next().unwrap().unwrap());
    let b = extract_register_value(&lines.next().unwrap().unwrap());
    let c = extract_register_value(&lines.next().unwrap().unwrap());
    
    lines.next().unwrap().unwrap();
    let program = lines.next().unwrap().unwrap();
    let (_, raw_instructions) = program.split_once(": ").unwrap();

    let state = State{a, b, c};
    let instructions: Vec<i64> = raw_instructions.split(",")
        .map(|i| i.parse::<i64>().unwrap_or_else(|_| panic!("Cannot parse: {i}")))
        .collect::<Vec<_>>();
    
    (state, instructions)
}

fn extract_register_value(line: &str) -> i64 {
    let (_, a_register) = line.split_once(": ").unwrap();
    a_register.parse().unwrap()
}

/*

1 -> b = a % 8;     // b has a[-3:]
2 -> b ^= 5         // flip 101
3 -> c = a >> b;
4 -> b ^= 6         // flip 100, -> b = a[-3:] ^ 1
5 -> a >>= 3        // get rid of a[-3:]
6 -> b ^= c
7 -> print(b % 8)
if (a >0) { go to 1; }
 */
