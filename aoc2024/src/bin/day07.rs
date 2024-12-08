use std::fs;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day07.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn part1(input: &str) -> u64 {
    let operators = vec![Operator::Multiply, Operator::Add];
    input
        .par_lines()
        .filter_map(|line| {
            let numbers = line
                .replace(":", "")
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let desired = numbers[0];
            let numbers = &numbers[1..];

            let acc = 0;
            if eval_recursive(desired, acc, numbers, Operator::Add, &operators) {
                Some(desired)
            } else {
                None
            }
        })
        .sum::<u64>()
}


fn part2(input: &str) -> u64 {
    let operators = vec![Operator::Concatenate, Operator::Multiply, Operator::Add];
    input
        .par_lines()
        .filter_map(|line| {
            let numbers = line
                .replace(":", "")
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let desired = numbers[0];
            let numbers = &numbers[1..];

            let acc = 0;
            if eval_recursive(desired, acc, numbers, Operator::Add, &operators) {
                Some(desired)
            } else {
                None
            }
        })
        .sum::<u64>()
}

fn eval_recursive(target: u64, acc: u64, numbers: &[u64], op: Operator, operators: &[Operator]) -> bool {
    if numbers.is_empty() {
        return acc == target;
    }

    if acc > target {
        return false;
    }

    let acc = match op {
        Operator::Add => acc + numbers[0],
        Operator::Multiply => acc * numbers[0],
        Operator::Concatenate => concatenate_numbers(acc, numbers[0]),
    };

    let numbers = &numbers[1..];
    operators.iter().any(|new_op| eval_recursive(target, acc, numbers, *new_op, operators))
}

fn concatenate_numbers(a: u64, b: u64) -> u64 {
    let multiplier = b.ilog10() + 1;
    a * 10_u64.pow(multiplier) + b
}
