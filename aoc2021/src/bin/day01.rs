use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day01.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let numbers: Vec<i32> = input.split("\n")
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut deeper = 0;
    for (a, b) in numbers.into_iter().tuple_windows() {
        if b > a {
            deeper = deeper + 1;
        }
    }
    deeper
}

fn part2(input: &str) -> i32 {
    let numbers: Vec<i32> = input.split("\n")
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let windows: Vec<i32> = numbers.into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a+b+c)
        .collect();
    let mut deeper = 0;
    for (a, b) in windows.into_iter().tuple_windows() {
        if b > a {
            deeper = deeper + 1;
        }
    }
    deeper
}