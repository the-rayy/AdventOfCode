use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day02.txt")
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
    input.split("\n").enumerate()
        .map(|(idx, line)| {
            let sets = line.split(": ").nth(1).unwrap();
            let sets = sets.split("; ").map(parse_set).collect::<Vec<(usize, usize, usize)>>();
            let idx = idx + 1;
            let is_invalid = sets.iter().filter(|set| set.0 > 12 || set.1 > 13 || set.2 > 14).count() > 0;
            (idx, is_invalid)
        })
        .filter(|(_, is_invalid)| !is_invalid)
        .map(|(idx, _)| idx)
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    input.split("\n")
        .map(|line| {
            let sets = line.split(": ").nth(1).unwrap();
            let sets = sets.split("; ").map(parse_set).collect::<Vec<(usize, usize, usize)>>();
            let reds = sets.iter().map(|(r, _, _)| r).max().unwrap();
            let greens = sets.iter().map(|(_, g, _)| g).max().unwrap();
            let blues = sets.iter().map(|(_, _, b)| b).max().unwrap();
            reds * greens * blues
        })
        .sum::<usize>()
}

fn parse_set(set: &str) -> (usize, usize, usize) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for x in set.split(", ") {
        let mut parts = x.split(" ");
        let count = parts.next().unwrap().parse::<usize>().unwrap();
        let colour = parts.next().unwrap();

        match colour {
            "red" => r = count,
            "green" => g = count,
            "blue" => b = count,
            _ => unreachable!()
        }
    }

    (r, g, b)
}