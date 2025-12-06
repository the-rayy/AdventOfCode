use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day06.txt").expect("Unable to load input file");

    let part1_start = Instant::now();
    let part1_ans = part1(&input);
    println!("Part 1 time: {:.2?}", part1_start.elapsed());
    println!("Part 1 ans: {:?}", part1_ans);

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> u64 {
    let mut foo = input
        .lines()
        .map(|line| line.split(" ").filter(|x| x.len() > 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let signs = foo.pop().unwrap();

    let mut ans = 0;

    for i in 0..signs.len() {
        let mut total: u64 = match signs[i] {
            "+" => 0,
            "*" => 1,
            _ => unreachable!(),
        };
        for j in foo.iter() {
            match signs[i] {
                "+" => total += j[i].parse::<u64>().unwrap(),
                "*" => total *= j[i].parse::<u64>().unwrap(),
                _ => unreachable!(),
            }
        }
        ans += total;
    }

    ans
}

fn part2(input: &str) -> u64 {
    let foo = input
        .lines()
        .map(|line| line.chars().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let foo = transpose(foo);

    let mut ans: u64 = 0;
    let mut nums: Vec<u64> = vec![];
    for line in foo {
        let num = line
            .iter()
            .filter(|x| x.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .unwrap_or(0);
        if num == 0 { continue };

        if *line.last().unwrap() == '+' {
            let mut total = num;
            while let Some(n) = nums.pop() {
                total += n;
            }
            ans += total;
        } else if *line.last().unwrap() == '*' {
            let mut total = num;
            while let Some(n) = nums.pop() {
                total *= n;
            }
            ans += total;
        } else {
            nums.push(num);
        }
    }

    ans
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
