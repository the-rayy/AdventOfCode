use std::collections::{HashMap, HashSet, VecDeque};
use std::env::current_exe;
use std::fs;
use std::time::Instant;
use num::integer::lcm;

fn main() {
    let input = fs::read_to_string("data/day21.txt")
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
    let mut grid = input.split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| ((i as i64, j as i64), c))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<(i64, i64), char>>();

    let start = *grid.iter().filter(|(_, &c)| c == 'S').next().unwrap().0;
    grid.insert(start, '.');

    let mut current = HashSet::<(i64, i64)>::new();
    current.insert(start);

    for _ in 0..64 {
        let mut new_set = HashSet::<(i64, i64)>::new();
        for p in current.iter() {
            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            for dir in dirs {
                let next = (p.0 + dir.0, p.1 + dir.1);
                if let Some(x) = grid.get(&next) {
                    if *x == '.' {
                        new_set.insert(next);
                    }
                }
            }
        }
        current = new_set;
    }

    current.len()
}
