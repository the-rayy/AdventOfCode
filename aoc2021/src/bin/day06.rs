use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day06.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let max_step: i32 = 80;
    let mut map: HashMap<i64, i64> = HashMap::new();
    input.split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .for_each(|x| *map.entry(x).or_insert(0) += 1);

    for _ in 1..max_step+1 {
        map = step(map);
    }

    map.values().sum()
}

fn part2(input: &str) -> i64 {
    let max_step: i32 = 256;
    let mut map: HashMap<i64, i64> = HashMap::new();
    input.split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .for_each(|x| *map.entry(x).or_insert(0) += 1);

    for _ in 1..max_step+1 {
        map = step(map);
    }

    map.values().sum()
}

fn step(current_map: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_map: HashMap<i64, i64> = HashMap::new();
    for (state, count) in current_map {
        match state {
            0 => {
                *new_map.entry(6).or_insert(0) += count;
                new_map.insert(8, count);
            }
            _ => {
                *new_map.entry(state - 1).or_insert(0) += count;
            }
        }
    };

    new_map
}