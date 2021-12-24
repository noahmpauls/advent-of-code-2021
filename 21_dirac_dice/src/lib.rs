use std::collections::{HashMap};

pub fn play_deterministic(players: Vec<u32>) -> u32 {
    let mut die = DieDeterministic::new();
    let mut rolls = 0;
    let mut positions = players;
    let mut scores: Vec<u32> = positions.iter().map(|_| 0).collect();

    loop {
        for player in 0..positions.len() {
            let roll: u32 = (0..3).map(|_| {
                rolls += 1;
                die.next().unwrap()
            }).sum();
    
            positions[player] = (((positions[player] - 1) + roll) % 10) + 1;
    
            scores[player] += positions[player];
    
            if scores[player] >= 1000 {
                let loser = match player {
                    0 => 1,
                    _ => 0,
                };
                return rolls * scores[loser];
            }
        }
    }
}

struct DieDeterministic {
    next: u32,
}

impl DieDeterministic {
    pub fn new() -> Self {
        Self { next: 0 }
    }
}

impl Iterator for DieDeterministic {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = (self.next + 1) % 100;
        Some(next + 1)
    }
}

mod counter;
use counter::Counter;

const ROLL_FREQS: [(u32, u128); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Player {
    One,
    Two,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct PlayerState {
    pub position: u32,
    pub score: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct DiracDiceState {
    pub p1: PlayerState,
    pub p2: PlayerState,
    pub next: Player,
}

impl DiracDiceState {
    pub fn next_player_state(&self) -> PlayerState {
        match self.next {
            Player::One => self.p1,
            Player::Two => self.p2,
        }
    }

    pub fn set_next_player_state(&mut self, state: PlayerState) {
        match self.next {
            Player::One => self.p1 = state,
            Player::Two => self.p2 = state,
        }
    }

    pub fn toggle_next(&mut self) {
        self.next = match self.next {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

pub fn play_dirac(p1: u32, p2: u32) -> Counter<Player> {
    let start_state = DiracDiceState {
        p1: PlayerState {
            position: p1,
            score: 0,
        },
        p2: PlayerState {
            position: p2,
            score: 0,
        },
        next: Player::One,
    };
    win_counts(start_state, &mut HashMap::new())
}

fn win_counts(state: DiracDiceState, memo: &mut HashMap<DiracDiceState, Counter<Player>>) -> Counter<Player> {
    if memo.contains_key(&state) {
        memo.get(&state).unwrap().clone()
    } else {
        let result: Counter<Player> = ROLL_FREQS.iter().flat_map(|&(roll, count)| {
            let mut next_state = state.clone();
            let next_player = next_state.next_player_state();

            let new_position = ((((next_player.position) - 1) + roll) % 10) + 1;
            let new_score = next_player.score + new_position;
            if new_score >= 21 {
                let mut c = Counter::new();
                c.add_count(next_state.next, count);
                c.into_iter()
            } else {
                next_state.set_next_player_state(PlayerState {
                    position: new_position,
                    score: new_score,
                });
                next_state.toggle_next();
                win_counts(next_state, memo);
                let mut c = Counter::new();
                for (player, win_count) in win_counts(next_state, memo).into_iter() {
                    c.add_count(player, win_count * count);
                }
                c.into_iter()
            }
        }).collect();

        memo.insert(state, result.clone());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
