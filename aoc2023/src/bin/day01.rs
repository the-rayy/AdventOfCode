use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day01.txt")
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

fn part1(input: &str) -> u32 {
    input.split("\n")
        .map(|line| {
            let c = line.chars().filter(|x| x.is_digit(10));
            10 * c.clone().next().unwrap().to_digit(10).unwrap() + c.clone().last().unwrap().to_digit(10).unwrap()
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input.split("\n")
        .map(|line| {
            let line = line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            let c = line.chars().filter(|x| x.is_digit(10));
            10 * c.clone().next().unwrap().to_digit(10).unwrap() + c.clone().last().unwrap().to_digit(10).unwrap()
        })
        .sum::<u32>()
}
