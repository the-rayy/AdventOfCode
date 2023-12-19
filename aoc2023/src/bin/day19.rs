use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day19.txt")
        .expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}


fn part1(input: &str) -> usize {
    let (rules, parts) = parse(input);

    parts.iter()
        .filter(|part| eval(&rules, part))
        .count()
}

fn parse(input: &str) -> (HashMap<&str, Rules>, Vec<Part>) {
    let mut splitted = input.split("\n\n");
    let rules = splitted.next().unwrap().split("\n")
        .map(|line| {
            let line = line.strip_suffix("}").unwrap();
            let mut splitted = line.split("{");
            let key = splitted.next().unwrap();
            let value = splitted.next().unwrap().split(",")
                .map(|x| Rule::new(x))
                .collect::<Vec<Rule>>();
            (key, Rules{rules: value})
        })
        .collect::<HashMap<_, _>>();

    let parts = splitted.next().unwrap().split("\n")
        .map(|line| Part::new(line))
        .collect::<Vec<_>>();

    (rules, parts)
}

fn eval(rules_map: &HashMap<&str, Rules>, part: &Part) -> bool {
    let mut rules = rules_map.get("in").unwrap();
    loop {
        match rules.eval(part) {
            Target::Accepted => return true,
            Target::Rejected => return false,
            Target::Rule(r) => {
                rules = rules_map.get(r.as_str()).unwrap();
                continue;
            }
        }

    }
}

struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn eval(&self, part: &Part) -> Target {
        for rule in &self.rules {
            match rule.eval(part) {
                Some(t) => return t,
                None => continue,
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone)]
enum Target {
    Rule(String),
    Accepted,
    Rejected,
}

#[derive(Debug)]
struct Rule {
    left: char,
    operator: char,
    right: i64,
    target: Target,
}

impl Rule {
    fn new(raw: &str) -> Rule {
        if raw.contains(":") {
            let mut splitted = raw.split(":");
            let splitted1 = splitted.next().unwrap();
            let left = splitted1.chars().nth(0).unwrap();
            let operator = splitted1.chars().nth(1).unwrap();
            let right = splitted1.chars().skip(2).collect::<String>().parse::<i64>().unwrap();
            let target = match splitted.next().unwrap() {
                "A" => Target::Accepted,
                "R" => Target::Rejected,
                x => Target::Rule(x.to_string()),
            };

            Rule {
                left,
                operator,
                right,
                target,
            }
        } else {
            Rule {
                left: 'x',
                operator: '#',
                right: 0,
                target: match raw {
                    "A" => Target::Accepted,
                    "R" => Target::Rejected,
                    x => Target::Rule(x.to_string()),
                },
            }
        }
    }

    fn eval(&self, part: &Part) -> Option<Target> {
        let v = match self.left {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!(),
        };

        match self.operator {
            '>' => {
                if v > self.right {
                    Some(self.target.clone())
                } else {
                    None
                }
            },
            '<' => {
                if v < self.right {
                    Some(self.target.clone())
                } else {
                    None
                }
            },
            '#' => {
                Some(self.target.clone())
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn new(raw: &str) -> Part {
        let raw = raw.strip_prefix("{").unwrap();
        let raw = raw.strip_suffix("}").unwrap();
        let mut p = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for s in raw.split(",") {
            let mut splitted = s.split("=");
            match splitted.next().unwrap() {
                "x" => p.x = splitted.next().unwrap().parse::<i64>().unwrap(),
                "m" => p.m = splitted.next().unwrap().parse::<i64>().unwrap(),
                "a" => p.a = splitted.next().unwrap().parse::<i64>().unwrap(),
                "s" => p.s = splitted.next().unwrap().parse::<i64>().unwrap(),
                _ => unreachable!(),
            }
        }

        p
    }
}