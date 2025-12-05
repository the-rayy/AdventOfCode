use std::fs;
use std::time::Instant;

use hashbrown::HashSet;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day05.txt").expect("Unable to load input file");

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
    let mut s = input.split("\n\n");
    let ranges = s
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut s = line.split("-");
            let start = s.next().unwrap().parse::<usize>().unwrap();
            let end = s.next().unwrap().parse::<usize>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    s.next()
        .unwrap()
        .lines()
        .filter(|line| {
            let n = line.parse::<usize>().unwrap();
            ranges.iter().any(|r| r.contains(&n))
        })
        .count() as u32
}

fn part2(input: &str) -> usize {
    let ranges = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut s = line.split("-");
            let start = s.next().unwrap().parse::<usize>().unwrap();
            let end = s.next().unwrap().parse::<usize>().unwrap();
            start..=end
        })
        .sorted_by_key(|r| *r.start())
        .collect::<Vec<_>>();

    let mut merged = Vec::new();
    let mut current = ranges[0].clone();

    for r in ranges.into_iter().skip(1) {
        if r.start() <= current.end() {
            current = *current.start()..=(*current.end()).max(*r.end());
        } else {
            merged.push(current);
            current = r;
        }
    }

    merged.push(current);

    merged.into_iter().map(|m| m.count()).sum::<usize>()
}
