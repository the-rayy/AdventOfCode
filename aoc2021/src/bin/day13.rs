use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day13.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    part2(&input); // do the OCR part yourself ;)
}

fn part1(input: &str) -> i64 {
    let (map, folds) = load(input);
    fold(&map, folds[0]).len() as i64
}

fn part2(input: &str) {
    let (mut map, folds) = load(input);

    for fold_point in folds {
        map = fold(&map, fold_point)
    };

    display_map(&map);
}

fn load(input: &str) -> (HashSet<(usize, usize)>, Vec<(char, usize)>) {
    let mut lines = input.split("\n");
    let mut map: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let next_line = lines.next().unwrap();
        if next_line == "" {
            break
        }
        let mut splitted = next_line.split(",");
        map.insert((
            splitted.next().unwrap().parse::<usize>().unwrap(),
            splitted.next().unwrap().parse::<usize>().unwrap()
        ));
    }
    let mut folds: Vec<(char, usize)> = Vec::new();
    loop {
        match lines.next() {
            None => break,
            Some(line) => {
                let mut splitted = line.split("=");
                folds.push((
                    splitted.next().unwrap().chars().last().unwrap(),
                    splitted.next().unwrap().parse::<usize>().unwrap()
                ));
            }
        }
    }
    (map, folds)
}

fn fold(map: &HashSet<(usize, usize)>, fold_line: (char, usize)) -> HashSet<(usize, usize)> {
    return if fold_line.0 == 'y' {
        fold_y(map, fold_line.1)
    } else {
        fold_x(map, fold_line.1)
    }
}

fn fold_y(map: &HashSet<(usize, usize)>, fold_point: usize) -> HashSet<(usize, usize)> {
    let mut new_map: HashSet<(usize, usize)> = HashSet::new();
    for point in map {
        match point.1.partial_cmp(&fold_point).unwrap() {
            Ordering::Less => {new_map.insert(*point);}
            Ordering::Equal => {}
            Ordering::Greater => {
                let new_point = (point.0, fold_point - (point.1 - fold_point));
                new_map.insert(new_point);
            }
        }
    }

    new_map
}

fn fold_x(map: &HashSet<(usize, usize)>, fold_point: usize) -> HashSet<(usize, usize)> {
    let mut new_map: HashSet<(usize, usize)> = HashSet::new();
    for point in map {
        match point.0.partial_cmp(&fold_point).unwrap() {
            Ordering::Less => {new_map.insert(*point);}
            Ordering::Equal => {}
            Ordering::Greater => {
                let new_point = (fold_point - (point.0 - fold_point), point.1);
                new_map.insert(new_point);
            }
        }
    }

    new_map
}

fn display_map(map: &HashSet<(usize, usize)>) {
    let max_x = map.iter()
        .map(|point| point.0)
        .max()
        .unwrap();
    let max_y = map.iter()
        .map(|point| point.1)
        .max()
        .unwrap();

    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            let sign = match map.contains(&(x, y)) {
                true => {'#'}
                false => {' '}
            };
            print!("{}", sign);
        }
        print!("\n");
    }
}
