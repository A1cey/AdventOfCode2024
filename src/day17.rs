use std::{
    fs,
    ops::{BitXor, Div},
};

pub fn run() {
    println!("Day 17:");

    let input = fs::read_to_string("src/input/input17.txt").unwrap();

    let (mut registers, program) = parse_program(&input);

    let output = execute(registers, &program);
    let reg_a_value = smarter_brute_force(&program);

    println!(
        "Output: {}",
        output.iter().fold(String::new(), |mut s, num| {
            s.push_str(num.to_string().as_str());
            s
        })
    );
    println!("Register a: {reg_a_value}");
    println!();
}

fn parse_program(input: &str) -> ([u64; 3], Vec<u64>) {
    let (registers_str, program_str) = input.split_once("\r\n\r\n").unwrap();

    let registers = registers_str
        .lines()
        .enumerate()
        .fold([0; 3], |mut registers, (idx, line)| {
            registers[idx] = line.split_once(": ").unwrap().1.parse().unwrap();
            registers
        });

    let program = program_str
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    (registers, program)
}

fn execute(mut registers: [u64; 3], program: &[u64]) -> Vec<u64> {
    let mut out_buffer = Vec::new();

    let mut pointer = 0;

    while pointer < program.len() - 1 {
        let opcode = program[pointer];
        let operand = program[pointer + 1];

        match opcode {
            0 => registers[0] = registers[0].div(2u64.pow(get_combo_operand(operand, &registers))),
            1 => registers[1] = registers[1].bitxor(operand),
            2 => registers[1] = get_combo_operand(operand, &registers) as u64 % 8,
            3 => {
                if registers[0] != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => registers[1] = registers[1].bitxor(registers[2]),
            5 => out_buffer.push(get_combo_operand(operand, &registers) as u64 % 8),
            6 => registers[1] = registers[0].div(2u64.pow(get_combo_operand(operand, &registers))),
            7 => registers[2] = registers[0].div(2u64.pow(get_combo_operand(operand, &registers))),
            _ => unreachable!(),
        }

        pointer += 2;
    }

    out_buffer
}

fn get_combo_operand(operand: u64, registers: &[u64; 3]) -> u32 {
    match operand {
        0 | 1 | 2 | 3 => operand as u32,
        4 => registers[0] as u32,
        5 => registers[1] as u32,
        6 => registers[2] as u32,
        7 => panic!("Not a valid program. 7 is reserved."),
        _ => unreachable!(),
    }
}

fn brute_force(program: &[u64]) -> u64 {
    for reg_a_val in 0..u64::MAX {
        if program == execute([reg_a_val, 0, 0], program) {
            return reg_a_val;
        }
    }

    panic!("No possible a value found");
}

fn smarter_brute_force(program: &[u64]) -> u64 {
    // The digits in the output seem to change after a fixed period. Each
    // period is the previous one times 8: [1, 8, 64, 512, ...].

    // Try all factors of each period and compare their output.
    let mut factors = vec![0; program.len()];

    loop {
        let mut init_a = 0;
        for (i, f) in factors.iter().enumerate() {
            init_a += 8u64.pow(i as u32) * f
        }

        let output = execute([init_a, 0, 0], &program);

        if output == program {
            return init_a;
        }

        for i in (0..program.len()).rev() {
            if output.len() < i {
                factors[i] += 1;
                break;
            }
            if output[i] != program[i] {
                factors[i] += 1;
                break;
            }
        }
    }
}
