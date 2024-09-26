use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day13.txt")
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

const REGEX: &str = r"(.*) would (gain|lose) (\d+) happiness units by sitting next to (.*).";
fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(REGEX).unwrap();

    let parsed = input.split("\n")
        .filter_map(|line| {
            let caps = re.captures(line)?;
            let a = caps[1].to_string();
            let b = caps[4].to_string();
            let happiness = caps[3].parse::<i32>().unwrap();
            let happiness = if &caps[2] == "gain" { happiness } else { -happiness };
            Some((a, b, happiness))
        })
        .collect::<Vec<_>>();

    let mut people = parsed.iter()
        .flat_map(|(a, b, _)| vec![a.clone(), b.clone()])
        .unique()
        .collect::<Vec<_>>();

    people.iter()
    .permutations(people.len())
    .map(|seating| {
        seating.iter()
        .zip(seating.iter().cycle().skip(1))
        .map(|(a, b)| {
            parsed.iter()
            .find(|(x, y, _)| (x == *a && y == *b))
            .unwrap()
            .2 + 

            parsed.iter()
            .find(|(x, y, _)| (y == *a && x == *b))
            .unwrap()
            .2
        })
        .sum::<i32>()
    })
    .max().unwrap()
}

fn part2(input: &str) -> i32 {
    let re = regex::Regex::new(REGEX).unwrap();

    let mut parsed = input.split("\n")
        .filter_map(|line| {
            let caps = re.captures(line)?;
            let a = caps[1].to_string();
            let b = caps[4].to_string();
            let happiness = caps[3].parse::<i32>().unwrap();
            let happiness = if &caps[2] == "gain" { happiness } else { -happiness };
            Some((a, b, happiness))
        })
        .collect::<Vec<_>>();

    let mut people = parsed.iter()
        .flat_map(|(a, b, _)| vec![a.clone(), b.clone()])
        .unique()
        .collect::<Vec<_>>();

    people.iter()
    .for_each(|person| {
        parsed.push(("me".to_string(), person.clone(), 0));
        parsed.push((person.clone(), "me".to_string(), 0));
    });

    people.push("me".to_string());

    people.iter()
    .permutations(people.len())
    .map(|seating| {
        seating.iter()
        .zip(seating.iter().cycle().skip(1))
        .map(|(a, b)| {
            parsed.iter()
            .find(|(x, y, _)| (x == *a && y == *b))
            .unwrap()
            .2 + 

            parsed.iter()
            .find(|(x, y, _)| (y == *a && x == *b))
            .unwrap()
            .2
        })
        .sum::<i32>()
    })
    .max().unwrap()
}
