use std::fs;
use std::time::Instant;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day10.txt").expect("Unable to load input file");

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
    let grid: HashMap<(i32, i32), u32> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c.to_digit(10).unwrap()))
        })
        .flatten()
        .collect();

    grid.iter()
        .filter(|(_, c)| **c == 0)
        .map(|(pos, c)| score(&grid, *pos, *c).len() as u32)
        .sum()
}

fn score(grid: &HashMap<(i32, i32), u32>, pos: (i32, i32), c: u32) -> HashSet<(i32, i32)> {
    if c == 9 {
        return HashSet::from([pos]);
    }

    let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    dirs.iter()
        .map(|dir| {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(&new_c) = grid.get(&new_pos) {
                if new_c == c + 1 {
                    score(grid, new_pos, new_c)
                } else {
                    HashSet::new()
                }
            } else {
                HashSet::new()
            }
        })
        .flatten()
        .collect()
}

fn part2(input: &str) -> u32 {
    let grid: HashMap<(i32, i32), u32> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c.to_digit(10).unwrap()))
        })
        .flatten()
        .collect();

    grid.iter()
        .filter(|(_, c)| **c == 0)
        .map(|(pos, c)| score2(&grid, *pos, *c))
        .sum()
}

fn score2(grid: &HashMap<(i32, i32), u32>, pos: (i32, i32), c: u32) -> u32 {
    if c == 9 {
        return 1;
    }

    let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    dirs.iter()
        .map(|dir| {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(&new_c) = grid.get(&new_pos) {
                if new_c == c + 1 {
                    score2(grid, new_pos, new_c)
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}
