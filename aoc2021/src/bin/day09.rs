use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day09.txt")
        .expect("Unable to load input file");
    // println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    input.split("\n")
        .enumerate()
        .for_each(|(i, line)| line.chars()
            .enumerate()
            .for_each(|(j, c)| {
                map.insert((i as i32, j as i32), c.to_digit(10).unwrap());
            })
        );

    map.iter()
        .filter(|((i, j), _)| is_low_point(*i, *j, &map))
        .map(|(_, point)| *point + 1)
        .sum::<u32>() as i64
}

fn part2(input: &str) -> i64 {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    input.split("\n")
        .enumerate()
        .for_each(|(i, line)| line.chars()
            .enumerate()
            .for_each(|(j, c)| {
                map.insert((i as i32, j as i32), c.to_digit(10).unwrap());
            })
        );

    let mut score: i64 = 1;
    map.iter()
        .filter(|((i, j), _)| is_low_point(*i, *j, &map))
        .map(|((i, j), _)| find_basin(*i, *j, &map))
        .map(|hs| hs.len() as i64)
        .sorted()
        .rev()
        .take(3)
        .for_each(|x| score = score * x);

    score
}

fn is_low_point(i: i32, j: i32, map: &HashMap<(i32, i32), u32>) -> bool {
    let point = map.get(&(i, j)).unwrap();
    let upper = map.get(&(i-1, j)).unwrap_or(&999);
    let lower = map.get(&(i+1, j)).unwrap_or(&999);
    let left = map.get(&(i, j-1)).unwrap_or(&999);
    let right = map.get(&(i, j+1)).unwrap_or(&999);

    point < upper && point < lower && point < left && point < right
}

fn find_basin(i: i32, j: i32, map: &HashMap<(i32, i32), u32>) -> HashSet<(i32, i32)> {
    let mut basin: HashSet<(i32, i32)> = HashSet::new();
    let mut old_basin_count = basin.iter().count();
    basin.insert((i, j));

    while old_basin_count != basin.iter().count() {
        let mut new_basin: HashSet<(i32, i32)> = HashSet::new();
        old_basin_count = basin.iter().count();
        for (ii, jj) in &basin {
            let point = map.get(&(*ii, *jj)).unwrap();
            let upper = map.get(&(*ii - 1, *jj)).unwrap_or(&999);
            if point < upper && upper < &9 {
                new_basin.insert((*ii - 1, *jj));
            }
            let lower = map.get(&(*ii + 1, *jj)).unwrap_or(&999);
            if point < lower && lower < &9 {
                new_basin.insert((*ii + 1, *jj));
            }
            let left = map.get(&(*ii, *jj - 1)).unwrap_or(&999);
            if point < left && left < &9 {
                new_basin.insert((*ii, jj - 1));
            }
            let right = map.get(&(*ii, *jj + 1)).unwrap_or(&999);
            if point < right && right < &9 {
                new_basin.insert((*ii, *jj + 1));
            }
        }
        basin.extend(&new_basin);
    };
    basin
}