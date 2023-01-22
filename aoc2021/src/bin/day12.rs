use std::collections::{HashMap};
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day12.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.split("\n") {
        let mut split = line.split("-");
        let cave1 = split.next().unwrap();
        let cave2 = split.next().unwrap();
        if cave2 != "start" {
            match map.get_mut(cave1) {
                None => { map.insert(cave1, vec![cave2]); },
                Some(v) => v.push(cave2),
            }
        }
        if cave1 != "start"  && cave2 != "end" {
            match map.get_mut(cave2) {
                None => { map.insert(cave2, vec![cave1]); },
                Some(v) => v.push(cave1),
            }
        }
    }

    let mut paths: Vec<Vec<&str>> = vec![vec!["start"]];

    loop {
        let mut new_paths: Vec<Vec<&str>> = Vec::new();
        for path in paths.clone() {
            let mut discovered_paths = step_1(path, &map);
            new_paths.append(&mut discovered_paths);
        }
        if new_paths.len() == paths.len() {
            break
        }
        paths = new_paths;
    }

    paths.len() as i64
}

fn part2(input: &str) -> i64 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.split("\n") {
        let mut split = line.split("-");
        let cave1 = split.next().unwrap();
        let cave2 = split.next().unwrap();
        if cave2 != "start" {
            match map.get_mut(cave1) {
                None => { map.insert(cave1, vec![cave2]); },
                Some(v) => v.push(cave2),
            }
        }
        if cave1 != "start"  && cave2 != "end" {
            match map.get_mut(cave2) {
                None => { map.insert(cave2, vec![cave1]); },
                Some(v) => v.push(cave1),
            }
        }
    }

    let mut paths: Vec<Vec<&str>> = vec![vec!["start"]];

    loop {
        let mut new_paths: Vec<Vec<&str>> = Vec::new();
        for path in paths.clone() {
            let mut discovered_paths = step_2(path, &map);
            new_paths.append(&mut discovered_paths);
        }
        if new_paths.len() == paths.len() {
            break
        }
        paths = new_paths;
    }

    paths.len() as i64
}

fn step_1<'a>(path: Vec<&'a str>, map: &'a HashMap<&str, Vec<&str>>) -> Vec<Vec<&'a str>> {
    let last_cave = path.last().unwrap().clone();
    if last_cave == "end" {
        return vec![path]
    };
    if is_invalid_1(&path) {
        return vec![]
    }

    let mut new_paths: Vec<Vec<&str>> = Vec::new();
    let next_caves = map.get(last_cave).unwrap();
    for &cave in next_caves {
        let mut new_path = path.iter()
            .map(|e| e.clone())
            .collect::<Vec<&str>>();
        new_path.push(cave);
        new_paths.push(new_path);
    };

    new_paths
}

fn is_invalid_1(path: &Vec<&str>) -> bool {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &x in path {
        *counts.entry(x).or_insert(0) += 1;
    }

    counts.iter()
        .filter(|(&key, &value)| key.to_lowercase() == key && value > 1)
        .count() > 0
}


fn step_2<'a>(path: Vec<&'a str>, map: &'a HashMap<&str, Vec<&str>>) -> Vec<Vec<&'a str>> {
    let last_cave = path.last().unwrap().clone();
    if is_invalid_2(&path) {
        return vec![]
    };
    if last_cave == "end" {
        return vec![path]
    };

    let mut new_paths: Vec<Vec<&str>> = Vec::new();
    let next_caves = map.get(last_cave).unwrap();
    for &cave in next_caves {
        let mut new_path = path.iter()
            .map(|e| e.clone())
            .collect::<Vec<&str>>();
        new_path.push(cave);
        new_paths.push(new_path);
    };

    new_paths
}

fn is_invalid_2(path: &Vec<&str>) -> bool {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &x in path {
        *counts.entry(x).or_insert(0) += 1;
    }

    let mut counts = counts.iter()
        .filter(|(&key, _)| key.to_lowercase() == key)
        .map(|(_, &count)| count)
        .collect::<Vec<usize>>();

    counts.sort();
    if counts.pop().unwrap() > 2 {
        return true
    };

    return counts.len() != counts.iter().sum::<usize>();
}
