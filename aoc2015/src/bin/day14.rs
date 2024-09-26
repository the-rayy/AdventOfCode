use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day14.txt")
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

const REGEX: &str = r"(.*) can fly (\d+) km\/s for (\d+) seconds, but then must rest for (\d+) seconds.";
fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(REGEX).unwrap();

    input.split("\n")
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let deer = caps[1].to_string();
            let speed = caps[2].parse::<i32>().unwrap();
            let dist = caps[3].parse::<i32>().unwrap();
            let rest = caps[4].parse::<i32>().unwrap();
            (deer, speed, dist, rest)
        }).map(|(deer, speed, dist, rest)| {
            let cycle = 2503 / (dist + rest);
            let rem = 2503 % (dist + rest);
            let dist_covered = cycle * speed * dist + std::cmp::min(rem, dist) * speed;
            (deer, dist_covered)
        }).max_by_key(|(_, dist)| *dist).unwrap().1
}
