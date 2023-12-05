use std::fs;
use std::time::Instant;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day04.txt")
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

fn part1(input: &str) -> usize {
    input.split("\n")
        .map(parse_line)
        .map(|x| score(&x.0, &x.1))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut scores = input.split("\n")
        .map(|x| {
            let parsed = parse_line(x);
            let score = matches(&parsed.0, &parsed.1);
            score
        })
        .collect::<Vec<usize>>();

    scores.insert(0, 0); //just to align with card numbers starting with 1

    //because we start with 1 copy of each card
    let mut counts = scores.iter().enumerate()
        .map(|(i, _)| (i, 1))
        .collect::<HashMap<usize, usize>>();

    for (i, score) in scores.iter().enumerate() {
        let current = *counts.get(&i).unwrap_or(&0);
        for x in i+1..i+score+1 {
            *counts.entry(x).or_default() += current;
        }
    }

    counts.values().sum::<usize>() - 1
}

fn parse_line(line: &str) -> (HashSet<usize>, HashSet<usize>) {
    let mut line = line.split(": ")
        .nth(1)
        .unwrap()
        .split(" | ");

    let winning = line.nth(0)
        .unwrap()
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();

    let guessed = line.nth(0)
        .unwrap()
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();

    (winning, guessed)
}

fn score(winning: &HashSet<usize>, guessed: &HashSet<usize>) -> usize {
    match matches(winning, guessed) {
        0 => 0,
        x => 2_usize.pow(x as u32 - 1)
    }
}

fn matches(winning: &HashSet<usize>, guessed: &HashSet<usize>) -> usize {
    winning.intersection(guessed).count()
}