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

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}


fn part1(input: &str) -> i64 {
    let (rules, parts) = parse(input);

    parts.iter()
        .filter(|part| eval(&rules, part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}


fn part2(input: &str) -> i64 {
    let (rules, _) = parse(input);

    let mut parts = vec![(String::from("in"), PartRange {
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    })];

    let mut accepted_parts = Vec::<PartRange>::new();

    while let Some((dest, part)) = parts.pop() {
        let dest = rules.get(dest.as_str()).unwrap();
        for (d, p) in dest.split(&part) {
            match d {
                Target::Accepted => {
                    accepted_parts.push(p);
                },
                Target::Rejected => {},
                Target::Rule(r) => {
                    parts.push((r.clone(), p));
                },

            }
        }
    }

    println!("{}", accepted_parts.len());

    accepted_parts.iter()
        .map(|p| p.score())
        .sum::<i64>()
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

    fn split(&self, part: &PartRange) -> Vec<(Target, PartRange)> {
        let mut ret = Vec::<(Target, PartRange)>::new();
        let mut to_match = part.clone();
        for rule in &self.rules {
            let (target, matched, not_matched) = rule.split(&to_match);
            if matched.is_some() {
                ret.push((target, matched.unwrap()));
            }

            match not_matched {
                Some(x) => to_match = x,
                None => break,
            }
        }

        return ret
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

    fn split(&self, part: &PartRange) -> (Target, Option<PartRange>, Option<PartRange>) {
        if self.operator == '#' {
            return (self.target.clone(), Some(part.clone()), None);
        }

        let (p1, p2) = part.split(self.left, self.operator, self.right);
        (self.target.clone(), Some(p1), Some(p2))
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

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartRange {
    fn split(&self, field: char, operator: char, val: i64)  -> (PartRange, PartRange) {
        let mut ret1 = self.clone();
        let mut ret2 = self.clone();
        match (field, operator) {
            ('x', '>') => {
                ret1.x.0 = val+1;
                ret2.x.1 = val+1;
            },
            ('x', '<') => {
                ret1.x.1 = val;
                ret2.x.0 = val;
            },
            ('m', '>') => {
                ret1.m.0 = val+1;
                ret2.m.1 = val+1;
            },
            ('m', '<') => {
                ret1.m.1 = val;
                ret2.m.0 = val;
            },
            ('a', '>') => {
                ret1.a.0 = val+1;
                ret2.a.1 = val+1;
            },
            ('a', '<') => {
                ret1.a.1 = val;
                ret2.a.0 = val;
            },
            ('s', '>') => {
                ret1.s.0 = val+1;
                ret2.s.1 = val+1;
            },
            ('s', '<') => {
                ret1.s.1 = val;
                ret2.s.0 = val;
            },
            _ => unreachable!(),
        }

        (ret1, ret2)
    }

    fn score(&self) -> i64 {
        let mut ret = 1;
        ret *= self.x.1 - self.x.0;
        ret *= self.m.1 - self.m.0;
        ret *= self.a.1 - self.a.0;
        ret *= self.s.1 - self.s.0;
        ret
    }
}