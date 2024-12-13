use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day13.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> i64 {
    let pattern = r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)";
    let pattern = regex::Regex::new(pattern).unwrap();

    pattern
        .captures_iter(input)
        .filter_map(|cap| {
            let ax: i64 = cap[1].parse().unwrap();
            let ay: i64 = cap[2].parse().unwrap();
            let bx: i64 = cap[3].parse().unwrap();
            let by: i64 = cap[4].parse().unwrap();
            let px: i64 = cap[5].parse().unwrap();
            let py: i64 = cap[6].parse().unwrap();
            
            let b = (py*ax - px*ay) / (by*ax - bx*ay);
            let a = (px - b*bx) / ax;

            if a*ax + b*bx == px && a*ay + b*by == py {
                Some(3*a + b)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let pattern = r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)";
    let pattern = regex::Regex::new(pattern).unwrap();

    pattern
        .captures_iter(input)
        .filter_map(|cap| {
            let ax: i64 = cap[1].parse().unwrap();
            let ay: i64 = cap[2].parse().unwrap();
            let bx: i64 = cap[3].parse().unwrap();
            let by: i64 = cap[4].parse().unwrap();
            let px: i64 = cap[5].parse::<i64>().unwrap() + 10000000000000;
            let py: i64 = cap[6].parse::<i64>().unwrap() + 10000000000000;
            
            let b = (py*ax - px*ay) / (by*ax - bx*ay);
            let a = (px - b*bx) / ax;

            if a*ax + b*bx == px && a*ay + b*by == py {
                Some(3*a + b)
            } else {
                None
            }
        })
        .sum()
}
