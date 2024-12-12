use std::fs;
use std::time::Instant;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day12.txt").expect("Unable to load input file");

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
    let mut grid: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .flatten()
        .collect();

    let mut total_score = 0;
    loop {
        let start = grid.iter().filter(|(k, v)| **v != ' ').next();
        if start.is_none() {
            break;
        }

        let start = start.unwrap();

        let region = flood_fill(&grid, *start.0, *start.1);
        let score = calculate_score(&grid, &region, *start.1);
        total_score += score;

        for pos in region {
            grid.insert(pos, ' ');
        }
    }

    total_score
}

fn flood_fill(grid: &HashMap<(i32, i32), char>, pos: (i32, i32), c: char) -> HashSet<(i32, i32)> {
    let mut stack = vec![pos];
    let mut region = HashSet::new();

    while let Some(pos) = stack.pop() {
        if let Some(&new_c) = grid.get(&pos) {
            if new_c == c {
                region.insert(pos);

                let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

                for dir in dirs {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if !region.contains(&new_pos) {
                        stack.push(new_pos);
                    }
                }
            }
        }
    }

    region
}

fn calculate_score(grid: &HashMap<(i32, i32), char>, region: &HashSet<(i32, i32)>, c:  char) -> u32 {
    let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let perimeter = region.iter().map(|pos| {
        4 - dirs.iter().filter(|dir| {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(&new_c) = grid.get(&new_pos) {
                new_c == c
            } else {
                false
            }
        }).count() as u32
    }).sum::<u32>();

    perimeter * region.len() as u32
}
