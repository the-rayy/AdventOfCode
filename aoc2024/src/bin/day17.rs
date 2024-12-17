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

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> String {
    let mut cpu = Cpu::default();

    for line in input.lines() {
        let mut s = line.split(": ");
        match s.next().unwrap() {
            "Register A" => {
                cpu.register_a = s.next().unwrap().parse::<u32>().unwrap();
            },
            "Register B" => {
                cpu.register_b = s.next().unwrap().parse::<u32>().unwrap();
            },
            "Register C" => {
                cpu.register_c = s.next().unwrap().parse::<u32>().unwrap();
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

#[derive(Default)]
struct Cpu {
    register_a: u32,
    register_b: u32,
    register_c: u32,

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

    fn tick(&mut self, opcode: u32, operand: u32) {
        match opcode {
            0 => { //adv
                self.register_a = self.register_a / 2_u32.pow(self.combo(operand));
                self.pointer += 2;
            },
            1 => { //bxl
                self.register_b = self.register_b.bitxor(operand);
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
                self.output.push(self.combo(operand) % 8);
                self.pointer += 2;
            },
            6 => { //bdv
                self.register_b = self.register_a / 2_u32.pow(self.combo(operand));
                self.pointer += 2;
            },
            7 => { //cdv
                self.register_c = self.register_a / 2_u32.pow(self.combo(operand));
                self.pointer += 2;
            }
            _ => unreachable!(),

        }
    }

    fn combo(&self, operand: u32) -> u32 {
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
