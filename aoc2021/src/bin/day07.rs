use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day07.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut numbers: Vec<i64> = input.split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    numbers.sort();
    let mid = numbers.len() / 2;
    let pos: i64 = numbers[mid]; //median is the best position

    numbers.iter()
        .map(|x| (*x - pos).abs())
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    let numbers: Vec<i64> = input.split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    let mut fuel_consumption: HashMap<i64, u64> = HashMap::new();
    for candidate in min..max {
        let fuel: u64 = numbers.iter()
            .map(|x| (*x - candidate).abs())
            .map(|x| dist(x as u64))
            .sum();
        fuel_consumption.insert(candidate, fuel);
    };

    *fuel_consumption.values().min().unwrap() as i64
}

fn dist(x: u64) -> u64 {
    (x * (x + 1)) / 2
}
