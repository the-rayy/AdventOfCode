use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("data/day25.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> i64 {
    let mut map = load(input);
    let maxx = map.keys().map(|(x, _)| *x).max().unwrap();
    let maxy = map.keys().map(|(_, y)| *y).max().unwrap();

    for i in 1.. {
        let (new_map, moved_east) = step_east(&map, maxx);
        let (new_map, moved_south) = step_south(&new_map, maxy);
        if !(moved_east || moved_south) {
            return i
        }
        map = new_map;
    }
    unreachable!();
}

fn load(input: &str) -> HashMap<(usize, usize), char> {
    let mut ret: HashMap<(usize, usize), char> = HashMap::new();

    for (line_idx, line) in input.split("\n").enumerate() {
        for (char_idx, c) in line.chars().enumerate() {
            if c != '.' {
                ret.insert((char_idx, line_idx), c);
            }
        }
    }

    ret
}

fn step_east(map: &HashMap<(usize, usize), char>, maxx: usize) -> (HashMap<(usize, usize), char>, bool) {
    let mut ret: HashMap<(usize, usize), char> = HashMap::new();
    let mut moved = false;

    for (&pos, &c) in map {
        if c != '>' {
            ret.insert(pos, c);
            continue
        }
        let new_pos = ((pos.0 + 1) % (maxx + 1), pos.1);

        match map.get(&new_pos) {
            None => {ret.insert(new_pos, c); moved = true;}
            Some(_) => {ret.insert(pos, c);}
        }
    }

    (ret, moved)
}

fn step_south(map: &HashMap<(usize, usize), char>, maxy: usize) -> (HashMap<(usize, usize), char>, bool) {
    let mut ret: HashMap<(usize, usize), char> = HashMap::new();
    let mut moved = false;

    for (&pos, &c) in map {
        if c != 'v' {
            ret.insert(pos, c);
            continue
        }
        let new_pos = (pos.0, (pos.1 + 1) % (maxy + 1));

        match map.get(&new_pos) {
            None => {ret.insert(new_pos, c); moved = true;}
            Some(_) => {ret.insert(pos, c);}
        }
    }

    (ret, moved)
}