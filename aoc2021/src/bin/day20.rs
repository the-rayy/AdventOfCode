use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day20.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let (alg, mut map) = load(input);

    simulate(&alg, &mut map, 2);

    map.iter().filter(|(_, v)| **v == '1').count() as i64
}

fn part2(input: &str) -> i64 {
    let (alg, mut map) = load(input);

    simulate(&alg, &mut map, 50);

    map.iter().filter(|(_, v)| **v == '1').count() as i64
}

fn load(input: &str) -> (HashSet<u16>, HashMap<(i64, i64), char>) {
    let mut iter = input.split("\n\n");
    let alg: HashSet<u16> = iter.next().unwrap().chars().enumerate().filter(|(_, c)| *c == '#').map(|(i, _)| i as u16).collect();
    let mut map: HashMap<(i64, i64), char> = HashMap::new();
    for (row, line) in iter.next().unwrap().split("\n").enumerate() {
        for (column, c) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
            map.insert((column as i64, row as i64), if c == '#' { '1' } else { '0' });
        }
    }
    (alg, map)
}

fn simulate(alg: &HashSet<u16>, map: &mut HashMap<(i64, i64), char>, steps: usize) {
    for i in 0..steps {
        let border = if alg.get(&0).is_some() {
            if i % 2 == 0 { '0' } else { '1' }
        } else {
            '0'
        };
        *map = step(border, &alg, &map);
    }
}

fn step(border: char, alg: &HashSet<u16>, current: &HashMap<(i64, i64), char>) -> HashMap<(i64, i64), char> {
    let minx = current.keys().map(|x| x.0).min().unwrap();
    let maxx = current.keys().map(|x| x.0).max().unwrap();
    let miny = current.keys().map(|x| x.1).min().unwrap();
    let maxy = current.keys().map(|x| x.1).max().unwrap();

    let mut new_map: HashMap<(i64, i64), char> = HashMap::new();
    for x in minx-1..maxx+2 {
        for y in miny-1..maxy+2 {
            if enhance(border, alg, current, x, y) {
                new_map.insert((x, y), '1');
            } else {
                new_map.insert((x, y), '0');
            }
        }
    };
    new_map
}

fn enhance(border: char, alg: &HashSet<u16>, current: &HashMap<(i64, i64), char>, x: i64, y: i64) -> bool {
    let bin: String = [
        current.get(&(x - 1, y - 1)).or(Some(&border)).unwrap(),
        current.get(&(x, y - 1)).or(Some(&border)).unwrap(),
        current.get(&(x + 1, y - 1)).or(Some(&border)).unwrap(),
        current.get(&(x - 1, y)).or(Some(&border)).unwrap(),
        current.get(&(x, y)).or(Some(&border)).unwrap(),
        current.get(&(x + 1, y)).or(Some(&border)).unwrap(),
        current.get(&(x - 1, y + 1)).or(Some(&border)).unwrap(),
        current.get(&(x, y + 1)).or(Some(&border)).unwrap(),
        current.get(&(x + 1, y + 1)).or(Some(&border)).unwrap(),
    ].iter().cloned().collect();
    let intval = isize::from_str_radix(bin.as_str(), 2).unwrap();
    alg.get(&(intval as u16)).is_some()
}
