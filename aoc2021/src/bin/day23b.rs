use std::cmp::{min, max, Reverse};
use std::fs;
use lazy_static::lazy_static;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = fs::read_to_string("data/day23b.txt")
        .expect("Unable to load input file");
    println!("Part 2: {}", part2(&input));
}

lazy_static! {
    static ref ENERGY_PER_MOVE: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('A', 1);
        m.insert('B', 10);
        m.insert('C', 100);
        m.insert('D', 1000);
        m
    };
    static ref TARGET_ROOM: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('A', 0);
        m.insert('B', 1);
        m.insert('C', 2);
        m.insert('D', 3);
        m
    };
}

const HALL_SPOTS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOM_ENTRY: [usize; 4] = [2, 4, 6, 8];
const ROOM_TARGET: [char; 4] = ['A', 'B', 'C', 'D'];

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq, Hash)]
struct State {
    rooms: [Vec<char>; 4],
    hall: [char; 11],
}

impl State {
    fn all_valid_next_states(&self) -> Vec<(usize, State)> {
        let mut ret = self.move_from_hall_to_room();
        ret.append(&mut self.move_from_room_to_hall());
        ret
    }
    fn move_from_room_to_hall(&self) -> Vec<(usize, State)> {
        let mut ret: Vec<(usize, State)> = Vec::new();

        for room_idx in 0..4 {
            if self.room_complete(room_idx) {
                continue
            }
            let mut clone = self.clone();
            let c = clone.rooms[room_idx].pop();
            if c.is_none() { return ret; }
            let c = c.unwrap();

            for hall_idx in HALL_SPOTS {
                let move_cost = clone.can_move_from_room_to_hall(room_idx, hall_idx);
                if move_cost.is_none() { continue; }
                let total_move_cost = (move_cost.unwrap() + clone.room_exit_cost(room_idx)) * ENERGY_PER_MOVE.get(&c).unwrap();
                let mut new_state = clone.clone();
                new_state.hall[hall_idx] = c;
                ret.push((total_move_cost, new_state));
            }
        }

        ret
    }

    fn room_complete(&self, room_idx: usize) -> bool {
        let target = ROOM_TARGET[room_idx];
        self.rooms[room_idx].iter().filter(|c| **c == target).count() == self.rooms[room_idx].len()
    }

    fn can_move_from_room_to_hall(&self, room_idx: usize, hall_idx: usize) -> Option<usize> {
        let room_entry = ROOM_ENTRY[room_idx];
        let min = min(room_entry, hall_idx);
        let max = max(room_entry, hall_idx);
        return if self.hall[min..max + 1].iter().filter(|c| **c != '.').count() == 0 {
            Some(max - min)
        } else {
            None
        };
    }

    fn can_move_from_hall_to_room(&self, room_idx: usize, hall_idx: usize) -> Option<usize> {
        let mut cloned_hall = self.hall.clone();
        cloned_hall[hall_idx] = '.';
        let room_entry = ROOM_ENTRY[room_idx];
        let min = min(room_entry, hall_idx);
        let max = max(room_entry, hall_idx);
        return if cloned_hall[min..max + 1].iter().filter(|c| **c != '.').count() == 0 {
            Some(max - min)
        } else {
            None
        };
    }

    fn room_exit_cost(&self, room_idx: usize) -> usize {
        4 - self.rooms[room_idx].len()
    }

    fn room_enter_cost(&self, room_idx: usize) -> usize { //TODO same as above?
        4 - self.rooms[room_idx].len()
    }

    fn move_from_hall_to_room(&self) -> Vec<(usize, State)> {
        let mut ret: Vec<(usize, State)> = Vec::new();

        for (hall_idx, c) in self.hall.iter().enumerate().filter(|(_, c)| **c != '.') {
            let room_idx = *TARGET_ROOM.get(c).unwrap();
            if !self.is_room_valid_to_enter(room_idx, *c) {
                continue
            }
            let move_cost = self.can_move_from_hall_to_room(room_idx, hall_idx);
            if move_cost.is_none() { continue; }
            let total_move_cost = (move_cost.unwrap() + self.room_enter_cost(room_idx)) * ENERGY_PER_MOVE.get(&c).unwrap();
            let mut clone = self.clone();
            clone.hall[hall_idx] = '.';
            clone.rooms[room_idx].push(*c);
            ret.push((total_move_cost, clone));

        }

        return ret
    }

    fn is_room_valid_to_enter(&self, room_idx: usize, target: char) -> bool {
        return self.rooms[room_idx].iter().filter(|c| **c == target).count() == self.rooms[room_idx].len()
    }
}

fn part2(input: &str) -> i64 {
    let first_state = load(input);
    let desired_state = State {
        hall: ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        rooms: [vec!['A', 'A', 'A', 'A'], vec!['B', 'B', 'B', 'B'], vec!['C', 'C', 'C', 'C'], vec!['D', 'D', 'D', 'D']],
    };

    let mut distances = BinaryHeap::new();
    distances.push(Reverse((0, first_state)));
    let mut visited: HashMap<State, usize> = HashMap::new();

    let min_dist = loop {
        let (dist, state) = distances.pop().unwrap().0;
        if state == desired_state {
            break dist
        }

        for (new_dist, new_state) in state.all_valid_next_states() {
            match visited.get(&new_state) {
                None => {}
                Some(old_dist) => {
                    if *old_dist <= new_dist + dist {continue}
                }
            }
            visited.insert(new_state.clone(), dist + new_dist);
            distances.push(Reverse((dist + new_dist, new_state.clone())));
        }
    };


    min_dist as i64
}

fn load(input: &str) -> State {
    let lines: Vec<&str> = input.split("\n").collect();
    let room1: Vec<char> = vec![lines[5].chars().nth(3).unwrap(), lines[4].chars().nth(3).unwrap(), lines[3].chars().nth(3).unwrap(), lines[2].chars().nth(3).unwrap()];
    let room2: Vec<char> = vec![lines[5].chars().nth(5).unwrap(), lines[4].chars().nth(5).unwrap(), lines[3].chars().nth(5).unwrap(), lines[2].chars().nth(5).unwrap()];
    let room3: Vec<char> = vec![lines[5].chars().nth(7).unwrap(), lines[4].chars().nth(7).unwrap(), lines[3].chars().nth(7).unwrap(), lines[2].chars().nth(7).unwrap()];
    let room4: Vec<char> = vec![lines[5].chars().nth(9).unwrap(), lines[4].chars().nth(9).unwrap(), lines[3].chars().nth(9).unwrap(), lines[2].chars().nth(9).unwrap()];
    State {
        hall: ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        rooms: [room1, room2, room3, room4],
    }
}