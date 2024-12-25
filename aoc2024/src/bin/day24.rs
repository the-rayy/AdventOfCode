use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::ops::BitXor;
use std::time::Instant;
use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("data/day24.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    //assert_eq!(part1_ans, 56729630917616);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

#[derive(Debug, Clone)]
struct Instruction {
    in1: String,
    op: String,
    in2: String,
}

#[derive(Clone)]
struct Adder {
    cache: HashMap<String, bool>,
    instructions: HashMap<String, Instruction>,
    bitlen: usize,
}

impl Adder {
    fn integer(&mut self, start: &str) -> u64 {
        let mut result = 0;
        for pos in 0..=self.bitlen {
            let key = format!("{}{:0>2}", start, pos);
            match self.solve(&key) {
                Some(true) => result |= 1 << pos,
                Some(_) => (),
                None => {
                    if start == "z" {
                        unreachable!()
                    }
                }
            }
        }

        result
    }

    fn swap(&mut self, k1: &str, k2: &str) {
        let tmp = self.instructions.get(k1).unwrap().clone();
        self.instructions
            .insert(k1.to_string(), self.instructions.get(k2).unwrap().clone());
        self.instructions.insert(k2.to_string(), tmp);
    }

    fn bitprecision(&mut self) -> u32 {
        let x = self.integer("x");
        let y = self.integer("y");
        let z = self.integer("z");
        let zz = x + y;
        let diff = z.bitxor(&zz);
        diff.count_ones()
    }

    fn solve(&mut self, key: &str) -> Option<bool> {
        match self.cache.get(key) {
            Some(x) => Some(*x),
            None => {
                let instruction = self.instructions.get(key);
                if instruction.is_none() {
                    return None;
                }
                let instruction = instruction.unwrap().clone();
                let in1 = self.solve(&instruction.in1).unwrap();
                let in2 = self.solve(&instruction.in2).unwrap();

                let out = match instruction.op.as_str() {
                    "AND" => in1 && in2,
                    "OR" => in1 || in2,
                    "XOR" => in1 ^ in2,
                    _ => unreachable!(),
                };

                self.cache.insert(key.to_string(), out);
                Some(out)
            }
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    let cache = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line = line.split(": ");
            let key = line.next().unwrap().to_string();
            let value = match line.next().unwrap() {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            };
            (key, value)
        })
        .collect::<HashMap<String, bool>>();

    let instructions = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.replace(" ->", "");
            let mut line = line.split(" ");
            let in1 = line.next().unwrap().to_string();
            let op = line.next().unwrap().to_string();
            let in2 = line.next().unwrap().to_string();
            let out = line.next().unwrap().to_string();
            (out, Instruction { in1, op, in2 })
        })
        .collect::<HashMap<String, Instruction>>();

    let bitlen = cache.len() / 2;
    let mut adder = Adder {
        cache,
        instructions,
        bitlen,
    };
    adder.integer("z")
}

fn part2(input: &str) -> String {
    let mut input = input.split("\n\n");
    let cache = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line = line.split(": ");
            let key = line.next().unwrap().to_string();
            let value = match line.next().unwrap() {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            };
            (key, value)
        })
        .collect::<HashMap<String, bool>>();

    let instructions = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.replace(" ->", "");
            let mut line = line.split(" ");
            let in1 = line.next().unwrap().to_string();
            let op = line.next().unwrap().to_string();
            let in2 = line.next().unwrap().to_string();
            let out = line.next().unwrap().to_string();
            (out, Instruction { in1, op, in2 })
        })
        .collect::<HashMap<String, Instruction>>();

    let bitlen = cache.len() / 2;
    let mut adder = Adder {
        cache,
        instructions: instructions.clone(),
        bitlen,
    };

    // Find candidates possible candidates. Try to go from X and Y wires to corresponding Z wire. If
    // its not possible, then Z wire or xy_xor wire is wrong.
    let mut candidates = HashSet::new();
    for n in 1..45 {
        let x_n = format!("x{:0>2}", n);
        let y_n = format!("y{:0>2}", n);
        let z_n = format!("z{:0>2}", n);
        let xy_xor = instructions
            .iter()
            .find(|(out, instruction)| {
                (instruction.in1 == x_n && instruction.in2 == y_n && instruction.op == "XOR")
                    || (instruction.in2 == x_n && instruction.in1 == y_n && instruction.op == "XOR")
            })
            .unwrap()
            .0;

        if instructions
            .iter()
            .filter(|(out, instruction)| {
                (instruction.in1 == *xy_xor || instruction.in2 == *xy_xor)
                    && instruction.op == "XOR"
                    && **out == z_n
            })
            .count()
            != 1
        {
            candidates.insert(z_n);
            candidates.insert(xy_xor.to_string());
        }
    }

    // Print candidates instructions to see what other wire should go there.
    // After that, the rest was found by hand.
    //
    // This code is just to show that i found the solution by hand.
    // Works only for my input.
    let mut wrong_wires = vec!["z13", "hsw", "z18", "skf", "z07", "bjm", "wkr", "nvr"];
    for i in 0..4 {
        adder.swap(wrong_wires[i * 2], wrong_wires[i * 2 + 1]);
    }
    assert_eq!(adder.bitprecision(), 0);

    wrong_wires.sort();
    wrong_wires.join(",")
}
