use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data/day12.txt")
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

const REGEX: &str = r"(-?\d+)";
fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(REGEX).unwrap();

    re.find_iter(input)
        .map(|s| s.as_str().parse::<i32>().unwrap())
        .sum()

}

fn part2(input: &str) -> i32 {
    let deserialized = serde_json::from_str::<serde_json::Value>(input).unwrap();
    calc(&deserialized)
}

fn calc(node: &serde_json::Value) -> i32 {
    match node {
        serde_json::Value::Number(n) => n.as_i64().unwrap() as i32,
        serde_json::Value::Array(arr) => arr.iter().map(|x| calc(x)).sum(),
        serde_json::Value::Object(obj) => {
            if obj.values().any(|v| v == "red") {
                0
            } else {
                obj.values().map(|x| calc(x)).sum()
            }
        }
        _ => 0
    }
}
