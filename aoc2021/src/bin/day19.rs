use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;
use itertools::Itertools;
use regex::Regex;
use ndarray::{arr1, arr2, Array2};

fn main() {
    let input = fs::read_to_string("data/day19.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut scanners = input.split("\n\n").map(|x| parse_scanner(x).1).collect::<Vec<Vec<[i32; 3]>>>();
    let mut distances: Vec<Vec<Vec<i64>>> = scanners.iter()
        .map(|beacons| calc_distances(beacons))
        .collect();


    while scanners.len() > 1 {
        let mut scanner_to_remove: Option<usize> = None;
        for scanner_numbers in (0..scanners.len()).combinations(2) {
            let base_scanner_no = scanner_numbers[0];
            let secondary_scanner_no = scanner_numbers[1];

            distances[base_scanner_no] = calc_distances(&scanners[base_scanner_no]);
            distances[secondary_scanner_no] = calc_distances(&scanners[secondary_scanner_no]);

            let overlaps = overlap(&distances[base_scanner_no], &distances[secondary_scanner_no]);
            if overlaps.len() < 12 {
                continue
            }
            let translation = find_translation(&overlaps, &scanners[base_scanner_no], &scanners[secondary_scanner_no]);
            match translation {
                None => {continue}
                Some((tx, rot_id)) => {
                    let mut new_beacons: Vec<[i32; 3]> = Vec::new();
                    for b in scanners[secondary_scanner_no].clone() {
                        let vec = get_all_orientations()[rot_id].dot(&arr1(&b)).to_vec();
                        let vec = [vec[0] + tx[0], vec[1] + tx[1], vec[2] + tx[2]];
                        if !scanners[base_scanner_no].contains(&vec) {
                            new_beacons.push(vec);
                        }
                    }
                    scanners[base_scanner_no].append(&mut new_beacons);
                    scanner_to_remove = Some(secondary_scanner_no);
                    break;
                }
            }
        }
        match scanner_to_remove {
            None => {
                unreachable!();
            },
            Some(scanner_no) => {
                scanners.remove(scanner_no);
            }
        };
        scanner_to_remove = None;
    }
    scanners[0].len() as i64
}

fn part2(input: &str) -> i64 {
    let mut scanners = input.split("\n\n").map(|x| parse_scanner(x).1).collect::<Vec<Vec<[i32; 3]>>>();
    let mut distances: Vec<Vec<Vec<i64>>> = scanners.iter()
        .map(|beacons| calc_distances(beacons))
        .collect();

    let mut translations: Vec<[i32; 3]> = Vec::new();

    while scanners.len() > 1 {
        let mut scanner_to_remove: Option<usize> = None;
        for scanner_numbers in (0..scanners.len()).combinations(2) {
            let base_scanner_no = scanner_numbers[0];
            let secondary_scanner_no = scanner_numbers[1];

            distances[base_scanner_no] = calc_distances(&scanners[base_scanner_no]);
            distances[secondary_scanner_no] = calc_distances(&scanners[secondary_scanner_no]);

            let overlaps = overlap(&distances[base_scanner_no], &distances[secondary_scanner_no]);
            if overlaps.len() < 12 {
                continue
            }
            let translation = find_translation(&overlaps, &scanners[base_scanner_no], &scanners[secondary_scanner_no]);
            match translation {
                None => {continue}
                Some((tx, rot_id)) => {
                    let mut new_beacons: Vec<[i32; 3]> = Vec::new();
                    for b in scanners[secondary_scanner_no].clone() {
                        let vec = get_all_orientations()[rot_id].dot(&arr1(&b)).to_vec();
                        let vec = [vec[0] + tx[0], vec[1] + tx[1], vec[2] + tx[2]];
                        if !scanners[base_scanner_no].contains(&vec) {
                            new_beacons.push(vec);
                        }
                    }
                    scanners[base_scanner_no].append(&mut new_beacons);
                    scanner_to_remove = Some(secondary_scanner_no);
                    translations.push(tx);
                    break;
                }
            }
        }
        match scanner_to_remove {
            None => {
                unreachable!();
            },
            Some(scanner_no) => {
                scanners.remove(scanner_no);
            }
        };
        scanner_to_remove = None;
    }
    translations.iter()
        .combinations(2)
        .map(|t| manhattan(*t[0], *t[1]))
        .max()
        .unwrap() as i64
}

fn manhattan(v1: [i32; 3], v2: [i32; 3]) -> i32 {
    (v1[0] - v2[0]).abs() + (v1[1] - v2[1]).abs() + (v1[2] - v2[2]).abs()
}


fn parse_scanner(input: &str) -> (usize, Vec<[i32; 3]>) {
    let mut iter = input.split("\n");
    let pattern = r"--- scanner (\d+) ---";
    let rg = Regex::new(pattern).unwrap();
    let scanner_no = rg.captures(iter.next().unwrap()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
    let mut points: Vec<[i32; 3]> = Vec::new();
    for line in iter {
        let mut splitted = line.split(",");
        points.push([splitted.next().unwrap().parse::<i32>().unwrap(), splitted.next().unwrap().parse::<i32>().unwrap(), splitted.next().unwrap().parse::<i32>().unwrap()])
    }
    (scanner_no, points)
}

fn calc_distances(beacons :&Vec<[i32; 3]>) -> Vec<Vec<i64>> {
    let mut distances: Vec<Vec<i64>> = Vec::new();
    for i in 0..beacons.len() {
        let mut dst: Vec<i64> = Vec::new();
        for j in 0..beacons.len() {
            dst.push(calc_distance(beacons[i], beacons[j]));
        }
        distances.push(dst);
    }

    distances
}

fn calc_distance(p1: [i32; 3], p2: [i32; 3]) -> i64 {
    let p1 = [p1[0] as i64, p1[1] as i64, p1[2] as i64];
    let p2 = [p2[0] as i64, p2[1] as i64, p2[2] as i64];
    i64::pow(p1[0] - p2[0], 2) + i64::pow(p1[1] - p2[1], 2) + i64::pow(p1[2] - p2[2], 2)
}

fn overlap(distances1: &Vec<Vec<i64>>, distances2: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    let mut overlaps: Vec<(usize, usize)> = Vec::new();
    for i in 0..distances1.len() {
        let p1: HashSet<i64> = HashSet::from_iter(distances1[i].iter().cloned());
        for j in 0..distances2.len() {
            let p2: HashSet<i64> = HashSet::from_iter(distances2[j].iter().cloned());
            if p1.intersection(&p2).count() >= 12 {
                overlaps.push((i, j));
                break;
            }
        }
    }
    overlaps
}

fn get_all_orientations() -> Vec<Array2<i32>> {
    vec![
        arr2(&[[0, 1, 0],
            [-1, 0, 0],
            [0, 0, 1]]),
        arr2(&[[1, 0, 0],
            [0, 1, 0],
            [0, 0, 1]]),
        arr2(&[[0, -1, 0],
            [0, 0, 1],
            [-1, 0, 0]]),
        arr2(&[[0, 1, 0],
            [0, 0, 1],
            [1, 0, 0]]),
        arr2(&[[0, 1, 0],
            [0, 0, -1],
            [-1, 0, 0]]),
        arr2(&[[-1, 0, 0],
            [0, 0, 1],
            [0, 1, 0]]),
        arr2(&[[1, 0, 0],
            [0, 0, -1],
            [0, 1, 0]]),
        arr2(&[[1, 0, 0],
            [0, 0, 1],
            [0, -1, 0]]),
        arr2(&[[0, -1, 0],
            [0, 0, -1],
            [1, 0, 0]]),
        arr2(&[[0, 0, 1],
            [-1, 0, 0],
            [0, -1, 0]]),
        arr2(&[[0, 0, -1],
            [-1, 0, 0],
            [0, 1, 0]]),
        arr2(&[[0, 0, 1],
            [0, 1, 0],
            [-1, 0, 0]]),
        arr2(&[[0, 0, -1],
            [1, 0, 0],
            [0, -1, 0]]),
        arr2(&[[1, 0, 0],
            [0, -1, 0],
            [0, 0, -1]]),
        arr2(&[[0, -1, 0],
            [-1, 0, 0],
            [0, 0, -1]]),
        arr2(&[[-1, 0, 0],
            [0, 1, 0],
            [0, 0, -1]]),
        arr2(&[[0, 0, 1],
            [1, 0, 0],
            [0, 1, 0]]),
        arr2(&[[0, 0, 1],
            [0, -1, 0],
            [1, 0, 0]]),
        arr2(&[[0, 1, 0],
            [1, 0, 0],
            [0, 0, -1]]),
        arr2(&[[0, 0, -1],
            [0, -1, 0],
            [-1, 0, 0]]),
        arr2(&[[0, -1, 0],
            [1, 0, 0],
            [0, 0, 1]]),
        arr2(&[[-1, 0, 0],
            [0, -1, 0],
            [0, 0, 1]]),
        arr2(&[[0, 0, -1],
            [0, 1, 0],
            [1, 0, 0]]),
        arr2(&[[-1, 0, 0],
            [0, 0, -1],
            [0, -1, 0]])
    ]
}

fn find_all_rotations(v: [i32; 3]) -> Vec<[i32; 3]> {
    get_all_orientations().iter()
        .map(|orientation| {
            let tmp2 = arr1(&v);
            let vec = orientation.dot(&tmp2).to_vec();
            [vec[0], vec[1], vec[2]]
        })
        .collect()
}

fn translation(v1: [i32; 3], v2: [i32; 3]) -> [i32; 3] {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}

fn find_translation(overlaps: &Vec<(usize, usize)>, base_beacons: &Vec<[i32; 3]>, beacons: &Vec<[i32; 3]>) -> Option<([i32; 3], usize)> {
    let mut possible_translations: HashSet<([i32; 3], usize)> = HashSet::new();
    let mut dirty: bool = true;
    for (i, j) in overlaps {
        let base_beacon = base_beacons[*i];
        let same_beacon = beacons[*j];
        let rotations = find_all_rotations(same_beacon);
        let translation_candidates = rotations.iter().enumerate()
            .map(|(rot_id, rotated)| (translation(base_beacon, *rotated), rot_id))
            .collect::<Vec<([i32; 3], usize)>>();
        if dirty {
            possible_translations = translation_candidates.into_iter().collect();
            dirty = false;
        } else {
            possible_translations = possible_translations.intersection(&translation_candidates.into_iter().collect()).cloned().collect();
        }
        if possible_translations.len() == 0 {
            return None
        }
        if possible_translations.len() == 1 {
            return Some(*possible_translations.iter().next().unwrap());
        }
    }
    return None
}