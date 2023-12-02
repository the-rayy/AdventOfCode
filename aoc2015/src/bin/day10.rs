use std::time::Instant;

fn main() {
    let input = "1113222113";

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> usize {
    let mut input = input.to_string();
    for _ in 0..40 {
        input = look_and_see(input);
    }
    input.len()
}

fn part2(input: &str) -> usize {
    let mut input = input.to_string();
    for _ in 0..50 {
        input = look_and_see(input);
    }
    input.len()
}

fn look_and_see(input: String) -> String {
    let mut output = Vec::<usize>::new();
    let acc = input.chars().fold((' ', 0), |acc, c| {
        if c == acc.0 || acc.0 == ' ' {
            (c, acc.1 + 1)
        } else {
            output.push(acc.1);
            output.push(acc.0.to_string().parse::<usize>().unwrap());
            (c, 1)
        }
    });

    output.push(acc.1);
    output.push(acc.0.to_string().parse::<usize>().unwrap());

    output.iter()
        .map(|x| x.to_string())
        .collect::<String>()
}
