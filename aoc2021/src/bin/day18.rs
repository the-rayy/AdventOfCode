use std::cmp::max;
use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day18.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Copy, Clone)]
enum Token<T> {
    Open,
    Close,
    Value(T)
}

fn part1(input: &str) -> i64 {
    let lines: Vec<_> = input.split("\n").map(|l| parse_line(l)).collect();
    let mut iter = lines.iter();
    let mut current = iter.next().unwrap().clone();
    while let Some(nxt) = iter.next() {
        current = add(&current, &nxt.clone());
        reduce(&mut current);
    };
    magnitude(&mut current) as i64
}

fn part2(input: &str) -> i64 {
    let lines: Vec<_> = input.split("\n").map(|l| parse_line(l)).collect();
    let mut max_magnitude: u32 = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue
            }
            let mut tokens = add(&lines[i], &lines[j]);
            reduce(&mut tokens);
            max_magnitude = max(max_magnitude, magnitude(&mut tokens));
        }
    };
    max_magnitude as i64
}

fn parse_line(line: &str) -> Vec<Token<u32>> {
    line.chars()
        .filter_map(|c| match c {
            '[' => {Some(Token::Open)},
            ']' => {Some(Token::Close)},
            _ => {c.to_digit(10).map(Token::Value)},
        })
        .collect()
}

fn add(left: &Vec<Token<u32>>, right: &Vec<Token<u32>>) -> Vec<Token<u32>> {
    let mut ret: Vec<Token<u32>> = Vec::new();
    ret.push(Token::Open);
    ret.extend(left.iter().cloned());
    ret.extend(right.iter().cloned());
    ret.push(Token::Close);

    ret
}

fn reduce(tokens: &mut Vec<Token<u32>>) {
    loop {
        let exploded = explode(tokens);
        if exploded {
            continue
        }
        let splitted = split(tokens);
        if !splitted {
            break
        }
    }
}

fn explode(tokens: &mut Vec<Token<u32>>) -> bool {
    for (idx, (t1, t2, t3, t4)) in tokens.iter().tuple_windows().enumerate() {
        let d = depth(&tokens[..idx]);
        if d != 4 {
            continue
        }
        // println!("i: {}, depth {}, ts: {:?} {:?} {:?} {:?}", idx, d, t1, t2, t3, t4);
        match (t1, t2, t3, t4) {
            (Token::Open, Token::Value(left), Token::Value(right), Token::Close) => {
                let left = (*left).clone();
                let right = (*right).clone();
                explode_left(tokens, idx, left);
                explode_right(tokens, idx, right);
                tokens[idx] = Token::Value(0);
                for _ in idx+1..idx+4 { tokens.remove(idx+1); }
                return true
            }
            _ => {
                continue
            }
        };
    }
    return false
}

fn split(tokens: &mut Vec<Token<u32>>) -> bool {
    for (idx, token) in tokens.iter().enumerate() {
        match token {
            Token::Value(value) => {
                if *value < 10 {
                    continue
                }
                let value = (*value).clone() as f64;
                let left = (value / 2f64).floor() as u32;
                let right = (value / 2f64).ceil() as u32;
                tokens[idx] = Token::Open;
                tokens.insert(idx + 1, Token::Value(left));
                tokens.insert(idx + 2, Token::Value(right));
                tokens.insert(idx + 3, Token::Close);
                return true
            }
            _ => {
                continue
            }
        };
    }
    return false
}

fn magnitude(tokens: &mut Vec<Token<u32>>) -> u32 {
    while tokens.len() > 1 {
        collapse(tokens);
    };
    match tokens[0] {
        Token::Value(x) => {x},
        _ => unreachable!()
    }
}

fn collapse(tokens: &mut Vec<Token<u32>>) -> bool {
    for (idx, (t1, t2, t3, t4)) in tokens.iter().tuple_windows().enumerate() {
        match (t1, t2, t3, t4) {
            (Token::Open, Token::Value(left), Token::Value(right), Token::Close) => {
                let left = (*left).clone();
                let right = (*right).clone();
                let magnitude = 3 * left + 2 * right;
                tokens[idx] = Token::Value(magnitude);
                for _ in idx+1..idx+4 { tokens.remove(idx+1); }
                return true
            }
            _ => {
                continue
            }
        };
    }
    return false
}


fn depth(tokens: &[Token<u32>]) -> usize {
    tokens.iter().fold(0, |acc, t| match *t {
        Token::Open => {acc + 1},
        Token::Close => {acc - 1},
        _ => {acc}
    }) as usize
}

fn explode_left(tokens: &mut Vec<Token<u32>>, idx: usize, val_to_add: u32) {
    for i in (0..idx).rev() {
        match tokens[i] {
            Token::Value(old_val) => {
                tokens[i] = Token::Value(old_val + val_to_add);
                return;
            }
            _ => {}
        }
    }
}

fn explode_right(tokens: &mut Vec<Token<u32>>, idx: usize, val_to_add: u32) {
    for i in idx+4..tokens.len() {
        match tokens[i] {
            Token::Value(old_val) => {
                tokens[i] = Token::Value(old_val + val_to_add);
                return;
            }
            _ => {}
        }
    }
}