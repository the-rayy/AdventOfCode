use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day07.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    // let part2_start = Instant::now();
    // let part2_ans = part2(&input);
    // println!("Part 2 time: {:.2?}", part2_start.elapsed());
    // println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u64 {
    let mut beams = Vec::new();
    let mut split_counter = 0;

    input.lines().for_each(|line| {
        let mut new_beams = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                new_beams.push(i);
            };
            if c == '^' && beams.contains(&i) {
                new_beams.push(i - 1);
                new_beams.push(i + 1);
                split_counter += 1;
            }
            if c == '.' && beams.contains(&i) {
              new_beams.push(i);
            }
        }
        beams = new_beams;
    });

  split_counter
}
