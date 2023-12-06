use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day06.txt")
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

fn part1(input: &str) -> u64 {
    let (time, distance) = parse(input);

    time.iter()
        .zip(distance)
        .map(|(&t, d)| race(t, d))
        .product()
}

fn part2(input: &str) -> u64 {
    let (time, distance) = parse2(input);

    race(time, distance)
}

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut splitted = input.split("\n");
    let time = splitted.next().unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let distance = splitted.next().unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    (time, distance)
}

fn parse2(input: &str) -> (u64, u64) {
    let input = input.replace(" ", "");
    let mut splitted = input.split("\n");
    let time = splitted.next().unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let distance = splitted.next().unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    (time, distance)
}

fn race(time: u64, distance_to_beat: u64) -> u64 {
    let d = time * time - 4 * distance_to_beat;
    let d = (d as f64).sqrt();

    let x1 = (time as f64 - d) / 2.0;
    let x2 = (time as f64 + d) / 2.0;

    (x2.ceil()-x1.floor()) as u64 - 1
}
