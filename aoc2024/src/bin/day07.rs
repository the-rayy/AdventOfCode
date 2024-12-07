use itertools::Itertools;
use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

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

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn part1(input: &str) -> u64 {
    let available_operations = vec![Operator::Add, Operator::Multiply];
    input
        .lines()
        .filter_map(|line| {
            let numbers = line
                .replace(":", "")
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let desired = numbers[0];
            let numbers = &numbers[1..];

            eval(desired, numbers, &available_operations)
        })
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let available_operations = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];
    input
        .lines()
        .filter_map(|line| {
            let numbers = line
                .replace(":", "")
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let desired = numbers[0];
            let numbers = &numbers[1..];
            eval(desired, numbers, &available_operations)
        })
        .sum::<u64>()
}

fn eval(target: u64, numbers: &[u64], available_operations: &[Operator]) -> Option<u64> {
    let operations =
        itertools::repeat_n(available_operations.iter(), numbers.len()).multi_cartesian_product();

    'outer: for operation in operations {
        if *operation[0] != Operator::Add {
            continue;
        }

        let mut acc = 0;
        for i in 0..numbers.len() {
            match operation[i] {
                Operator::Add => acc += numbers[i],
                Operator::Multiply => acc *= numbers[i],
                Operator::Concatenate => {
                    acc = concatenate_numbers(acc, numbers[i]);
                }
            }
            if acc > target {
                continue 'outer;
            }
        }

        if acc == target {
            return Some(acc);
        }
    }
    None
}

fn concatenate_numbers(a: u64, b: u64) -> u64 {
    let mut temp_b = b;
    let mut multiplier = 1;

    while temp_b > 0 {
        multiplier *= 10;
        temp_b /= 10;
    }

    a * multiplier + b
}
