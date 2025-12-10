use std::fs;
use std::time::Instant;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day10.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
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
                        lights = press(&lights, btn);
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

fn press(lights: &Vec<u64>, button: &Vec<usize>) -> Vec<u64> {
    let mut lights = lights.clone();

    for btn in button {
        lights[*btn] += 1;
    }

    lights
}

fn finished(lights: &Vec<u64>, target: &Vec<u64>) -> bool {
    lights.iter().zip(target).all(|(l, t)| l % 2 == *t)
}
