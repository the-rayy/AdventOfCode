use std::cmp::max;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use regex;

fn main() {
    let input = fs::read_to_string("data/day06.txt")
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

const REGEXP: &str = r"(.*) (\d+),(\d+) through (\d+),(\d+)";
fn part1(input: &str) -> usize {
    let mut lights = HashSet::new();
    let regex = regex::Regex::new(REGEXP).unwrap();

    for line in input.lines() {
        let caps = regex.captures(line).unwrap();
        let action = caps.get(1).unwrap().as_str();
        let x1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y1 = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let x2 = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let y2 = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
        for x in x1..=x2 {
            for y in y1..=y2 {
                match action {
                    "turn on" => { lights.insert((x, y)); },
                    "turn off" => { lights.remove(&(x, y)); },
                    "toggle" => {
                        if lights.contains(&(x, y)) {
                            lights.remove(&(x, y));
                        } else {
                            lights.insert((x, y));
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }
    lights.len()
}

fn part2(input: &str) -> i64 {
    let mut lights = HashMap::<(usize, usize), i64>::new();
    let regex = regex::Regex::new(REGEXP).unwrap();

    for line in input.lines() {
        let caps = regex.captures(line).unwrap();
        let action = caps.get(1).unwrap().as_str();
        let x1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y1 = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let x2 = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let y2 = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
        for x in x1..=x2 {
            for y in y1..=y2 {
                let light = lights.entry((x, y)).or_insert(0);
                match action {
                    "turn on" => { *light += 1; },
                    "turn off" => { *light = max((*light) - 1, 0); },
                    "toggle" => { *light += 2; },
                    _ => unreachable!()
                }
            }
        }
    }
    lights.values().sum()
}