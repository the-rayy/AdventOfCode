use std::collections::VecDeque;
use std::fs;
use std::ops::BitXor;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day24.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    assert_eq!(part1_ans, 56729630917616);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

#[derive(Debug, Clone)]
struct Instruction {
  in1: String,
  op: String,
  in2: String,
}

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
      if self.solve(&key) {
        result |= 1 << pos;
      }
    }

    result
  }

  fn solve(&mut self, key: &str) -> bool {
    match self.cache.get(key) {
      Some(x) => *x,
      None => {
        let instruction = self.instructions.get(key).unwrap().clone();
        let in1 = self.solve(&instruction.in1);
        let in2 = self.solve(&instruction.in2);

        let out = match instruction.op.as_str() {
          "AND" => in1 && in2,
          "OR" => in1 || in2,
          "XOR" => in1 ^ in2,
          _ => unreachable!(),
        };

        self.cache.insert(key.to_string(), out);
        out
      }
    }
  }
}

fn part1(input: &str) -> u64 {
  let mut input = input.split("\n\n");
  let cache = input.next().unwrap().lines().map(|line| {
    let mut line = line.split(": ");
    let key = line.next().unwrap().to_string();
    let value = match line.next().unwrap() {
      "0" => false,
      "1" => true,
      _ => unreachable!(),
    };
    (key, value)
  }).collect::<HashMap<String, bool>>();
  
  let instructions = input.next().unwrap().lines().map(|line| {
    let line = line.replace(" ->", "");
    let mut line = line.split(" ");
    let in1 = line.next().unwrap().to_string();
    let op = line.next().unwrap().to_string();
    let in2 = line.next().unwrap().to_string();
    let out = line.next().unwrap().to_string();
    (out, Instruction { in1, op, in2 })
  }).collect::<HashMap<String, Instruction>>();

  let bitlen = cache.len() / 2;
  let mut adder = Adder { cache, instructions, bitlen };
  adder.integer("z")
} 

