use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

use hashbrown::HashSet;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day10.txt").expect("Unable to load input file");

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
            let line = line.split(" ").collect::<Vec<&str>>();
            let target = line[0]
                .chars()
                .filter_map(|c| match c {
                    '[' | ']' => None,
                    '.' => Some(0),
                    '#' => Some(1),
                    _ => None,
                })
                .collect_vec();
            let buttons = line
                .iter()
                .skip(1)
                .rev()
                .skip(1)
                .map(|btns| {
                    btns.replace("(", "")
                        .replace(")", "")
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let lights = vec![0; target.len()];

            let mut n = 1_u64;
            loop {
                for comb in buttons.iter().combinations_with_replacement(n as usize) {
                    let mut lights = lights.clone();
                    for btn in comb {
                        lights = press(&lights, btn, 1);
                    }
                    if finished(&lights, &target) {
                        return n;
                    }
                }
                n += 1;
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let line = line.split(" ").collect::<Vec<&str>>();
            let target = line[line.len() - 1]
                .replace("{", "")
                .replace("}", "")
                .split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec();

            let buttons = line
                .iter()
                .skip(1)
                .rev()
                .skip(1)
                .map(|btns| {
                    btns.replace("(", "")
                        .replace(")", "")
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let lights = vec![0; target.len()];
            let mut q = VecDeque::new();
            q.push_back((0_u64, lights));
            let mut visited = HashSet::<Vec<u64>>::new();
            while let Some((count, lights)) = q.pop_front() {
                if visited.contains(&lights) {
                  continue;
                }
                visited.insert(lights.clone());
                match finished2(&lights, &target) {
                    Ordering::Less => {
                        for i in 0..buttons.len() {
                            let new_lights = press(&lights, &buttons[i], 1);
                            q.push_back((count + 1, new_lights));
                        }
                    }
                    Ordering::Equal => return count,
                    Ordering::Greater => continue,
                }
            }
            unreachable!();
        })
        .sum()
}

fn press(lights: &Vec<u64>, button: &Vec<usize>, times: u64) -> Vec<u64> {
    let mut lights = lights.clone();

    for btn in button {
        lights[*btn] += times;
    }

    lights
}

fn finished(lights: &Vec<u64>, target: &Vec<u64>) -> bool {
    lights.iter().zip(target).all(|(l, t)| l % 2 == *t)
}

fn finished2(lights: &Vec<u64>, target: &Vec<u64>) -> Ordering {
    if lights.iter().zip(target).all(|(l, t)| *l == *t) {
        Ordering::Equal
    } else if lights.iter().zip(target).any(|(l, t)| *l > *t) {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
