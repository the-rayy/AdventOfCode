use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = "hxbxwxba";

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> String {
    let mut input = input.chars()
        .rev()
        .map(|c| c as u32)
        .collect::<Vec<u32>>();

    while !valid(&input) {
        next(&mut input);
    }

    input.iter()
        .rev()
        .map(|&c| std::char::from_u32(c).unwrap())
        .collect()
}

fn part2(input: &str) -> String {
    let mut input = input.chars()
        .rev()
        .map(|c| c as u32)
        .collect::<Vec<u32>>();

    while !valid(&input) {
        next(&mut input);
    }

    next(&mut input);

    while !valid(&input) {
        next(&mut input);
    }

    input.iter()
        .rev()
        .map(|&c| std::char::from_u32(c).unwrap())
        .collect()
}

const I: u32 = 'i' as u32;
const O: u32 = 'o' as u32;
const L: u32 = 'l' as u32;

fn valid(input: &Vec<u32>) -> bool {
    if input.contains(&I) || input.contains(&O) || input.contains(&L) {
        return false
    }

    if !input.iter()
        .rev()
        .tuple_windows()
        .any(|(&a, &b, &c)| a+1 == b && b+1 == c) {
        return false
    }

    let mut pairs = 0;
    let mut foo = true;
    input.iter()
        .tuple_windows()
        .for_each(|(&a, &b)| {
            if a == b && foo {
                pairs += 1; foo = false;
            } else {
                foo = true;
            }
        });

    pairs >= 2
}

const A: u32 = 'a' as u32;
const Z: u32 = 'z' as u32;

fn next(input: &mut Vec<u32>) {
    for i in 0..input.len() {
        match input.get(i).unwrap() {
            &Z => {
                *input.get_mut(i).unwrap() = A
            },
            _ => {
                *input.get_mut(i).unwrap() += 1;
                break;
            }
        }
    }
}