use std::fs;

fn main() {
    let input = fs::read_to_string("data/day10.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    input.split("\n")
        .map(|line| get_first_illegal_char(line))
        .filter(|&c| c != ' ')
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!()
        })
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    let mut scores = input.split("\n")
        .map(|line| (line, get_first_illegal_char(line)))
        .filter(|(_, c)| *c == ' ')
        .map(|(line, _)| get_remainder(line))
        .map(|remainder| score_remainder(&remainder))
        .collect::<Vec<i64>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn get_first_illegal_char(line: &str) -> char {
    let left_chars: Vec<char> = vec!['(', '[', '{', '<'];
    let right_chars: Vec<char> = vec![')', ']', '}', '>'];
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        if left_chars.contains(&c) {
            stack.push(c);
            continue
        }
        let idx = right_chars.iter().position(|&ch| ch == c).unwrap();
        if *stack.last().unwrap() == left_chars[idx] {
            stack.pop();
            continue
        }
        return c
    }
    return ' '
}

fn get_remainder(line: &str) -> Vec<char> {
    let left_chars: Vec<char> = vec!['(', '[', '{', '<'];
    let right_chars: Vec<char> = vec![')', ']', '}', '>'];
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        if left_chars.contains(&c) {
            stack.push(c);
            continue
        }
        let idx = right_chars.iter().position(|&ch| ch == c).unwrap();
        if *stack.last().unwrap() == left_chars[idx] {
            stack.pop();
            continue
        }
        unreachable!();
    }

    stack
}

fn score_remainder(remainder: &Vec<char>) -> i64 {
    let mut score: i64 = 0;
    remainder.iter()
        .rev()
        .map(|&c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!()
        })
        .for_each(|points| {
            score *= 5;
            score += points;
        });

    score
}