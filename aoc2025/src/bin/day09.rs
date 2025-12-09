use itertools::Itertools;
use std::cmp::{max, min};
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day09.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut s = line.split(",");
            let x = s.next().unwrap().parse::<i64>().unwrap();
            let y = s.next().unwrap().parse::<i64>().unwrap();
            (x, y)
        })
        .combinations(2)
        .map(|p| ((p[0].0 - p[1].0).abs() + 1) * ((p[0].1 - p[1].1).abs() + 1))
        .max()
        .unwrap() as u64
}

fn part2(input: &str) -> u64 {
    let mut points = input
        .lines()
        .map(|line| {
            let mut s = line.split(",");
            let x = s.next().unwrap().parse::<i64>().unwrap();
            let y = s.next().unwrap().parse::<i64>().unwrap();
            (x, y)
        })
        .collect_vec();
    let biggest = points
        .iter()
        .cloned()
        .combinations(2)
        .map(|p| {
            let area = ((p[0].0 - p[1].0).abs() + 1) * ((p[0].1 - p[1].1).abs() + 1);
            (p[0], p[1], area)
        })
        .sorted_by_key(|(_, _, area)| area.clone())
        .rev()
        .collect_vec();

    let first = points.first().unwrap().clone();
    let last = points.last().unwrap().clone();
    points.push(first);
    points.insert(0, last);

    for candidate in biggest {
        let top = min(candidate.0.1, candidate.1.1);
        let bottom = max(candidate.0.1, candidate.1.1);
        let left = min(candidate.0.0, candidate.1.0);
        let right = max(candidate.0.0, candidate.1.0);

        let is_inside = |x: (i64, i64)| x.0 > left && x.0 < right && x.1 > top && x.1 < bottom;
        let is_outside = |x: (i64, i64)| x.0 < left || x.0 > right || x.1 < top || x.1 > bottom;
        let crosses = |x: (i64, i64), y: (i64, i64)| {
            if x.0 == y.0 {
                if x.0 < left || x.0 > right {
                    return false;
                }
                let (y1, y2) = if x.1 < y.1 { (x.1, y.1) } else { (y.1, x.1) };
                return y2 < bottom && y1 > top;
            } else {
                if x.1 > bottom || x.1 < top {
                    return false;
                }
                let (x1, x2) = if x.0 < y.0 { (x.0, y.0) } else { (y.0, x.0) };
                return x2 > left && x1 < right;
            }
        };

        let bar = points.iter().tuple_windows().any(|(n1, p, n2)| {
            if *p == candidate.0 || *p == candidate.1 {
                return false;
            }
            if is_inside(*p) || is_inside(*n1) || is_inside(*n2) {
                return true;
            }

            if is_outside(*p) && is_outside(*n1) && crosses(*p, *n1) {
                return true;
            }

            if is_outside(*p) && is_outside(*n2) && crosses(*p, *n2) {
                return true;
            }

            let n = if *n1 == candidate.0 || *n1 == candidate.1 {
                n2
            } else {
                n1
            };
            let dir = {
                let v = (n.0 - p.0, n.1 - p.1);
                let dist = max(v.0.abs(), v.1.abs());
                (v.0 / dist, v.1 / dist)
            };

            let foo = (p.0 + dir.0, p.1 + dir.1);
            is_inside(foo)
        });
        if !bar {
            return candidate.2 as u64;
        }
    }

    unreachable!()
}
