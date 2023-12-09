use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day09.txt")
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
    input.split("\n")
        .map(|line| {
            let state = line.split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut states = Vec::<Vec<i64>>::new();
            states.push(state);

            let mut rank = 0;
            loop {
                let der = states.get(rank).unwrap().iter()
                    .tuple_windows()
                    .map(|(x1, x2)| x2 - x1)
                    .collect::<Vec<i64>>();
                rank += 1;
                states.push(der);
                if states.get(rank).unwrap().iter().all(|&x| x == 0) {
                    break;
                }
            }

            states.get_mut(rank).unwrap().push(0);

            for i in (0..rank).rev() {
                let x1 = *states.get(i).unwrap().last().unwrap();
                let y = *states.get(i+1).unwrap().last().unwrap();
                let x2 = x1 + y;
                states.get_mut(i).unwrap().push(x2);
            }

            *states.get(0).unwrap().last().unwrap()
        }).sum::<i64>()
}


fn part2(input: &str) -> i64 {
    input.split("\n")
        .map(|line| {
            let state = line.split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut states = Vec::<Vec<i64>>::new();
            states.push(state);

            let mut rank = 0;
            loop {
                let der = states.get(rank).unwrap().iter()
                    .tuple_windows()
                    .map(|(x1, x2)| x2 - x1)
                    .collect::<Vec<i64>>();
                rank += 1;
                states.push(der);
                if states.get(rank).unwrap().iter().all(|&x| x == 0) {
                    break;
                }
            }

            states.get_mut(rank).unwrap().insert(0, 0);

            for i in (0..rank).rev() {
                let x2 = *states.get(i).unwrap().first().unwrap();
                let y = *states.get(i+1).unwrap().first().unwrap();
                let x1 = x2 - y;
                states.get_mut(i).unwrap().insert(0, x1);
            }

            *states.get(0).unwrap().first().unwrap()
        }).sum::<i64>()
}