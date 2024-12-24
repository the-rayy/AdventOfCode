use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day24.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

struct Instruction {
  in1: String,
  op: String,
  in2: String,
  out: String,
}

fn part1(input: &str) -> u64 {
  let mut input = input.split("\n\n");
  let mut cache = input.next().unwrap().lines().map(|line| {
    let mut line = line.split(": ");
    let key = line.next().unwrap().to_string();
    let value = match line.next().unwrap() {
      "0" => false,
      "1" => true,
      _ => unreachable!(),
    };
    (key, value)
  }).collect::<HashMap<String, bool>>();
  
  let mut instructions = input.next().unwrap().lines().map(|line| {
    let line = line.replace(" ->", "");
    let mut line = line.split(" ");
    let in1 = line.next().unwrap().to_string();
    let op = line.next().unwrap().to_string();
    let in2 = line.next().unwrap().to_string();
    let out = line.next().unwrap().to_string();
    Instruction { in1, op, in2, out }
  }).collect::<VecDeque<Instruction>>();

  while let Some(x) = instructions.pop_front() {
    let in1 = cache.get(x.in1.as_str());
    let in2 = cache.get(x.in2.as_str());

    if in1.is_none() || in2.is_none() {
      instructions.push_back(x);
      continue;
    }

    let out = match x.op.as_str() {
      "AND" => *in1.unwrap() && *in2.unwrap(),
      "OR" => *in1.unwrap() || *in2.unwrap(),
      "XOR" => in1.unwrap() ^ in2.unwrap(),
      _ => unreachable!(),
    };

    cache.insert(x.out, out);
  }

    let mut keys = cache.keys().filter(|x| x.starts_with("z")).collect::<Vec<_>>();
    keys.sort();

    let mut result = 0;
    for (pos, key) in keys.iter().enumerate() {
      let key = cache.get(*key).unwrap();
      if *key {
        result |= 1 << pos;
      }
    }

    result


}    
