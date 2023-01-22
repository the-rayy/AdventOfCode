use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    for a in input.split("\n") {
        for b in input.split("\n") {
            let a: i32 = a.parse().unwrap();
            let b: i32 = b.parse().unwrap();
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!()
}

fn part2(input: &str) -> i32 {
    for a in input.split("\n") {
        for b in input.split("\n") {
            for c in input.split("\n") {
                let a: i32 = a.parse().unwrap();
                let b: i32 = b.parse().unwrap();
                let c: i32 = c.parse().unwrap();
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!()
}
