use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day15.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}


fn part1(input: &str) -> usize {
    input.split(",")
        .map(|x| hash(x))
        .sum()
}

fn hash(input: &str) -> usize {
    input.chars()
        .fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
}