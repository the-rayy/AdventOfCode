use std::fs;

fn main() {
    let input = fs::read_to_string("data/day02.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut forward: i32 = 0;
    let mut depth: i32 = 0;
    input.split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|splitted| (splitted[0], splitted[1].parse::<i32>().unwrap()))
        .for_each(|(command, value)| match (command, value) {
            ("forward", _) => { forward = forward + value },
            ("up", _) => { depth = depth - value },
            ("down", _) => { depth = depth + value },
            (_, _) => unreachable!(),
        });
    forward * depth
}

fn part2(input: &str) -> i32 {
    let mut forward: i32 = 0;
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    input.split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|splitted| (splitted[0], splitted[1].parse::<i32>().unwrap()))
        .for_each(|(command, value)| match (command, value) {
            ("forward", _) => { forward = forward + value; depth = depth + (aim * value) },
            ("up", _) => { aim = aim - value },
            ("down", _) => { aim = aim + value },
            (_, _) => unreachable!(),
        });
    forward * depth
}