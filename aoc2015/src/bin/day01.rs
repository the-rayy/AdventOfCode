use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day01.txt")
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

fn part1(input: &str) -> i64 {
    input.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        })
        .scan(0, |acc, v| {
            *acc = *acc + v;
            Some(*acc)
        })
        .position(|v| v == -1)
        .unwrap() + 1
}