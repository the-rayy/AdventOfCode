use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day14.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let (mut template, last_char, rules) = load(input);

    for _ in 0..10 {
        template = step(&template, &rules);
    }

    let counts = calc_counts(&template, last_char);
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part2(input: &str) -> i64 {
    let (mut template, last_char, rules) = load(input);

    for _ in 0..40 {
        template = step(&template, &rules);
    }

    let counts = calc_counts(&template, last_char);
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn load(input: &str) -> (HashMap<(char, char), i64>, char, HashMap<(char, char), char>) {
    let mut lines = input.split("\n");
    let mut template: HashMap<(char, char), i64> = HashMap::new();
    let mut last_char: char = 'x';
    for (e1, e2) in lines.next().unwrap().chars().tuple_windows() {
        *template.entry((e1, e2)).or_insert(0) += 1;
        last_char = e2;
    }
    lines.next(); //blank line
    let rules: HashMap<(char, char), char> = lines
        // lines
        .map(|line| line.split(" -> ").collect::<Vec<&str>>())
        .map(|splitted| {
            (splitted[0].chars().collect::<Vec<char>>(), splitted[1].chars().last().unwrap())
        })
        .map(|(key, val)| ((key[0], key[1]), val))
        .collect();
    (template, last_char, rules)
}

fn step(template: &HashMap<(char, char), i64>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), i64> {
    let mut new_template: HashMap<(char, char), i64> = HashMap::new();

    for (&key, &val) in template {
            let new_element = *rules.get(&(key.0, key.1)).unwrap();
            let new_key1 = (key.0, new_element);
            let new_key2 = (new_element, key.1);
            *new_template.entry(new_key1).or_insert(0) += val;
            *new_template.entry(new_key2).or_insert(0) += val;
    }

    new_template
}

fn calc_counts(template: &HashMap<(char, char), i64>, last_char: char) -> HashMap<char, i64> {
    let mut counts: HashMap<char, i64> = HashMap::new();
    for (&key, &val) in template {
        *counts.entry(key.0).or_insert(0) += val;
    }
    *counts.entry(last_char).or_insert(0) += 1;
    counts
}