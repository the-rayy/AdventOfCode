use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let time :i32 = input.split("\n")
        .collect::<Vec<&str>>()[0]
        .parse::<i32>()
        .unwrap();
    let buses = input.split("\n")
        .collect::<Vec<&str>>()[1]
        .split(",")
        .filter(|x| *x != "x")
        .map(|x| x.parse::<i32>().unwrap())
        .map(|x| (x, x - (time % x)))
        .collect::<HashMap<i32, i32>>();

    let mut min_idx :i32 = -1;
    let mut min_val :i32 = i32::MAX;
    buses.iter()
        .for_each(|(key, val)| {
            if val < &min_val {
                min_val = *val;
                min_idx = *key;
            }
    });


    min_idx * min_val
}

fn part2(input: &str) -> i64 {
    let buses = input.split("\n")
        .collect::<Vec<&str>>()[1]
        .split(",")
        .map(|x| x.parse::<i64>().unwrap_or(1))
        .collect::<Vec<i64>>();

    let res = buses.iter()
        .enumerate()
        .map(|(i,b)| *b - i as i64)
        .collect::<Vec<i64>>();

    chinese_remainder(&res, &buses).unwrap()
}

// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}