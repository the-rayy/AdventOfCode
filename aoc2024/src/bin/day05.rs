use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day05.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);
    //
    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let mut splitted = input.split("\n\n");
    let rules = splitted.next().unwrap().lines().map(|line| {
        let mut parts = line.split("|");
        let first = parts.next().unwrap().parse::<u32>().unwrap();
        let second = parts.next().unwrap().parse::<u32>().unwrap();
        (first, second)
    }).collect::<Vec<(u32, u32)>>();

    splitted.next().unwrap().lines().filter_map(|line| {
        let update = line.split(",").map(|part| part.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        for rule in &rules {
            let first = update.iter().position(|&x| x == rule.0);
            let second = update.iter().position(|&x| x == rule.1);

            if second.is_some() && first.is_some() && second.unwrap() < first.unwrap() {
                return None;
            }
        }

        Some(update[update.len() / 2])
    }).sum::<u32>()

}

