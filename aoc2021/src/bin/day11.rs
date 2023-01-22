use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day11.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    for (i, line) in input.split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            map.insert((i as i32, j as i32), c.to_digit(10).unwrap());
        }
    }

    let mut flashed: i64 = 0;
    for _ in 0..100 {
        let ret = step(&map);
        map = ret.0;
        flashed += ret.1
    }
    flashed
}

fn part2(input: &str) -> i64 {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    for (i, line) in input.split("\n").enumerate() {
        for (j, c) in line.chars().enumerate() {
            map.insert((i as i32, j as i32), c.to_digit(10).unwrap());
        }
    }

    let mut curr_step: i64 = 0;
    loop {
        let ret = step(&map);
        map = ret.0;
        if ret.1 == map.len() as i64 {
            break
        }
        curr_step += 1;
    }

   curr_step + 1
}

fn step(map: &HashMap<(i32, i32), u32>) -> (HashMap<(i32, i32), u32>, i64) {
    //increment all
    let mut ret: HashMap<(i32, i32), u32> = map.iter()
        .map(|(pos, val)| (*pos, val+1))
        .collect();
    let keys: Vec<(i32, i32)> = ret.keys().map(|pos| *pos).collect();

    let mut flashed: HashSet<(i32, i32)> = HashSet::new();
    loop {
        let flashed_before = flashed.len();
        for &pos in &keys {
            if *ret.get(&pos).unwrap() > 9 as u32 && flashed.get(&pos).is_none() {
                flash(&mut ret, pos);
                flashed.insert(pos);
            }
        }
        if flashed_before == flashed.len() {
            break
        }
    }
    for &pos in &flashed {
        ret.insert(pos, 0);
    }

    (ret, flashed.len() as i64)
}

fn flash(map: &mut HashMap<(i32, i32), u32>, pos: (i32, i32)) {
    for x in (pos.0)-1..(pos.0)+2 {
        for y in (pos.1)-1..(pos.1)+2 {
            match map.get(&(x, y)) {
                None => {}
                Some(_) => { map.insert((x, y), *map.get(&(x, y)).unwrap() + 1); }
            }
        }
    }
}