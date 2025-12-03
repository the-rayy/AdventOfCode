use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day03.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bank: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let (first_idx, first) = &bank[0..bank.len() - 1]
                .iter()
                .enumerate()
                .max_by(|(i, a), (j, b)| match a.cmp(b) {
                    std::cmp::Ordering::Equal => i.cmp(j).reverse(),
                    x => x,
                })
                .unwrap();
            let second = &bank[*first_idx + 1..bank.len()].iter().max().unwrap();

            10 * (*first) + *second
        })
        .sum()
}
