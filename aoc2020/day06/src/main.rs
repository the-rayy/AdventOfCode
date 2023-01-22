use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    input.split("\n\n")
        .map(|g| g.replace("\n", ""))
        .map(|g| g.chars().collect::<Vec<char>>())
        .map(|chrs| chrs.into_iter().sorted())
        .map(|chrs| chrs.dedup().collect::<Vec<char>>())
        .map(|chrs| chrs.len())
        .map(|l| l as i32)
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut correct :i32 = 0;
    for group in input.split("\n\n") {
        let people = group.split("\n").collect::<Vec<&str>>();
        let group_len = people.len();
        let flat = people.iter()
            .map(|p| p.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .into_iter()
            .flatten()
            .into_iter()
            .sorted()
            .collect::<Vec<char>>();

        for ch in 'a' ..= 'z' {
            if flat.iter().filter(|c| **c == ch).count() == group_len {
                correct += 1;
            }
        }

    }
    correct
}