use std::io::BufRead;
use crate::common::io::file_reader;

struct State {
    a: i32,
    b: i32,
    c: i32,
    outputs: Vec<String>,
}
struct Instruction {id: i32, arg: i32}

pub fn run(file_path: &str) -> (String, String) {
    let (mut state, instructions) = parse_inputs(file_path);

    let mut i_instr = 0;
    while i_instr < instructions.len() {
        i_instr = execute_instruction(&mut state, &instructions[i_instr], i_instr);
    }
    
    let outputs = state.outputs.join(",");
    (outputs, "-".to_string())
}

fn execute_instruction(state: &mut State, inst: &Instruction, i_instr: usize) -> usize {
    match inst.id {
        0 => state.a = state.a / (1 << combo(state, inst.arg)),
        1 => state.b ^= inst.arg,
        2 => state.b = combo(state, inst.arg) % 8,
        3 => {
            if state.a != 0 {
                return inst.arg as usize;
            }
        },
        4 => state.b = state.b ^ state.c,
        5 => state.outputs.push((combo(state, inst.arg) % 8).to_string()),
        6 => state.b = state.a / (1 << combo(state, inst.arg)),
        7 => state.c = state.a / (1 << combo(state, inst.arg)),
        _ => panic!("Bad instruction id {}", inst.id)
    }
    i_instr + 1
}

fn combo(state: &State, arg: i32) -> i32 {
    match arg {
        0 | 1 | 2 | 3 => return arg,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("Bad combo: {arg}")
    }
}

fn parse_inputs(file_path: &str) -> (State, Vec<Instruction>) {
    let mut lines = file_reader(file_path).lines();
    let a = extract_register_value(&lines.next().unwrap().unwrap());
    let b = extract_register_value(&lines.next().unwrap().unwrap());
    let c = extract_register_value(&lines.next().unwrap().unwrap());
    
    lines.next().unwrap().unwrap();
    let program = lines.next().unwrap().unwrap();
    let (_, raw_instructions) = program.split_once(": ").unwrap();

    let state = State{a, b, c, outputs: Vec::new()};
    let instructions = raw_instructions.split(",")
        .map(|i| i.parse::<i32>().unwrap_or_else(|_| panic!("Cannot parse: {i}")))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c|  Instruction{id: c[0], arg: c[1]})
        .collect::<Vec<_>>();
    
    (state, instructions)
}

fn extract_register_value(line: &str) -> i32 {
    let (_, a_register) = line.split_once(": ").unwrap();
    a_register.parse().unwrap()
}