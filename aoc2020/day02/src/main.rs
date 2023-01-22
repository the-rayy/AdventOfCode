use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut correct = 0;
    let re: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    for line in input.split("\n") {
        let caps = re.captures(line).unwrap();
        let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let char = caps.get(3).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();

        let count = password.matches(char).count();
        if min <= count && count <= max {
            correct += 1;
        }
    }
    correct
}

fn part2(input: &str) -> i32 {
    let mut correct = 0;
    let re: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    for line in input.split("\n") {
        let caps = re.captures(line).unwrap();
        let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let ch = caps.get(3).unwrap().as_str().chars().collect::<Vec<char>>()[0];
        let password = caps.get(4).unwrap().as_str().chars().collect::<Vec<char>>();

        if (password[min-1] == ch) ^ (password[max-1] == ch) {
            correct += 1;
        }
    }
    correct
}