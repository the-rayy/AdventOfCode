use std::{collections::VecDeque, fs};
use std::time::Instant;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day18.txt").expect("Unable to load input file");

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
    let dims = (70, 70);
    let start = (0, 0);
    let end = (70, 70);
    let num_obstacles = 1024;

    let obstacles = input.lines().enumerate().filter_map(|(i, l)| {
        if i >= num_obstacles { return None; }
        let mut s = l.split(",");
        Some((s.next().unwrap().parse::<u32>().unwrap(), s.next().unwrap().parse::<u32>().unwrap()))
    }).collect::<HashSet<_>>();

    pathfinding(dims, start, end, &obstacles).unwrap()
}

fn part2(input: &str) -> String {
    let dims = (70, 70);
    let start = (0, 0);
    let end = (70, 70);

    let obstacles = input.lines().map(|l| {
        let mut s = l.split(",");
        (s.next().unwrap().parse::<u32>().unwrap(), s.next().unwrap().parse::<u32>().unwrap())
    }).collect::<Vec<_>>();

    let mut min_obstacle = 1024;
    let mut max_obstacle = obstacles.len();

    while min_obstacle < max_obstacle {
        let mid = (min_obstacle + max_obstacle) / 2;
        let obstacles = obstacles.iter().take(mid).cloned().collect::<HashSet<_>>();
        match pathfinding(dims, start, end, &obstacles) {
            Some(_) => {min_obstacle = mid + 1;}
            None => {max_obstacle = mid;}
        }
    }

    format!("{},{}", obstacles[min_obstacle-1].0, obstacles[min_obstacle-1].1)
}

fn pathfinding(dims: (u32, u32), start: (u32, u32), end: (u32, u32), obstacles: &HashSet<(u32, u32)>) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, steps)) = queue.pop_front() {
        if pos == end {
            return Some(steps);
        }

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for dir in dirs.iter() {
            let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 > dims.0 as i32 || new_pos.1 > dims.1 as i32 {
                continue;
            }

            let new_pos = (new_pos.0 as u32, new_pos.1 as u32);
            if obstacles.contains(&new_pos) {
                continue;
            }

            queue.push_back((new_pos, steps + 1));
        }
    }

    None
}

