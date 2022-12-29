use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("data/day05.txt")
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

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn part1(input: &str) -> usize {
    input.split("\n")
        .map(|line| {
            let vowels_count = line.chars().filter(|c| VOWELS.contains(c)).count();

            let mut line_copy = line.chars().collect::<Vec<char>>();
            line_copy.dedup();

            let forbidden = FORBIDDEN.iter()
                .map(|f| line.contains(f))
                .any(|f| f);

            vowels_count >= 3 && line_copy.len() < line.len() && !forbidden
        })
        .filter(|x| *x)
        .count()
}