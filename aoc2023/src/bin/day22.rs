use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env::current_exe;
use std::fs;
use std::ops::Sub;
use std::time::Instant;
use num::integer::lcm;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("data/day22.txt")
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


fn part1(input: &str) -> usize {
    let mut bricks = input.split("\n")
        .map(|x| Brick::new(x))
        .collect::<Vec<_>>();

    bricks.sort_by_key(|x| x.z.0);

    loop {
        for i in 0..bricks.len() {
            if bricks[i].settled {
                continue;
            }

            let other_bricks = bricks.iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, x)| x);

            if let Some(dropped) = bricks[i].dropped() {
                if !other_bricks.clone().any(|x| x.collides(&dropped)) {
                    bricks[i].drop();
                    break;
                }

                let supports = other_bricks.clone()
                    .enumerate()
                    .filter(|(_, x)| x.collides(&dropped) && x.settled)
                    .map(|(j, _)| j)
                    .collect::<HashSet<_>>();

                if supports.len() > 0 {
                    bricks[i].settled = true;
                    bricks[i].supported_by = supports;
                    // supports.iter()
                    //     .for_each(|j| bricks[*j].supported_by.push(i));
                    break;
                }
            }
        }

        if bricks.iter().all(|x| x.settled) {
            break;
        }
    }

    bricks.len() - bricks.iter()
        .filter(|b| b.supported_by.len() == 1)
        .map(|b| b.supported_by.iter().next().unwrap())
        .collect::<HashSet<_>>()
        .len()
}


fn part2(input: &str) -> usize {
    let mut bricks = input.split("\n")
        .map(|x| Brick::new(x))
        .collect::<Vec<_>>();

    bricks.sort_by_key(|x| x.z.0);

    loop {
        for i in 0..bricks.len() {
            if bricks[i].settled {
                continue;
            }

            let other_bricks = bricks.iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, x)| x);

            if let Some(dropped) = bricks[i].dropped() {
                if !other_bricks.clone().any(|x| x.collides(&dropped)) {
                    bricks[i].drop();
                    break;
                }

                let supports = other_bricks.clone()
                    .enumerate()
                    .filter(|(_, x)| x.collides(&dropped) && x.settled)
                    .map(|(j, _)| j)
                    .collect::<HashSet<_>>();

                if supports.len() > 0 {
                    bricks[i].settled = true;
                    supports.iter()
                        .for_each(|j| bricks[*j].supports.push(i));
                    bricks[i].supported_by = supports;
                    break;
                }
            }
        }

        if bricks.iter().all(|x| x.settled) {
            break;
        }
    }

    (0..bricks.len())
        .map(|x| fall(&bricks, HashSet::from([x])).len() - 1)
        .sum()
}

//caching would be nice here, but 1,5s is good enough
fn fall(bricks: &Vec<Brick>, removed: HashSet<usize>) -> HashSet<usize> {
    let nxt = bricks.iter()
        .enumerate()
        .filter(|(i, x)| !removed.contains(i) && !x.supported_by.is_empty())
        .filter(|(_, x)| x.supported_by.sub(&removed).len() == 0)
        .map(|(i, _)| i)
        .collect::<HashSet<usize>>();

    if nxt.len() == 0 {
        return removed;
    }

    let mut removed = removed.clone();
    removed.extend(nxt.clone());
    fall(bricks, removed)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
    settled: bool,
    supported_by: HashSet<usize>,
    supports: Vec<usize>,
}

impl Brick {
    fn new(inp: &str) -> Brick {
        let pattern = r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)";
        let re = Regex::new(pattern).unwrap();
        let caps = re.captures(inp).unwrap();

        Brick {
            x: (min(caps[1].parse().unwrap(), caps[4].parse().unwrap()), max(caps[1].parse().unwrap(), caps[4].parse().unwrap())),
            y: (min(caps[2].parse().unwrap(), caps[5].parse().unwrap()), max(caps[2].parse().unwrap(), caps[5].parse().unwrap())),
            z: (min(caps[3].parse().unwrap(), caps[6].parse().unwrap()), max(caps[3].parse().unwrap(), caps[6].parse().unwrap())),
            settled: caps[3].parse::<usize>().unwrap() == 1,
            supported_by: HashSet::new(),
            supports: Vec::new(),
        }
    }

    fn drop(&mut self) {
        self.z.0 -= 1;
        self.z.1 -= 1;

        if self.z.0 == 1 {
            self.settled = true;
        }
    }

    fn dropped(&self) -> Option<Brick> {
        if self.settled {
            return None;
        }

        if self.z.0 == 1 {
            return None;
        }

        let mut new = self.clone();
        new.drop();
        Some(new)
    }

    fn collides(&self, other: &Brick) -> bool {
        if self.x.0 > other.x.1 || self.x.1 < other.x.0 {
            return false;
        }
        if self.y.0 > other.y.1 || self.y.1 < other.y.0 {
            return false;
        }
        if self.z.0 > other.z.1 || self.z.1 < other.z.0 {
            return false;
        }
        true
    }
}

