use std::array::IntoIter;
use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;
use itertools::{max, min};

fn main() {
    let input = fs::read_to_string("data/day21.txt")
        .expect("Unable to load input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct GameState {
    positions: [usize; 2],
    scores: [usize; 2],
    rolls: usize,
    dice: usize,
}

impl GameState {
    fn new(p1pos: usize, p2pos: usize) -> GameState {
        GameState{
            positions: [p1pos, p2pos],
            scores: [0, 0],
            rolls: 0,
            dice: 100
        }
    }

    fn turn(&mut self, player: usize) {
        let roll_val = self.roll3();
        self.movve(player, roll_val);
        self.scores[player] = self.scores[player] + self.positions[player]
    }

    fn roll(&mut self) -> usize {
        self.rolls = self.rolls + 1;
        self.dice = (self.dice + 1) % 100;
        if self.dice == 0 {100} else {self.dice}
    }

    fn roll3(&mut self) -> usize {
        self.roll() + self.roll() + self.roll()
    }

    fn movve(&mut self, player: usize, how_many: usize) {
        self.positions[player] = (self.positions[player] + how_many) % 10;
        if self.positions[player] == 0 {
            self.positions[player] = 10;
        };
    }

    fn end(&self) -> Option<usize> {
        if max(self.scores).unwrap() < 1000 {
            None
        } else {
            Some(min(self.scores).unwrap() * self.rolls)
        }
    }
}

#[derive(Clone, Copy, Eq, Hash)]
struct DiracGameState {
    positions: [usize; 2],
    scores: [usize; 2],
}

impl PartialEq for DiracGameState {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions && self.scores == other.scores
    }
}

impl DiracGameState {
    fn new(p1pos: usize, p2pos: usize) -> DiracGameState {
        DiracGameState{
            positions: [p1pos, p2pos],
            scores: [0, 0],
        }
    }

    fn turn(&self, player: usize) -> HashMap<DiracGameState, usize> {
        let mut ret: HashMap<DiracGameState, usize> = HashMap::new();
        for (m, c) in DiracGameState::roll3() {
            let mut new_state = self.clone();
            new_state.movve(player, m);
            new_state.scores[player] = new_state.scores[player] + new_state.positions[player];
            ret.insert(new_state, c);
        }
        ret
    }

    fn roll3() -> HashMap<usize, usize> {
        HashMap::<usize, usize>::from_iter(IntoIter::new([
            (3, 1),
            (4, 3),
            (5, 6),
            (6, 7),
            (7, 6),
            (8, 3),
            (9, 1),
        ]))
    }

    fn movve(&mut self, player: usize, how_many: usize) {
        self.positions[player] = (self.positions[player] + how_many) % 10;
        if self.positions[player] == 0 {
            self.positions[player] = 10;
        };
    }

    fn end(&self) -> bool {
        max(self.scores).unwrap() >= 21
    }

    fn p1won(&self) -> bool {
        self.scores[0] > self.scores[1]
    }
}

fn part1(input: &str) -> i64 {
    let (player1pos, player2pos) = load(input);
    let mut state = GameState::new(player1pos, player2pos);

    let mut turn: usize = 0;
    loop {
        state.turn(turn % 2);
        turn = turn + 1;
        if state.end().is_some() {
            break state.end().unwrap() as i64;
        }
    }
}

fn part2(input: &str) -> i64 {
    let (player1pos, player2pos) = load(input);
    let state = DiracGameState::new(player1pos, player2pos);
    let mut states: HashMap<DiracGameState, usize> = HashMap::new();
    states.insert(state, 1);

    let mut turn: usize = 0;
    loop {
        let mut new_states: HashMap<DiracGameState, usize> = HashMap::new();
        for (state, c) in states {
            if state.end() {
                *new_states.entry(state).or_insert(0) += c;
                continue
            }
            for (newstate, cc) in state.turn(turn % 2) {
                *new_states.entry(newstate).or_insert(0) += c*cc;
            }
        }
        turn += 1;
        states = new_states;
        if states.iter().filter(|(s, _)| !s.end()).count() == 0 {
            break
        }
    }
    let p1won = states.iter().filter(|(s, _)| s.p1won()).map(|(_, c)| *c).sum::<usize>() as i64;
    let p2won = states.iter().filter(|(s, _)| !s.p1won()).map(|(_, c)| *c).sum::<usize>() as i64;
    max([p1won, p2won]).unwrap()
}

fn load(input: &str) -> (usize, usize) {
    let mut iter = input.split("\n");
    (
        iter.next().unwrap().chars().last().unwrap().to_digit(10).unwrap() as usize,
        iter.next().unwrap().chars().last().unwrap().to_digit(10).unwrap() as usize,
    )
}
