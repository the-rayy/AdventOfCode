use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day19.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    _ = lines.next(); //empty line
    lines.map(|x| x.chars().collect::<Vec<_>>()).filter(|x| possible(&towels, x, Vec::new())).count() as u32
}

fn possible(towels: &Vec<Vec<char>>, design: &Vec<char>, test: Vec<char>) -> bool {
    if test.len() == design.len() && test == *design {
        return true;
    }

    if test.len() >= design.len() {
        return false;
    }

    if test != design[0..test.len()] {
        return false;
    }

    for i in 0..towels.len() {
        if possible(towels, design, test.clone().into_iter().chain(towels[i].clone()).collect()) {
            return true;
        }
    }

    false
}
