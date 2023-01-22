use std::collections::HashMap;
use std::fs;
use itertools::{max, min};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day05.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let rg = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)").unwrap();
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    input.split("\n")
        .map(|line| {
            let x = rg.captures(line).unwrap();
            (x[1].parse::<i32>().unwrap(), x[2].parse::<i32>().unwrap(), x[3].parse::<i32>().unwrap(), x[4].parse::<i32>().unwrap())
        })
        .filter(|(x1, y1, x2, y2)| is_hv(*x1, *y1, *x2, *y2))
        .for_each(|(x1, y1, x2, y2)| {
            line_hv(x1, y1, x2, y2).iter()
                .for_each(|(x, y)| *map.entry((*x, *y)).or_insert(0) += 1)
        });
    map.iter()
        .filter(|(_, v)| **v >= 2)
        .count() as i32
}

fn part2(input: &str) -> i32 {
    let rg = Regex::new(r"(\d*),(\d*) -> (\d*),(\d*)").unwrap();
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    //vertical or horizontal
    input.split("\n")
        .map(|line| {
            let x = rg.captures(line).unwrap();
            (x[1].parse::<i32>().unwrap(), x[2].parse::<i32>().unwrap(), x[3].parse::<i32>().unwrap(), x[4].parse::<i32>().unwrap())
        })
        .filter(|(x1, y1, x2, y2)| is_hv(*x1, *y1, *x2, *y2))
        .for_each(|(x1, y1, x2, y2)| {
            line_hv(x1, y1, x2, y2).iter()
                .for_each(|(x, y)| *map.entry((*x, *y)).or_insert(0) += 1)
        });

    //diagonal
    input.split("\n")
        .map(|line| {
            let x = rg.captures(line).unwrap();
            (x[1].parse::<i32>().unwrap(), x[2].parse::<i32>().unwrap(), x[3].parse::<i32>().unwrap(), x[4].parse::<i32>().unwrap())
        })
        .filter(|(x1, y1, x2, y2)| is_diagonal(*x1, *y1, *x2, *y2))
        .for_each(|(x1, y1, x2, y2)| {
            line_diag(x1, y1, x2, y2).iter()
                .for_each(|(x, y)| *map.entry((*x, *y)).or_insert(0) += 1)
        });

    map.iter()
        .filter(|(_, v)| **v >= 2)
        .count() as i32
}

fn is_hv(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    (x1 == x2) || (y1 == y2)
}

fn is_diagonal(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    if x1 == x2 {
        return false
    }
    let a = (y2 - y1) / (x2 - x1);
    (a == 1) || (a == -1)
}

fn line_hv(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut ret: Vec<(i32, i32)> = vec![];
    let x_min: i32 = min(vec![x1, x2]).unwrap();
    let x_max: i32 = max(vec![x1, x2]).unwrap();
    let y_min: i32 = min(vec![y1, y2]).unwrap();
    let y_max: i32 = max(vec![y1, y2]).unwrap();
    for i in x_min..x_max+1 {
        for j in y_min..y_max+1 {
            ret.push((i, j));
        }
    }

    ret
}

fn line_diag(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut ret: Vec<(i32, i32)> = vec![];

    let sign_x = if x1 < x2 {1} else {-1};
    let sign_y = if y1 < y2 {1} else {-1};

    let mut i: i32 = 0;
    loop {
        let x = x1 + (sign_x * i);
        let y = y1 + (sign_y * i);
        ret.push((x, y));
        if x == x2 || y == y2 {
            break;
        }
        i = i + 1;
    }

    ret
}