use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day01.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    input.lines().for_each(|line| {
        let mut split = line.split(" ");
        left.push(split.next().unwrap().parse::<i32>().unwrap());
        right.push(split.last().unwrap().parse::<i32>().unwrap());
    });

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>() as u32
}

fn part2(input: &str) -> u32 {
    let mut right_counts: HashMap<i32, i32> = HashMap::with_capacity(1000);
    let mut left = Vec::with_capacity(1000);

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let l = split.next().unwrap().parse::<i32>().unwrap();
        let right = split.next().unwrap().parse::<i32>().unwrap();

        *right_counts.entry(right).or_insert(0) += 1;

        left.push(l);        
    };

    left.iter()
        .map(|l| right_counts.get(l).unwrap_or(&0) * l)
        .sum::<i32>() as u32
}
