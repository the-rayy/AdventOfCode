use std::fs;
use std::time::Instant;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("data/day15.txt").expect("Unable to load input file");

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
    let mut input_split = input.split("\n\n");

    let mut grid: HashMap<(i32, i32), char> = input_split
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .flatten()
        .collect();

    let mut robot = grid
        .iter()
        .filter(|(_, v)| **v == '@')
        .next()
        .unwrap()
        .0
        .clone();
    grid.remove(&robot);

    for dir in input_split.next().unwrap().replace("\n", "").chars() {
        let dir = match dir {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        let new_pos = (robot.0 + dir.0, robot.1 + dir.1);
        match grid.get(&new_pos) {
            None => robot = new_pos,
            Some('#') => (),
            Some('O') => {
                if shift(&mut grid, new_pos, dir) {
                    robot = new_pos;
                }
            }
            Some(_) => unreachable!(),
        }
    }

    grid.iter()
        .filter(|(_, v)| **v == 'O')
        .map(|(k, _)| (100 * k.0 + k.1) as u32)
        .sum::<u32>()
}

fn shift(grid: &mut HashMap<(i32, i32), char>, start: (i32, i32), dir: (i32, i32)) -> bool {
    let mut new_pos = start;
    loop {
        new_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
        match grid.get(&new_pos) {
            None => {
                break;
            }
            Some('#') => return false,
            Some(_) => {
                continue;
            }
        }
    }

    grid.insert(new_pos, 'O');
    grid.remove(&start);
    true
}

fn part2(input: &str) -> u32 {
    let mut input_split = input.split("\n\n");

    let mut grid: HashMap<(i32, i32), char> = input_split
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(j, c)| {
                    let pos1 = (i as i32, 2 * j as i32);
                    let pos2 = (i as i32, 2 * j as i32 + 1);
                    match c {
                        '#' => vec![(pos1, '#'), (pos2, '#')],
                        'O' => vec![(pos1, '['), (pos2, ']')],
                        '@' => vec![(pos1, '@')],
                        _ => unreachable!(),
                    }
                })
                .flatten()
        })
        .flatten()
        .collect();

    let mut robot = grid
        .iter()
        .filter(|(_, v)| **v == '@')
        .next()
        .unwrap()
        .0
        .clone();
    grid.remove(&robot);

    for dir in input_split.next().unwrap().replace("\n", "").chars() {
        let dir = match dir {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!(),
        };

        let new_pos = (robot.0 + dir.0, robot.1 + dir.1);
        match grid.get(&new_pos) {
            None => robot = new_pos,
            Some('#') => (),
            Some('[') | Some(']') => {
                if wide_shift(&mut grid, new_pos, dir) {
                    robot = new_pos;
                }
            }
            Some(_) => unreachable!(),
        }
    }

    grid.iter()
        .filter(|(_, v)| **v == '[')
        .map(|(k, _)| (100 * k.0 + k.1) as u32)
        .sum::<u32>()
}

fn wide_shift(grid: &mut HashMap<(i32, i32), char>, start: (i32, i32), dir: (i32, i32)) -> bool {
    if dir.0 == 0 {
        wide_shift_horizontal(grid, start, dir)
    } else {
        wide_shift_vertical(grid, start, dir)
    }
}

fn wide_shift_horizontal(
    grid: &mut HashMap<(i32, i32), char>,
    start: (i32, i32),
    dir: (i32, i32),
) -> bool {
    let mut new_pos = start;
    loop {
        new_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
        match grid.get(&new_pos) {
            None => {
                break;
            }
            Some('#') => return false,
            Some(_) => {
                continue;
            }
        }
    }

    let end = (new_pos.0 + 2 * dir.0, new_pos.1 + 2 * dir.1);
    grid.remove(&start);
    let mut new_pos1 = (start.0 + dir.0, start.1 + dir.1);
    let mut new_pos2 = (start.0 + 2 * dir.0, start.1 + 2 * dir.1);
    while new_pos2 != end {
        if dir.1 == 1 {
            grid.insert(new_pos1, '[');
            grid.insert(new_pos2, ']');
        } else {
            grid.insert(new_pos1, ']');
            grid.insert(new_pos2, '[');
        }
        new_pos1 = (new_pos1.0 + 2 * dir.0, new_pos1.1 + 2 * dir.1);
        new_pos2 = (new_pos2.0 + 2 * dir.0, new_pos2.1 + 2 * dir.1);
    }
    true
}

fn wide_shift_vertical(
    grid: &mut HashMap<(i32, i32), char>,
    start: (i32, i32),
    dir: (i32, i32),
) -> bool {
    if wide_shift_vertical_check(grid, start, dir) {
        wide_shift_vertical_move(grid, start, dir);
        return true;
    }

    false
}

fn wide_shift_vertical_check(
    grid: &HashMap<(i32, i32), char>,
    start: (i32, i32),
    dir: (i32, i32),
) -> bool {
    let neighbour = match grid.get(&start) {
        Some('[') => (start.0, start.1 + 1),
        Some(']') => (start.0, start.1 - 1),
        _ => unreachable!(),
    };

    let new_pos = (start.0 + dir.0, start.1 + dir.1);
    let neighbour_new_pos = (neighbour.0 + dir.0, neighbour.1 + dir.1);

    let can_move = match grid.get(&new_pos) {
        None => true,
        Some('#') => false,
        Some('[') | Some(']') => wide_shift_vertical_check(grid, new_pos, dir),
        _ => unreachable!(),
    };

    let neighbour_can_move = match grid.get(&neighbour_new_pos) {
        None => true,
        Some('#') => false,
        Some('[') | Some(']') => wide_shift_vertical_check(grid, neighbour_new_pos, dir),
        _ => unreachable!(),
    };

    can_move && neighbour_can_move
}

fn wide_shift_vertical_move(
    grid: &mut HashMap<(i32, i32), char>,
    start: (i32, i32),
    dir: (i32, i32),
) {
    let neighbour = match grid.get(&start) {
        Some('[') => (start.0, start.1 + 1),
        Some(']') => (start.0, start.1 - 1),
        _ => unreachable!(),
    };

    let new_pos = (start.0 + dir.0, start.1 + dir.1);
    let neighbour_new_pos = (neighbour.0 + dir.0, neighbour.1 + dir.1);

    if let Some(&c) = grid.get(&new_pos) {
        if c == '[' || c == ']' {
            wide_shift_vertical_move(grid, new_pos, dir);
        }
    }

    if let Some(&c) = grid.get(&neighbour_new_pos) {
        if c == '[' || c == ']' {
            wide_shift_vertical_move(grid, neighbour_new_pos, dir);
        }
    }

    let c = grid.get(&start).unwrap().clone();
    grid.remove(&start);
    grid.insert(new_pos, c);

    let c = grid.get(&neighbour).unwrap().clone();
    grid.remove(&neighbour);
    grid.insert(neighbour_new_pos, c);
}
