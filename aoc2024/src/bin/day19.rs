use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day19.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    //let part2_start = Instant::now();
    //let part2_ans = part2(&input);
    //println!("Part 2 time: {:.2?}", part2_start.elapsed());
    //println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    _ = lines.next(); //empty line
    lines.map(|x| x.chars().collect::<Vec<_>>()).filter(|x| possible(&towels, x)).count() as u32
}

fn possible(towels: &Vec<Vec<char>>, design: &[char]) -> bool {
    if design.len() == 0 {
        return true;
    }

    for i in 0..towels.len() {
        let towel = &towels[i];
        if towel.len() > design.len() {
            continue;
        }

        if towel == &design[0..towel.len()] {
            if possible(towels, &design[towel.len()..]) {
                return true;
            }
        }
    }

    false
}

fn part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    _ = lines.next(); //empty line
    lines.map(|x| x.chars().collect::<Vec<_>>()).map(|x| possibillities(&towels, &x, Vec::new())).sum::<u32>()
}

fn possibillities(towels: &Vec<Vec<char>>, design: &Vec<char>, test: Vec<char>) -> u32 {
    if test.len() == design.len() && test == *design {
        return 1;
    }

    if test.len() >= design.len() {
        return 0;
    }

    if test != design[0..test.len()] {
        return 0;
    }

    towels.iter().map(|x| possibillities(towels, design, test.clone().into_iter().chain(x.clone()).collect())).sum()
}
