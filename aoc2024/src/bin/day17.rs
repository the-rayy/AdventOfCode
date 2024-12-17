use std::fs;
use std::ops::BitXor;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day17.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> String {
    let mut cpu = Cpu::default();

    for line in input.lines() {
        let mut s = line.split(": ");
        match s.next().unwrap() {
            "Register A" => {
                cpu.register_a = s.next().unwrap().parse::<u64>().unwrap();
            },
            "Register B" => {
                cpu.register_b = s.next().unwrap().parse::<u64>().unwrap();
            },
            "Register C" => {
                cpu.register_c = s.next().unwrap().parse::<u64>().unwrap();
            },
            "Program" => {
                cpu.program = s.next().unwrap().split(",").map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            },
            "" => (),
            _ => unreachable!(),
        }
    }

    cpu.run();
    cpu.output.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",")
}

fn part2(input: &str) -> u64 {
    let mut cpu = Cpu::default();

    for line in input.lines() {
        let mut s = line.split(": ");
        match s.next().unwrap() {
            "Register A" => {
                cpu.register_a = s.next().unwrap().parse::<u64>().unwrap();
            },
            "Register B" => {
                cpu.register_b = s.next().unwrap().parse::<u64>().unwrap();
            },
            "Register C" => {
                cpu.register_c = s.next().unwrap().parse::<u64>().unwrap();
            },
            "Program" => {
                cpu.program = s.next().unwrap().split(",").map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            },
            "" => (),
            _ => unreachable!(),
        }
    }

    let mut result = Vec::with_capacity(cpu.program.len());
    let mut i = 0;
    let mut d = 0;

    while i < cpu.program.len() {
        cpu.reset();
        result.push(d);
        cpu.register_a = digits_to_reg(&result);
        cpu.run();

        if partial_match(&cpu.output, &cpu.program) {
            i += 1;
            d = 0;
            continue;
        }

        if d < 7 {
            d += 1;
            result.pop();
            continue;
        }
    
        result.pop();
        d = result.pop().unwrap() + 1;
        i -= 1;
    }

    digits_to_reg(&result)
}

fn digits_to_reg(digits: &[u32]) -> u64 {
    let mut reg = 0_u64;
    for d in digits {
        reg = reg * 8 + *d as u64;
    }
    reg
}

fn partial_match(output: &[u32], program: &[u32]) -> bool {
    let program_tail = &program[program.len()-output.len()..];

    output == program_tail
}

#[derive(Default, Debug)]
struct Cpu {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    pointer: u32,
    program: Vec<u32>,
    output: Vec<u32>,
}

impl Cpu {
    fn run(&mut self) {
        while self.pointer < self.program.len() as u32 {
            let opcode = self.program[self.pointer as usize];
            let operand = self.program[self.pointer as usize + 1];
            self.tick(opcode, operand);
        }
    }

    fn reset(&mut self) {
        self.register_a = 0;
        self.register_b = 0;
        self.register_c = 0;
        self.pointer = 0;
        self.output.clear();
    }

    fn tick(&mut self, opcode: u32, operand: u32) {
        match opcode {
            0 => { //adv
                self.register_a = self.register_a / 2_u64.pow(self.combo(operand) as u32);
                self.pointer += 2;
            },
            1 => { //bxl
                self.register_b = self.register_b.bitxor(operand as u64);
                self.pointer += 2;
            },
            2 => { //bst
                self.register_b =  self.combo(operand) % 8;
                self.pointer += 2;
            },
            3 => { //jnz
                if self.register_a != 0 {
                    self.pointer = operand;
                } else {
                    self.pointer += 2;
                }
            },
            4 => { //bxc
                self.register_b = self.register_b.bitxor(self.register_c);
                self.pointer += 2;
            },
            5 => { //out
                self.output.push((self.combo(operand) % 8) as u32);
                self.pointer += 2;
            },
            6 => { //bdv
                self.register_b = self.register_a / 2_u64.pow(self.combo(operand) as u32);
                self.pointer += 2;
            },
            7 => { //cdv
                self.register_c = self.register_a / 2_u64.pow(self.combo(operand) as u32);
                self.pointer += 2;
            }
            _ => unreachable!(),

        }
    }

    fn combo(&self, operand: u32) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        }
    }
}
