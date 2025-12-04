use std::fs;
use std::time::Instant;

use hashbrown::HashSet;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day04.txt").expect("Unable to load input file");

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
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '@')
                .map(|(j, _)| (i as i32, j as i32))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashSet<_>>();

    map.iter()
        .filter(|(i, j)| {
            let mut neighbours = 0;
            for x in -1_i32..=1 {
                for y in -1_i32..=1 {
                    neighbours += if map.contains(&(i + x, j + y)) { 1 } else { 0 };
                }
            }
            neighbours < 5
        })
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let mut map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '@')
                .map(|(j, _)| (i as i32, j as i32))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashSet<_>>();
    let initial_size = map.len();
    loop {
        let rem = map
            .iter()
            .filter(|(i, j)| {
                let mut neighbours = 0;
                for x in -1_i32..=1 {
                    for y in -1_i32..=1 {
                        neighbours += if map.contains(&(i + x, j + y)) { 1 } else { 0 };
                    }
                }
                neighbours < 5
            })
            .cloned()
            .collect_vec();
        if rem.len() == 0 {
            break;
        };
        rem.iter().for_each(|x| {
            map.remove(x);
        });
    }
    (initial_size - map.len()) as u32
}
