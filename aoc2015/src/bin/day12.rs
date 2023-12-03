use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day12.txt")
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

const REGEX: &str = r"(-?\d+)";
fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(REGEX).unwrap();

    re.find_iter(input)
        .map(|s| s.as_str().parse::<i32>().unwrap())
        .sum()

}

fn part2(input: &str) -> i32 {
    input.match_indices(":\"red\"")
        .for_each(|(ind, _)| {
        })

    0
}
