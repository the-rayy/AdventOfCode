use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day15.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let map = load(input);

    dijkstra(&map)
}

fn part2(input: &str) -> i64 {
    let map = load(input);

    let mut new_map: HashMap<(i32, i32), u64> = HashMap::new();
    let end = (map.keys().map(|(x, _)| *x).max().unwrap(), map.keys().map(|(_, y)| *y).max().unwrap());
    for (point, val) in map {
        for i  in 0..5 {
            for j in 0..5 {
                let new_point = (
                    point.0 + i * (end.0+1),
                    point.1 + j * (end.1+1)
                );
                let new_val = wrap(val + i as u64 + j as u64);
                new_map.insert(new_point, new_val);
            }
        }
    }

    dijkstra(&new_map)
}

fn load(input: &str) -> HashMap<(i32, i32), u64> {
    let mut map: HashMap<(i32, i32), u64> = HashMap::new();
    for (i, line) in input.split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            map.insert((i as i32, j as i32), c.to_digit(10).unwrap() as u64);
        }
    }
    map
}

fn wrap(v: u64) -> u64 {
    return if v > 9 {
        wrap(v - 9)
    } else {
        v
    }
}

fn dijkstra(map: &HashMap<(i32, i32), u64>) -> i64 {
    let start = (0, 0);
    let end = (map.keys().map(|(x, _)| *x).max().unwrap(), map.keys().map(|(_, y)| *y).max().unwrap());

    let mut distances = BinaryHeap::new();
    distances.push(Reverse((0, start)));

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let min_val = loop {
        let (min_value, min_point) = distances.pop().unwrap().0;
        if min_point == end {
            break min_value
        }
        for (i, j) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_point = (min_point.0 + i, min_point.1 + j);
            match map.get(&new_point) {
                None => {}
                Some(val) => {
                    if !visited.contains(&new_point) {
                        distances.push(Reverse((min_value + *val, new_point)));
                        visited.insert(new_point);
                    }
                }
            }
        }
    };
    min_val as i64
}