use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day19.txt").expect("Unable to load input file");

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
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    _ = lines.next(); //empty line
    lines.map(|x| x.chars().collect::<Vec<_>>()).filter(|x| possible(&towels, x)).count() as u32
}

fn possible(towels: &Vec<Vec<char>>, design: &[char]) -> bool {
    if design.len() == 0 {
        return true;
    }

    towels.iter().any(|towel| {
        if towel.len() > design.len() {
            return false;
        }

        if towel != &design[0..towel.len()] {
            return false;
        }

        possible(towels, &design[towel.len()..])
    })
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    _ = lines.next(); //empty line
    //
    let mut cache = HashMap::new();
    lines.map(|x| x.chars().collect::<Vec<_>>()).map(|x| possibillities(&towels, &mut cache, &x)).sum::<u64>()
}

fn possibillities(towels: &Vec<Vec<char>>, cache: &mut HashMap<String, u64>, design: &[char]) -> u64 {
    if let Some(&res) = cache.get(&design.iter().collect::<String>()) {
        return res;
    }

    if design.len() == 0 {
        return 1;
    }

    towels.iter().map(|towel| {
        if towel.len() > design.len() {
            return 0;
        }

        if towel != &design[0..towel.len()] {
            return 0;
        }

        let res = possibillities(towels, cache, &design[towel.len()..]);
        cache.insert(design[towel.len()..].iter().collect(), res);
        res
    }).sum()
}


