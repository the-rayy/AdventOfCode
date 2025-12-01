use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day01.txt").expect("Unable to load input file");

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
    let mut current: i64 = 50;
    let mut password: u32 = 0;

    input.lines().for_each(|line| {
        let num = line
            .replace("L", "-")
            .replace("R", "")
            .parse::<i64>()
            .unwrap();
        current += num;
        if current % 100 == 0 {
            password += 1;
        }
    });

    password
}

fn part2(input: &str) -> u32 {
    let mut current: i64 = 50;
    let mut password: u32 = 0;

    input.lines().for_each(|line| {
        let num = line
            .replace("L", "-")
            .replace("R", "")
            .parse::<i64>()
            .unwrap();
        current += num;
        if current == 0 || current == 100 {
            password += 1;
            current = 0;
        }
        if current < 0 {
            password += (current / 100).abs() as u32 + 1;
            if current - num == 0 {
                password -= 1;
            }
            current = (current % 100) + 100;
            current = current % 100;
        }
        if current > 100 {
            password += (current / 100).abs() as u32;
            current = current % 100;
        }
    });

    password
}
