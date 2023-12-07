use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day07.txt")
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

fn part1(input: &str) -> usize {
    let mut cardbids = parse(input);

    cardbids.sort();

    cardbids.iter().enumerate()
        .map(|(idx, (hand, bid))| {
            bid * (idx + 1)
        })
        .sum()
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    value: u8,
    cards: Vec<u8>
}

impl Hand {
    fn from_str(s: &str) -> Hand {
        let cards = s.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                x => x.to_digit(10).unwrap()
            }
        })
            .map(|x| x as u8)
            .collect::<Vec<u8>>();

        let value = value(&cards);

        Hand{value, cards}
    }
}

fn parse(input: &str) -> Vec<(Hand, usize)> {
    input.split("\n")
        .map(|line| {
            let mut splitted = line.split(" ");
            let hand = splitted.next().unwrap();
            let hand = Hand::from_str(hand);

            let bid = splitted.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect()
}

fn value(arr: &Vec<u8>) -> u8 {
    let mut counts = HashMap::<u8, u8>::new();
    arr.iter()
        .for_each(|x| {
            *counts.entry(*x).or_default() += 1;
        });

    if counts.len() == 1 {
        return 7 // FIVE OF A KIND
    }

    if counts.len() == 2 && counts.values().filter(|&&x| x == 4).count() > 0 {
        return 6 // FOUR OF A KIND
    }

    if counts.len() == 2 && counts.values().filter(|&&x| x == 3).count() > 0 {
        return 5 // FULL HOUSE
    }

    if counts.len() == 3 && counts.values().filter(|&&x| x == 3).count() > 0 {
        return 4 // THREE OF A KIND
    }

    if counts.len() == 3 {
        return 3 // TWO PAIR
    }

    if counts.len() == 4 {
        return 2 // ONE PAIR
    }

    return 1 // HIGH CARD
}