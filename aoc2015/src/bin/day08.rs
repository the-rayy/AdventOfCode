use std::cmp::max;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use regex;

fn main() {
    let input = fs::read_to_string("data/day08.txt")
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

fn part1(input: &str) -> u64 {
    input.split("\n")
        .map(|line| {
            let literals = line.chars().count();

            let line = line.strip_prefix("\"").unwrap();
            let line = line.strip_suffix("\"").unwrap();

            let line = line.replace(r"\\", "_");
            let line = line.replace("\\\"", "_");

            let hexes = line.match_indices(r"\x").count();
            let memory = line.chars().count() - 3*hexes;

            literals - memory
        })
        .sum::<usize>() as u64
}

fn part2(input: &str) -> u64 {
    input.split("\n")
        .map(|line| {
            let literals = line.chars().count();

            let quotes = line.match_indices("\"").count();
            let slashes = line.match_indices("\\").count();

            let new_literals = literals + quotes + slashes + 2;

            new_literals - literals
        })
        .sum::<usize>() as u64
}
