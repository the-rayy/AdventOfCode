use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day08.txt")
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

const START: &str = "AAA";
const END: &str = "ZZZ";

fn part1(input: &str) -> usize {
    let (instr, nodes) = parse(input);

    let mut current = String::from(START);
    let mut step = 0;

    loop {
        let i = instr.get(step % instr.len()).unwrap();
        match i {
            'L' => current = nodes.get(current.as_str()).unwrap().0.clone(),
            'R' => current = nodes.get(current.as_str()).unwrap().1.clone(),
            _ => unreachable!()
        }
        step += 1;
        if current.as_str() == END {
            break step;
        }
    }
}


fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut splitted = input.split("\n\n");
    let instr = splitted.next().unwrap().chars().collect();
    let g = splitted.next().unwrap().split("\n").map(|line| {
        let chars = line.chars();
        let key = chars.clone().take(3).collect::<String>();
        let left = chars.clone().skip(7).take(3).collect::<String>();
        let right = chars.clone().skip(12).take(3).collect::<String>();
        (key, (left, right))
    })
        .collect();

    (instr, g)
}
