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

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
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

fn part2(input: &str) -> usize {
    input.split("\n")
        .map(|line| {
            let mut pair: bool = false;
            'outer: for i in 0..line.len()-1 {
                let a = &line[i..i+2];
                for j in i+2..line.len()-1 {
                    let b = &line[j..j+2];
                    if a == b {
                        pair = true;
                        break 'outer;
                    }
                }
            }

            let mut repeat: bool = false;
            for i in 0..line.len()-2 {
                if line.chars().nth(i).unwrap() == line.chars().nth(i+2).unwrap() {
                    repeat = true;
                    break;
                }
            }

            pair && repeat

        })
        .filter(|x| *x)
        .count()
}