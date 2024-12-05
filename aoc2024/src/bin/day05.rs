use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

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
    let mut splitted = input.split("\n\n");
    let rules = splitted
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split("|");
            let first = parts.next().unwrap().parse::<u32>().unwrap();
            let second = parts.next().unwrap().parse::<u32>().unwrap();
            (first, second)
        })
        .collect::<Vec<(u32, u32)>>();

    splitted
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            let update = line
                .split(",")
                .map(|part| part.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            for rule in &rules {
                if !validate_rule(*rule, &update) {
                    return None;
                }
            }

            Some(update[update.len() / 2])
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let mut splitted = input.split("\n\n");
    let rules = splitted
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split("|");
            let first = parts.next().unwrap().parse::<u32>().unwrap();
            let second = parts.next().unwrap().parse::<u32>().unwrap();
            (first, second)
        })
        .collect::<Vec<(u32, u32)>>();

    splitted
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            let update = line
                .split(",")
                .map(|part| part.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            for rule in &rules {
                if !validate_rule(*rule, &update) {
                    return Some(update);
                }
            }

            None
        })
        .map(|u| {
            let mut u = u;
            u.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    return Ordering::Less;
                }
                if rules.contains(&(*b, *a)) {
                    return Ordering::Greater;
                }
                Ordering::Equal
            });
            u[u.len() / 2]
        })
        .sum::<u32>()
}

fn validate_rule(rule: (u32, u32), update: &Vec<u32>) -> bool {
    let first = update.iter().position(|&x| x == rule.0);
    let second = update.iter().position(|&x| x == rule.1);

    if second.is_some() && first.is_some() && second.unwrap() < first.unwrap() {
        return false;
    }

    true
}
