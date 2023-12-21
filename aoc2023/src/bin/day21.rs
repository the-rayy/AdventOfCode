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

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
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


// very good explanation: https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
fn part2(input: &str) -> usize {
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

    let dims = (131, 131);

    let mut visited = HashMap::<(i64, i64), usize>::new();
    let mut queue = VecDeque::<((i64, i64), usize)>::new();
    queue.push_back((start, 0));

    while let Some((point, dist)) = queue.pop_front() {
        if visited.contains_key(&point) {
            continue;
        }

        visited.insert(point, dist);

        for dir in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (point.0 + dir.0, point.1 + dir.1);
            if next.0 < 0 || next.0 >= dims.0 || next.1 < 0 || next.1 >= dims.1 {
                continue;
            }

            if !visited.contains_key(&next) && grid.get(&next).unwrap() == &'.' {
                queue.push_back((next, dist + 1));
            }
        }
    }

    let event_corners = visited.values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited.values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let maps_count = ((26501365 - (dims.0 / 2)) / dims.0) as usize;

    let even = maps_count.pow(2);
    let odd = (maps_count+1).pow(2);

    odd * visited.values().filter(|v| **v % 2 == 1).count()
    + even * visited.values().filter(|v| **v % 2 == 0).count()
    - ((maps_count + 1) * odd_corners)
    + (maps_count) * event_corners
}
