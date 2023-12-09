use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use num::integer::lcm;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("data/day08.txt")
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
    let (instr, nodes) = parse(input);

    let mut current = "AAA";
    let mut step = 0;

    loop {
        let i = instr.get(step % instr.len()).unwrap();
        match i {
            'L' => current = nodes.get(current).unwrap().0,
            'R' => current = nodes.get(current).unwrap().1,
            _ => unreachable!()
        }
        step += 1;
        if current == "ZZZ" {
            break step;
        }
    }
}

fn part2(input: &str) -> usize {
    let (instr, nodes) = parse(input);

    nodes.keys()
        .collect::<Vec<&&str>>()
        .par_iter()
        .filter(|k| { k.ends_with("A") })
        .map(|&&k| {
            let mut current = k;
            let mut step = 0;

            loop {
                let i = instr.get(step % instr.len()).unwrap();
                match i {
                    'L' => current = &nodes.get(&current).unwrap().0,
                    'R' => current = &nodes.get(&current).unwrap().1,
                    _ => unreachable!()
                }
                step += 1;
                if current.ends_with("Z") {
                    break step;
                }
            }
        })
        .reduce(|| 1_usize, |acc, x| lcm(acc, x) )
}


fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut splitted = input.split("\n\n");
    let instr = splitted.next().unwrap().chars().collect();
    let g = splitted.next().unwrap().split("\n").map(|line| {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        (key, (left, right))
    })
        .collect();

    (instr, g)
}
