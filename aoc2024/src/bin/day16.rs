use std::time::Instant;
use std::{collections::VecDeque, fs};

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day16.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input, part1_ans);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let grid: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .flatten()
        .collect();

    let start = grid.iter().find(|(_, v)| **v == 'S').unwrap().0.clone();
    let end = grid.iter().find(|(_, v)| **v == 'E').unwrap().0.clone();

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    queue.push_back((start, 0, 0));
    visited.insert((start, 0), 0);

    while let Some((pos, dir, score)) = queue.pop_front() {
        //forward
        let new_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        if let Some(x) = grid.get(&new_pos) {
            if *x != '#'
                && (visited.get(&(new_pos, dir)).is_none()
                    || score + 1 < *visited.get(&(new_pos, dir)).unwrap())
            {
                visited.insert((new_pos, dir), score + 1);
                queue.push_back((new_pos, dir, score + 1));
            }
        }

        //left
        let new_dir = (dir - 1).rem_euclid(4);
        if visited.get(&(pos, new_dir)).is_none()
            || score + 1000 < *visited.get(&(pos, new_dir)).unwrap()
        {
            visited.insert((pos, new_dir), score + 1000);
            queue.push_back((pos, new_dir, score + 1000));
        }

        //right
        let new_dir = (dir + 1).rem_euclid(4);
        if visited.get(&(pos, new_dir)).is_none()
            || score + 1000 < *visited.get(&(pos, new_dir)).unwrap()
        {
            visited.insert((pos, new_dir), score + 1000);
            queue.push_back((pos, new_dir, score + 1000));
        }
    }

    visited
        .iter()
        .filter(|((pos, _), _)| *pos == end)
        .map(|(_, score)| *score)
        .min()
        .unwrap()
}

fn part2(input: &str, shortest_path_score: u32) -> u32 {
    let grid: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .flatten()
        .collect();

    let start = grid.iter().find(|(_, v)| **v == 'S').unwrap().0.clone();
    let end = grid.iter().find(|(_, v)| **v == 'E').unwrap().0.clone();

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    queue.push_back((start, 0, 0, vec![start]));
    visited.insert((start, 0), 0);

    let mut places = HashSet::new();
    while let Some((pos, dir, score, path)) = queue.pop_front() {
        if score > shortest_path_score {
            continue;
        }

        if score == shortest_path_score && pos == end {
            places.extend(path);
            continue;
        }

        //forward
        let new_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
        if let Some(x) = grid.get(&new_pos) {
            if *x != '#'
                && (visited.get(&(new_pos, dir)).is_none()
                    || score + 1 <= *visited.get(&(new_pos, dir)).unwrap())
            {
                let mut path = path.clone();
                path.push(new_pos);

                visited.insert((new_pos, dir), score + 1);
                queue.push_back((new_pos, dir, score + 1, path));
            }
        }

        //left
        let new_dir = (dir - 1).rem_euclid(4);
        if visited.get(&(pos, new_dir)).is_none()
            || score + 1000 <= *visited.get(&(pos, new_dir)).unwrap()
        {
            visited.insert((pos, new_dir), score + 1000);
            queue.push_back((pos, new_dir, score + 1000, path.clone()));
        }

        //right
        let new_dir = (dir + 1).rem_euclid(4);
        if visited.get(&(pos, new_dir)).is_none()
            || score + 1000 <= *visited.get(&(pos, new_dir)).unwrap()
        {
            visited.insert((pos, new_dir), score + 1000);
            queue.push_back((pos, new_dir, score + 1000, path.clone()));
        }
    }

    places.len() as u32
}
