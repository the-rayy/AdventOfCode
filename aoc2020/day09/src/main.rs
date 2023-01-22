use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let numbers = input.split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for i in 25 .. numbers.len() {
        if !is_valid(numbers[i], &numbers[i-25 .. i].to_vec()) {
            return numbers[i]
        }
    }
    unreachable!()
}

fn part2(input: &str) -> i64 {
    let numbers = input.split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let invalid_idx = (|| {
        for i in 25..numbers.len() {
            if !is_valid(numbers[i], &numbers[i - 25..i].to_vec()) {
                return i
            }
        }
        unreachable!()
    })();

    let mut sequences :HashMap<i32, (usize, usize)> = HashMap::new();
    for i in 0 .. invalid_idx {
        for j in i+1 .. invalid_idx {
            if numbers[i..=j].to_vec().iter().sum::<i64>() == numbers[invalid_idx] {
                sequences.insert((j-i) as i32, (i, j));
            }
        }
    }
    let seq_len = sequences.keys().max().unwrap();
    let seq = numbers[sequences.get(seq_len).unwrap().0 ..= sequences.get(seq_len).unwrap().1].to_vec();

    seq.iter().max().unwrap() + seq.iter().min().unwrap()
}

fn is_valid(number :i64, vec :&Vec<i64>) -> bool {
    vec.iter()
        .map(|x| number-x)
        .map(|diff| vec.contains(&diff) && diff != number)
        .filter(|is_valid| *is_valid)
        .count() > 0
}
