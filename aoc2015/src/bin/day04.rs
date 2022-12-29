use std::time::Instant;
use md5;

fn main() {
    let input = "yzbqklnj";

    let part1_start = Instant::now();
    let part1_ans = part1(input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> usize {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, i));
        if format!("{:x}", digest).starts_with("00000") {
            return i
        }
        i += 1;
    }
}