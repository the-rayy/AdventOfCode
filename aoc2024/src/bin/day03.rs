use std::fs;
use std::time::Instant;
use regex::Regex;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day03.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

//    let part2_start = Instant::now();
//    let part2_ans = part2(&input);
//    println!("Part 2 time: {:.2?}", part2_start.elapsed());
//    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let pattern = r"mul\((\d\d?\d?),(\d\d?\d?)\)";
    let pattern = Regex::new(pattern).unwrap();

    pattern.captures_iter(input).map(|cap| {
        let a: u32 = cap[1].parse().unwrap();
        let b: u32 = cap[2].parse().unwrap();
        a * b
    }).sum()
}

