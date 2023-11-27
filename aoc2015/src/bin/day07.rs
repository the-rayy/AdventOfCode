use std::cmp::max;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use regex;

fn main() {
    let input = fs::read_to_string("data/day07.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u16 {
    let mut gates = HashMap::<&str, &str>::new();
    for line in input.split("\n") {
        let mut splitted = line.split(" -> ");
        let instruction = splitted.next().unwrap();
        let target = splitted.next().unwrap();
        gates.insert(target, instruction);
    }

    let mut solver = Solver{
        gates: &gates,
        cache: HashMap::<&str, u16>::new(),
    };

    solver.solve("a")
}

fn part2(input: &str) -> u16 {
    let mut gates = HashMap::<&str, &str>::new();
    for line in input.split("\n") {
        let mut splitted = line.split(" -> ");
        let instruction = splitted.next().unwrap();
        let target = splitted.next().unwrap();
        gates.insert(target, instruction);
    }

    let mut solver = Solver{
        gates: &gates,
        cache: HashMap::<&str, u16>::new(),
    };

    let a = solver.solve("a");

    let mut solver = Solver{
        gates: &gates,
        cache: HashMap::<&str, u16>::from([("b", a)]),
    };

    solver.solve("a")
}

struct Solver<'a> {
    gates: &'a HashMap::<&'a str, &'a str>,
    cache: HashMap::<&'a str, u16>,
}

impl<'a> Solver<'a> {
    fn solve(&mut self, target: &'a str) -> u16 {
        if let Ok(x) = target.parse::<u16>() {
            return x
        }

        if let Some(&x) = self.cache.get(target) {
            return x
        }

        let instr = self.gates.get(target).unwrap();

        let mut splitted = instr.split(" ");
        if splitted.clone().count() == 1 {
            return self.solve(splitted.last().unwrap())
        }

        if splitted.clone().count() == 2 {
            return !self.solve(splitted.last().unwrap())
        }

        let left = self.solve(splitted.next().unwrap());
        let op = splitted.next().unwrap();
        let right = self.solve(splitted.next().unwrap());

        let res = match op {
            "AND" => { left & right },
            "OR" => { left | right },
            "LSHIFT" => { left << right },
            "RSHIFT" => { left >> right },
            _ => unreachable!()
        };

        self.cache.insert(target, res);
        return res;
    }
}