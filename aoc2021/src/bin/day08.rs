use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day08.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let correct_digits_lengths: Vec<usize> = vec![2, 4, 3, 7];
    input.split("\n")
        .map(|line| line.split(" | ").last().unwrap())
        .map(|code| code.split(" ").collect::<Vec<&str>>())
        .map(|digits| digits.iter()
            .map(|digit| digit.len())
            .filter(|digit_len| correct_digits_lengths.contains(digit_len))
            .count())
        .sum::<usize>() as i64
}

fn part2(input: &str) -> i64 {
    input.split("\n")
        .map(|line| decode_line(line))
        .sum()
}

fn decode_line(line: &str) -> i64 {
    let code = line.split(" | ").next().unwrap();
    let code = code.split(" ")
        .map(|c| c.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();
    let output = line.split(" | ").last().unwrap();
    let output = output.split(" ")
        .map(|c| c.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    let mut facts: Vec<HashSet<char>> = vec![HashSet::new(); 10];
    for c in code {
        match c.len() {
            2 => facts[1] = c,
            4 => facts[4] = c,
            3 => facts[7] = c,
            7 => facts[8] = c,
            _ => ()
        };
    }

    output.iter()
        .map(|digit| decode_digit(digit, &facts))
        .map(|d| d.to_string())
        .join("")
        .parse::<i64>()
        .unwrap()
}

fn decode_digit(digit: &HashSet<char>, facts: &Vec<HashSet<char>>) -> i32 {
    for (i, fact) in facts.iter().enumerate() {
        if digit == fact {
            return i as i32
        }
    };
    if digit.len() == 5 && digit.is_superset(&&facts[7]) {
        return 3
    }
    if digit.len() == 6 && !digit.is_superset(&&facts[7]) {
        return 6
    }
    if digit.len() == 5 && digit.intersection(&&facts[4]).count() == 3 {
        return 5
    }
    if digit.len() == 5 && digit.intersection(&&facts[4]).count() == 2 {
        return 2
    }
    if digit.len() == 6 && digit.is_superset(&&facts[4]) {
        return 9
    }
    if digit.len() == 6 {
        return 0
    }

    unreachable!()
}