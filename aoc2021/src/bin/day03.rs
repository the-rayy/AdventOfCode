use std::fs;

fn main() {
    let input = fs::read_to_string("data/day03.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let file_len = input.split("\n").collect::<Vec<&str>>().len() as i32;
    let line_len = input.split("\n").next().unwrap().len();
    let mut counters = vec![0; line_len];
    input.split("\n")
        .for_each(|line| line.chars()
            .into_iter()
            .enumerate()
            .for_each(|(i, x)| if x == '1' {counters[i] = counters[i] + 1} ));


    let mut gamma = vec!['0'; line_len];
    let mut epsilon = vec!['0'; line_len];

    counters.iter()
        .enumerate()
        .for_each(|(i, c)| if *c > file_len / 2 {
                gamma[i] = '1';
            } else {
                epsilon[i] = '1';
            }
        );

    let gamma: String = gamma.iter().collect();
    let epsilon: String = epsilon.iter().collect();

    let gamma = isize::from_str_radix(&*gamma, 2).unwrap() as i32;
    let epsilon = isize::from_str_radix(&*epsilon, 2).unwrap() as i32;

    gamma * epsilon
}

fn part2(input: &str) -> i32 {
    let line_len = input.split("\n").next().unwrap().len();

    let mut lines = input.split("\n").collect::<Vec<&str>>();
    let mut oxygen = "";
    for i in 0..line_len {
        let (mut zeros, mut ones) = (0, 0);
        lines.iter().for_each(|line| {
            let c = line.chars().into_iter().collect::<Vec<char>>()[i];
            match c {
                '0' => zeros = zeros + 1,
                '1' => ones = ones + 1,
                _ => unreachable!()
            }
        });
        let char_to_leave = if ones >= zeros {
            '1'
        } else {
            '0'
        };
        lines = lines.into_iter().filter(|line| line.chars().nth(i).unwrap() == char_to_leave).collect::<Vec<&str>>();
        if lines.len() == 1 {
            oxygen = lines[0];
            break;
        }
    };

    let oxygen = isize::from_str_radix(oxygen, 2).unwrap() as i32;

    let mut lines = input.split("\n").collect::<Vec<&str>>();
    let mut co2 = "";
    for i in 0..line_len {
        let (mut zeros, mut ones) = (0, 0);
        lines.iter().for_each(|line| {
            let c = line.chars().into_iter().collect::<Vec<char>>()[i];
            match c {
                '0' => zeros = zeros + 1,
                '1' => ones = ones + 1,
                _ => unreachable!()
            }
        });
        let char_to_leave = if ones >= zeros {
            '0'
        } else {
            '1'
        };
        lines = lines.into_iter().filter(|line| line.chars().nth(i).unwrap() == char_to_leave).collect::<Vec<&str>>();
        if lines.len() == 1 {
            co2 = lines[0];
            break;
        }
    };

    let co2 = isize::from_str_radix(co2, 2).unwrap() as i32;

    oxygen*co2
}