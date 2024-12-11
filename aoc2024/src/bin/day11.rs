use std::fs;
use std::time::Instant;

use hashbrown::HashMap;

fn main() {
    let input = fs::read_to_string("data/day11.txt").expect("Unable to load input file");

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
    let mut cache = HashMap::with_capacity(100_000);

    input.split_whitespace().map(|num| {
        let num = num.parse::<u64>().unwrap();
        blink(num, 25, &mut cache)
    }).sum()
}

fn part2(input: &str) -> u64 {
    let mut cache = HashMap::with_capacity(100_000);

    input.split_whitespace().map(|num| {
        let num = num.parse::<u64>().unwrap();
        blink(num, 75, &mut cache)
    }).sum()
}

fn blink(num: u64, times: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if times == 0 {
        return 1
    }

    if let Some(&x) = cache.get(&(num, times)) {
        return x;
    }

    if num == 0 {
        let b = blink(1, times - 1, cache);
        cache.insert((1, times - 1), b);
        return b;
    }

    if let Some(x) = split_in_half_if_even_length(num) {
        let b1 = blink(x.0, times - 1, cache);
        let b2 = blink(x.1, times - 1, cache);

        cache.insert((x.0, times - 1), b1);
        cache.insert((x.1, times - 1), b2);

        return b1 + b2;
    }

    let b = blink(num * 2024, times - 1, cache);
    cache.insert((num * 2024, times - 1), b);
    return b;
}

fn split_in_half_if_even_length(num: u64) -> Option<(u64, u64)> { 
    let num_digits = (num as f64).log10().floor() as u32 + 1; //added +1 as log10(1000)=3 but has 4 digits 
    if num_digits % 2 == 0 { 
        let half_digits = num_digits / 2; 
        let divisor = 10u64.pow(half_digits); 
        let left = num / divisor; 
        let right = num % divisor; 
 
        Some((left, right)) 
 
    } else { 
        None 
    } 
} 
