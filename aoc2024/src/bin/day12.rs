use std::fs;
use std::time::Instant;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day12.txt").expect("Unable to load input file");

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
        let start = grid.iter().filter(|(_, v)| **v != ' ').next();
        if start.is_none() {
            break;
        }

        let start = start.unwrap();

        let region = flood_fill(&grid, *start.0, *start.1);
        let score = calculate_score(&region);
        total_score += score;

        for pos in region {
            grid.insert(pos, ' ');
        }
    }

    total_score
}

fn part2(input: &str) -> u32 {
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
        let start = grid.iter().filter(|(_, v)| **v != ' ').next();
        if start.is_none() {
            break;
        }

        let start = start.unwrap();

        let region = flood_fill(&grid, *start.0, *start.1);
        let score = calculate_score2(&region);
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

fn calculate_score(region: &HashSet<(i32, i32)>) -> u32 {
    let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let perimeter = region
        .iter()
        .map(|pos| {
            4 - dirs
                .iter()
                .filter(|dir| {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    region.contains(&new_pos)
                })
                .count() as u32
        })
        .sum::<u32>();

    perimeter * region.len() as u32
}

fn calculate_score2(region: &HashSet<(i32, i32)>) -> u32 {
    let vertices = region
        .iter()
        .map(|pos| {
            let top_left = region.contains(&(pos.0 - 1, pos.1 - 1));
            let top = region.contains(&(pos.0 - 1, pos.1));
            let top_right = region.contains(&(pos.0 - 1, pos.1 + 1));
            let left = region.contains(&(pos.0, pos.1 - 1));
            let right = region.contains(&(pos.0, pos.1 + 1));
            let bottom_right = region.contains(&(pos.0 + 1, pos.1 + 1));
            let bottom = region.contains(&(pos.0 + 1, pos.1));
            let bottom_left = region.contains(&(pos.0 + 1, pos.1 - 1));

            let mut vertices = 0;

            //outer vertices
            if !left && !top_left && !top {
                vertices += 1;
            };
            if !top && !top_right && !right {
                vertices += 1;
            };
            if !right && !bottom_right && !bottom {
                vertices += 1;
            };
            if !bottom && !bottom_left && !left {
                vertices += 1;
            };

            //inner vertices
            if left && !top_left && top {
                vertices += 1;
            };
            if top && !top_right && right {
                vertices += 1;
            };
            if right && !bottom_right && bottom {
                vertices += 1;
            };
            if bottom && !bottom_left && left {
                vertices += 1;
            };

            //inner, but joining
            if !left && top_left && !top {
                vertices += 1;
            };
            if !top && top_right && !right {
                vertices += 1;
            };
            if !right && bottom_right && !bottom {
                vertices += 1;
            };
            if !bottom && bottom_left && !left {
                vertices += 1;
            };

            vertices
        })
        .sum::<u32>();

    vertices * region.len() as u32
}
