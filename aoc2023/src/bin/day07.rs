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

    let part2_start = Instant::now();
    let part2_ans = part2(&input);
    println!("Part 2 time: {:.2?}", part2_start.elapsed());
    println!("Part 2 ans: {:?}", part2_ans);
}

fn part1(input: &str) -> usize {
    let mut cardbids = parse(input);

    cardbids.sort_by_key(|(hand, _)| hand.as_sortable());

    cardbids.iter().enumerate()
        .map(|(idx, (hand, bid))| {
            bid * (idx + 1)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut cardbids = parse(input);

    cardbids.sort_by_key(|(hand, _)| hand.as_sortable_j());

    cardbids.iter().enumerate()
        .map(|(idx, (hand, bid))| {
            bid * (idx + 1)
        })
        .sum()
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    value: u8,
    value_j: u8,
    cards: Vec<u8>,
    cards_j: Vec<u8>,
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

        let cards_j = cards.iter()
            .map(|&x| {
                match x {
                    11 => 1,
                    z => z
                }
            })
            .collect::<Vec<u8>>();

        let value = value(&cards);

        let value_j = value_j(&cards_j);

        Hand{value, value_j, cards, cards_j}
    }

    fn as_sortable(&self) -> (u8, Vec<u8>) {
        (self.value, self.cards.clone())
    }

    fn as_sortable_j(&self) -> (u8, Vec<u8>) {
        (self.value_j, self.cards_j.clone())
    }
}

fn value(arr: &Vec<u8>) -> u8 {
    let mut counts = HashMap::<u8, u8>::new();
    arr.iter()
        .for_each(|x| {
            *counts.entry(*x).or_default() += 1;
        });

    return counts_to_fig(&counts);
}

fn value_j(arr: &Vec<u8>) -> u8 {
    let mut counts = HashMap::<u8, u8>::new();
    arr.iter()
        .for_each(|x| {
            *counts.entry(*x).or_default() += 1;
        });

    let count_j = counts.remove(&1).unwrap_or(0);
    let fig = counts_to_fig(&counts);
    match (count_j, fig) {
        (0, _) => fig,

        (1, 1) => 2,
        (1, 2) => 4,
        (1, 3) => 5,
        (1, 4) => 6,

        (2, 1) => 4,
        (2, 2) => 6,
        (2, 4) => 7,

        (3, 1) => 6,
        (3, 2) => 7,

        (_, 5) => fig,
        (_, 6) => 7,
        (_, 7) => fig,

        (4, _) => 7,
        (5, _) => 7,
        (x, y) => {
            println!("j:{}, fig:{}, cards:{:?}", x, y, arr);
            unreachable!();
        }
    }
}

fn counts_to_fig(counts: &HashMap<u8, u8>) -> u8 {
    if counts.values().filter(|&&x| x == 5).count() == 1 {
        return 7 // FIVE OF A KIND
    }

    if counts.values().filter(|&&x| x == 4).count() == 1 {
        return 6 // FOUR OF A KIND
    }

    if counts.values().filter(|&&x| x == 3).count() == 1 && counts.values().filter(|&&x| x == 2).count() == 1 {
        return 5 // FULL HOUSE
    }

    if counts.values().filter(|&&x| x == 3).count() > 0 {
        return 4 // THREE OF A KIND
    }

    if counts.values().filter(|&&x| x == 2).count() == 2 {
        return 3 // TWO PAIR
    }

    if counts.values().filter(|&&x| x == 2).count() == 1 {
        return 2 // ONE PAIR
    }

    return 1 // HIGH CARD
}