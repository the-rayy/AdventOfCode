use std::fs;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut rules :HashMap<&str,Vec<&str>> = HashMap::new();
    for rule in input.split("\n") {
        let splitted_rule = rule.split(" bags contain ").collect::<Vec<&str>>();
        let parent_bag = splitted_rule[0];
        splitted_rule[1]
            .split(", ")
            .filter(|b| *b != "no other bags.")
            .map(|b| find_bag(b).0)
            .for_each(|b| rules.entry(b).or_insert(Vec::new()).push(parent_bag));
    }

    let mut can_be_in :HashSet<&str> = HashSet::new();
    find("shiny gold", &rules, &mut can_be_in);

    can_be_in.len() as i32
}

fn part2(input: &str) -> i32 {
    let mut rules :HashMap<&str,Vec<(&str, i32)>> = HashMap::new();
    for rule in input.split("\n") {
        let splitted_rule = rule.split(" bags contain ").collect::<Vec<&str>>();
        let parent_bag = splitted_rule[0];
        splitted_rule[1]
            .split(", ")
            .filter(|b| *b != "no other bags.")
            .map(find_bag)
            .for_each(|(bag, count)| rules.entry(parent_bag)
                .or_insert(Vec::new())
                .push((bag, count))
            );
    }

    find2("shiny gold", &rules)
}

fn find_bag(input :&str) -> (&str, i32) {
    let re = Regex::new("([0-9]+) (.*) bag").unwrap();
    let bag = re.captures(input).unwrap().get(2).unwrap().as_str();
    let count :i32 = re.captures(input).unwrap().get(1).unwrap().as_str().parse().unwrap();
    (bag, count)
}

fn find<'a>(bag :&str, rules :&HashMap<&str,Vec<&'a str>>, can_be_in :&mut HashSet<&'a str>) {
    match rules.get(bag) {
        Some(v) => {
            for x in v {
                if !can_be_in.contains(x) {
                    can_be_in.insert(x);
                    find(x, rules, can_be_in)
                }
            }
        }
        None => ()
    }
}

fn find2(bag :&str, rules :&HashMap<&str,Vec<(&str, i32)>>) -> i32 {
    match rules.get(bag) {
        Some(rule) => {
            rule.iter()
                .map(|(bagg, count)| count + count * find2(bagg, rules))
                .sum()
        },
        None => 0
    }
}