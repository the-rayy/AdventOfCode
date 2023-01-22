use std::fs;
use std::collections::HashMap;

const EAST :char = 'E';
const WEST :char = 'W';
const NORTH :char = 'N';
const SOUTH :char = 'S';
const DIRS :[char;4] = [NORTH, EAST, SOUTH, WEST];
const LEFT :char = 'L';
const RIGHT :char = 'R';
const FORWARD :char = 'F';

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let instructions = input.split("\n")
        .map(|line| parse(line))
        .collect::<Vec<(char, i32)>>();

    let mut dir  = 1;
    let mut history :HashMap<char, i32> = HashMap::new();
    for (instr, val) in instructions.iter() {
        match *instr {
            FORWARD => *history.entry(DIRS[dir as usize]).or_insert(0) += val,
            RIGHT => dir = (dir + val/90) % 4,
            LEFT => dir = (dir + (360-val)/90) % 4,
            NORTH => *history.entry(NORTH).or_insert(0) += val,
            SOUTH => *history.entry(SOUTH).or_insert(0) += val,
            EAST => *history.entry(EAST).or_insert(0) += val,
            WEST => *history.entry(WEST).or_insert(0) += val,
            _ => unreachable!()
        }
    }
    let ns = history.get(&NORTH).unwrap() - history.get(&SOUTH).unwrap();
    let ew = history.get(&EAST).unwrap() - history.get(&WEST).unwrap();
    ns.abs() + ew.abs()
}

fn part2(input: &str) -> i32 {
    let instructions = input.split("\n")
        .map(|line| parse(line))
        .collect::<Vec<(char, i32)>>();

    let mut history :HashMap<char, i32> = HashMap::new();
    let mut waypoint :HashMap<char, i32> = HashMap::new();
    waypoint.insert(EAST, 10);
    waypoint.insert(NORTH, 1);
    for (instr, val) in instructions.iter() {
        match *instr {
            FORWARD => waypoint.iter()
                .for_each(|(dir, len)| *history.entry(*dir).or_insert(0) += val*len),
            RIGHT => for _ in 0 .. val/90 {
                waypoint = rotate_right(&waypoint);
            },
            LEFT => for _ in 0 .. val/90 {
                waypoint = rotate_left(&waypoint);
            },
            NORTH => *waypoint.entry(NORTH).or_insert(0) += val,
            SOUTH => *waypoint.entry(SOUTH).or_insert(0) += val,
            EAST => *waypoint.entry(EAST).or_insert(0) += val,
            WEST => *waypoint.entry(WEST).or_insert(0) += val,
            _ => unreachable!()
        }
    }
    let ns = history.get(&NORTH).unwrap() - history.get(&SOUTH).unwrap();
    let ew = history.get(&EAST).unwrap() - history.get(&WEST).unwrap();
    ns.abs() + ew.abs()
}

fn rotate_right(source :&HashMap<char, i32>) -> HashMap<char, i32> {
    let mut ret :HashMap<char, i32> = HashMap::new();
    ret.insert(EAST, *source.get(&NORTH).unwrap());
    ret.insert(SOUTH, *source.get(&EAST).unwrap());
    ret.insert(WEST, *source.get(&SOUTH).unwrap());
    ret.insert(NORTH, *source.get(&WEST).unwrap());

    ret
}

fn rotate_left(source :&HashMap<char, i32>) -> HashMap<char, i32> {
    let mut ret :HashMap<char, i32> = HashMap::new();
    ret.insert(NORTH, *source.get(&EAST).unwrap());
    ret.insert(WEST, *source.get(&NORTH).unwrap());
    ret.insert(SOUTH, *source.get(&WEST).unwrap());
    ret.insert(EAST, *source.get(&SOUTH).unwrap());

    ret
}

fn parse(line :&str) -> (char, i32) {
    let ch = *line.chars()
        .collect::<Vec<char>>()
        .first()
        .unwrap();
    let num = line.chars()
        .collect::<Vec<char>>()[1..line.len()]
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    (ch, num)
}