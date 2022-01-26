use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Self::Amber),
            'B' => Some(Self::Bronze),
            'C' => Some(Self::Copper),
            'D' => Some(Self::Desert),
            _ => None
        }
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Amber => 'A',
            Self::Bronze => 'B',
            Self::Copper => 'C',
            Self::Desert => 'D',
        })
    }
}

use hallway::Hallway;
use room::Room;

#[derive(Debug, Clone, Serialize)]
pub struct Burrow {
    hallway: Hallway,
    // Smart pointer; Room is immutable, so we want to share the same room
    //  across burrow copies if it hasn't changed.
    rooms: [Rc<Room>; 4],
    room_size: usize,
}

impl Burrow {
    pub fn new(rooms: &[Vec<Amphipod>]) -> Self {
        assert!(rooms.len() == 4, "invalid room count");
        let room_size = rooms[0].len();
        assert!(rooms.iter().all(|r| r.len() == room_size), "provided rooms do not match size");
        

        let rooms = [
            Rc::new(Room::new(Amphipod::Amber, &rooms[0])),
            Rc::new(Room::new(Amphipod::Bronze, &rooms[1])),
            Rc::new(Room::new(Amphipod::Copper, &rooms[2])),
            Rc::new(Room::new(Amphipod::Desert, &rooms[3])),
        ];
        
        Self {
            hallway: Hallway::new(),
            rooms,
            room_size,
        }
    }

    fn hallway_to_room(&self, space: usize, room: Amphipod, depth: usize) -> (u32, Self) {
        let mover = self.hallway.occupant(space).unwrap();
        let energy = self.move_energy(mover, room, depth, space);

        let next_hallway = self.hallway.remove_to_room(space, room);
        let next_room = self.room(room).insert(mover, depth);
        let mut next_rooms = self.rooms.clone();  // cloning an Rc changes ref count
        next_rooms[self.room_index(room)] = Rc::new(next_room);

        let next_burrow = Self {
            hallway: next_hallway,
            rooms: next_rooms,
            ..*self
        };

        (energy, next_burrow)
    }

    fn room_to_hallway(&self, room: Amphipod, depth: usize, space: usize) -> (u32, Self) {
        let mover = self.room(room).occupant(depth).unwrap();
        let energy = self.move_energy(mover, room, depth, space);

        let next_hallway = self.hallway.insert_from_room(mover, room, space);
        let next_room = self.room(room).remove(depth);
        let mut next_rooms = self.rooms.clone();  // cloning an Rc changes ref count
        next_rooms[self.room_index(room)] = Rc::new(next_room);

        let next_burrow = Self {
            hallway: next_hallway,
            rooms: next_rooms,
            ..*self
        };

        (energy, next_burrow)
    }

    fn move_energy(&self, mover: Amphipod, room: Amphipod, depth: usize, space: usize) -> u32 {
        let (x_start, x_end) = (space, Hallway::room_entrance(room));
        let dx = (max(x_start, x_end) - min(x_start, x_end)) as u32;
        let dy = (depth + 1) as u32;

        (dx + dy) * Self::step_energy(mover)
    }

    fn step_energy(amphipod: Amphipod) -> u32 {
        match amphipod {
            Amphipod::Amber  => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    pub fn min_energy(&self) -> Option<(Vec<Burrow>, u32)> {
        let mut memo = HashMap::new();
        self.min_energy_rec(&mut memo, 0, None)
            .map(|(states, energy)| (states.into_iter().rev().collect(), energy))
    }

    fn min_energy_rec(&self, memo: &mut HashMap<Vec<u8>, u32>, energy: u32, known_min: Option<u32>) -> Option<(Vec<Burrow>, u32)> {
        // update memo, return early if this state has already been reached with less energy
        let burrow_serialized = serde_json::to_vec(self).unwrap();
        if memo.contains_key(&burrow_serialized) && *memo.get(&burrow_serialized).unwrap() <= energy {
            return None;
        } else {
            memo.insert(burrow_serialized, energy);
        }

        if self.is_solved() {
            return Some((vec![self.clone()], energy));
        }

        let mut solutions = Vec::new();
        let mut known_min = known_min;

        // attempt to move amphipods from hallways to rooms
        for (space, amphipod) in self.hallway.occupied() {
            if self.hallway.can_remove_to(space, amphipod) && self.room(amphipod).can_insert(amphipod).is_some() {
                if let Some(depth) = self.room(amphipod).can_insert(amphipod) {
                    let (added_energy, next_burrow) = self.hallway_to_room(space, amphipod, depth);
                    if known_min.is_some() && energy + added_energy > known_min.unwrap() {
                        continue;
                    }
                    if let Some(solution) = next_burrow.min_energy_rec(memo, energy + added_energy, known_min) {
                        if known_min.is_none() || solution.1 < known_min.unwrap() {
                            known_min = Some(solution.1);
                        }
                        solutions.push(solution);
                    }
                }
            }
        }

        // attempt to move amphipods from rooms to hallways
        for room in self.rooms.iter() {
            for depth in 0..self.room_size {
                if room.can_remove(depth) {
                    let native = room.native();
                    for space in self.hallway.reachable_from(native) {
                        let (added_energy, next_burrow) = self.room_to_hallway(native, depth, space);
                        if known_min.is_some() && energy + added_energy > known_min.unwrap() {
                            continue;
                        }
                        if let Some(solution) = next_burrow.min_energy_rec(memo, energy + added_energy, known_min) {
                            if known_min.is_none() || solution.1 < known_min.unwrap() {
                                known_min = Some(solution.1);
                            }
                            solutions.push(solution);
                        }
                    }
                }
            }
        }

        // take minimum of gathered energies
        let min_solution = solutions.into_iter().min_by(|x, y| x.1.cmp(&y.1));
        min_solution.map(|(mut states, energy)| {
            states.push(self.clone());
            (states, energy)
        })
    }

    fn is_solved(&self) -> bool {
        self.rooms.iter()
            .all(|r| r.is_complete())
    }

    fn room(&self, amphipod: Amphipod) -> &Room {
        &self.rooms[self.room_index(amphipod)]
    }

    fn room_index(&self, amphipod: Amphipod) -> usize {
        match amphipod {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("#############\n#");
        for space in 0..11 {
            result.push_str(&match self.hallway.occupant(space) {
                None => String::from("."),
                Some(amphipod) => format!("{}", amphipod),
            });
        }
        result.push_str("#\n");
        for depth in 0..self.room_size {
            result.push_str(match depth {
                0 => "###",
                _ => "  #",
            });

            for amphipod in [Amphipod::Amber, Amphipod::Bronze, Amphipod::Copper, Amphipod::Desert] {
                result.push_str(&format!("{}#", match self.room(amphipod).occupant(depth) {
                    None => String::from("."),
                    Some(occupant) => format!("{}", occupant),
                }));
            }
            
            result.push_str(match depth {
                0 => "##\n",
                _ => "\n",
            });
        }
        result.push_str("  #########");

        write!(f, "{}", result)
    }
}

mod hallway {
    use std::cmp::{min, max};
    use serde::Serialize;

    use crate::Amphipod;

    #[derive(Debug, Copy, Clone, Serialize)]
    pub struct Hallway {
        spaces: [Option<Amphipod>; 11],
    }

    const ROOM_ENTRANCES: [usize; 4] = [2, 4, 6, 8];
    
    impl Hallway {
        pub fn new() -> Self {
            let hallway = Self {
                spaces: [None; 11],
            };
            hallway.check_rep();
            hallway
        }

        pub fn insert_from_room(&self, amphipod: Amphipod, from: Amphipod, to: usize) -> Self {
            assert!(self.can_insert_from(from, to), "insertion path is blocked");

            let mut new_spaces = self.spaces.clone();
            new_spaces[to].replace(amphipod);
            let hallway = Self {
                spaces: new_spaces,
            };
            hallway.check_rep();
            hallway
        }

        pub fn remove_to_room(&self, from: usize, to: Amphipod) -> Self {
            assert!(self.can_remove_to(from, to), "removal path is blocked");

            let mut new_spaces = self.spaces.clone();
            new_spaces[from].take();
            let hallway = Self {
                spaces: new_spaces,
            };
            hallway.check_rep();
            hallway
        }

        fn check_rep(&self) {
            ROOM_ENTRANCES.into_iter()
                .for_each(|i| assert!(self.spaces[i].is_none(),
                "spaces outside rooms cannot be occupied"));
        }

        pub fn can_insert_from(&self, from: Amphipod, to: usize) -> bool {
            assert!(to < self.spaces.len(), "destination not in hallway");
            assert!(!ROOM_ENTRANCES.contains(&to), "destination cannot be room entrance");

            (min(to, Self::room_entrance(from))..=max(to, Self::room_entrance(from)))
                .all(|i| self.spaces[i].is_none())
        }

        pub fn can_remove_to(&self, from: usize, to: Amphipod) -> bool {
            assert!(from < self.spaces.len(), "start not in hallway");
            assert!(!ROOM_ENTRANCES.contains(&from), "start cannot be room entrance");
            assert!(self.spaces[from].is_some(), "start is empty");

            // this is rough; I want to peek the option value without unwrapping
            self.spaces[from].unwrap() == to
            && (min(from, Self::room_entrance(to))..=max(from, Self::room_entrance(to)))
                .all(|i| i == from || self.spaces[i].is_none())
        }

        pub fn occupant(&self, space: usize) -> Option<Amphipod> {
            self.spaces[space]
        }

        pub fn occupied(&'_ self) -> impl Iterator<Item=(usize, Amphipod)> + '_ {
            self.spaces.iter().enumerate().filter_map(|(i, a)| {
                if a.is_some() {
                    Some((i, a.unwrap()))
                } else {
                    None
                }
            })
        }

        pub fn _occupied_by(&'_ self, amphipod: Amphipod) -> impl Iterator<Item=usize> + '_ {
            self.spaces.iter().enumerate().filter_map(move |(i, a)| {
                if let Some(occupant) = a {
                    if *occupant == amphipod {
                        return Some(i)
                    }
                }
                None
            })
        }

        pub fn reachable_from(&'_ self, room: Amphipod) -> impl Iterator<Item=usize> + '_ {
            // this method signature is awesome!
            let start = Self::room_entrance(room);

            // let left: Vec<usize> = (0..start).rev().collect();
            // let right: Vec<usize> = (start+1..self.spaces.len()).collect();

            // [left, right].into_iter().flat_map(|r| {
            //     r.into_iter().filter(|i| !ROOM_ENTRANCES.contains(&i)).take_while(|i| !self.spaces[*i].is_some())
            // })

            // not a huge fan of this... would like something more like the above
            (0..start).rev().into_iter()
            .filter(|i| !ROOM_ENTRANCES.contains(&i)).take_while(|i| !self.spaces[*i].is_some())
            .chain(
                (start+1..self.spaces.len()).into_iter()
                .filter(|i| !ROOM_ENTRANCES.contains(&i)).take_while(|i| !self.spaces[*i].is_some())
            )
        }

        pub fn room_entrance(native: Amphipod) -> usize {
            ROOM_ENTRANCES[match native {
                Amphipod::Amber => 0,
                Amphipod::Bronze => 1,
                Amphipod::Copper => 2,
                Amphipod::Desert => 3,
            }]
        }
    }
}

mod room {
    use crate::Amphipod;

    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    pub struct Room {
        size: usize,
        // who should live in the pod
        native: Amphipod,
        // who actually is in the pod; [upper, lower]
        occupants: Vec<Option<Amphipod>>,
    }

    impl Room {
        pub fn new(native: Amphipod, occupants: &[Amphipod]) -> Self {
            let room = Self {
                size: occupants.len(),
                native,
                occupants: occupants.into_iter().map(|&a| Some(a)).collect(),
            };
            room.check_rep();
            room
        }

        pub fn insert(&self, amphipod: Amphipod, depth: usize) -> Self {
            self.check_depth(depth);
            assert!(self.is_unoccupied(depth), "cannot insert into occupied position");
            assert!(self.can_insert(amphipod).is_some(), "invalid insert into {}", depth);

            let mut new_occupants = self.occupants.clone();
            new_occupants[depth].replace(amphipod);
            let room = Self {
                occupants: new_occupants,
                ..*self
            };
            room.check_rep();
            room
        }

        pub fn remove(&self, depth: usize) -> Self {
            self.check_depth(depth);
            assert!(self.occupant(depth).is_some(), "cannot remove from empty position");
            assert!(self.can_remove(depth), "invalid remove from {}", depth);
            
            let mut new_occupants = self.occupants.clone();
            new_occupants[depth].take();
            let room = Self {
                occupants: new_occupants,
                ..*self
            };
            room.check_rep();
            room
        }

        fn check_rep(&self) {
            assert_eq!(self.size, self.occupants.len(), "occupants size is not room size");
            // there cannot be a "floating occupant"
            let no_floaters = &self.occupants[..]
                .windows(2)
                .all(|w| {
                    !(w[0].is_some() && w[1].is_none())
                });
            assert!(no_floaters, "room contains floating occupant");
        }

        pub fn occupant(&self, depth: usize) -> Option<Amphipod> {
            self.check_depth(depth);
            self.occupants[depth]
        }

        pub fn _positions(&self, amphipod: Amphipod) -> Vec<usize> {
            (0..self.size).into_iter()
                .filter(|i| self.occupant(*i).map_or(false, |a| a == amphipod))
                .collect()
        }

        pub fn native(&self) -> Amphipod {
            self.native
        }

        pub fn is_complete(&self) -> bool {
            (0..self.size).into_iter()
                .all(|i| self.is_native(i))
        }

        fn _is_empty(&self) -> bool {
            (0..self.size).into_iter()
                .all(|i| self.is_unoccupied(i))
        }

        pub fn can_remove(&self, depth: usize) -> bool {
            self.check_depth(depth);

            let foreign_below = |d| (d + 1..self.size).into_iter().any(|i| !self.is_native(i));
            let empty_above = |d| (0..d).into_iter().all(|i| self.is_unoccupied(i));

            self.occupant(depth).is_some()
            && if self.is_native(depth) {
                foreign_below(depth) && empty_above(depth)
            } else {
                empty_above(depth)
            }
        }

        pub fn can_insert(&self, amphipod: Amphipod) -> Option<usize> {
            if amphipod != self.native {
                return None
            }
            
            let unoccupied_or_native = (0..self.size)
                .all(|i| self.is_unoccupied(i) || self.is_native(i));

            if !unoccupied_or_native {
                return None
            }
            
            (0..self.size).rev().find(|i| self.is_unoccupied(*i))
        }

        fn is_native(&self, depth: usize) -> bool {
            self.check_depth(depth);
            self.occupant(depth).map_or(false, |a| a == self.native)
        }

        fn is_unoccupied(&self, depth: usize) -> bool {
            self.check_depth(depth);
            self.occupant(depth).is_none()
        }

        fn check_depth(&self, depth: usize) {
            assert!(depth < self.size, "depth out of room range");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burrow_example_small() {
        let burrow = Burrow::new(&vec![
            vec! [
                Amphipod::Bronze,
                Amphipod::Amber,
            ],
            vec![
                Amphipod::Copper,
                Amphipod::Desert,
            ],
            vec![
                Amphipod::Bronze,
                Amphipod::Copper,
            ],
            vec![
                Amphipod::Desert,
                Amphipod::Amber,
            ],
        ]);

        let expected = 12521;

        assert_eq!(expected, burrow.min_energy().unwrap().1);
    }

    #[test]
    fn burrow_example_large() {
        let burrow = Burrow::new(&vec![
            vec! [
                Amphipod::Bronze,
                Amphipod::Desert,
                Amphipod::Desert,
                Amphipod::Amber,
            ],
            vec![
                Amphipod::Copper,
                Amphipod::Copper,
                Amphipod::Bronze,
                Amphipod::Desert,
            ],
            vec![
                Amphipod::Bronze,
                Amphipod::Bronze,
                Amphipod::Amber,
                Amphipod::Copper,
            ],
            vec![
                Amphipod::Desert,
                Amphipod::Amber,
                Amphipod::Copper,
                Amphipod::Amber,
            ],
        ]);

        let expected = 44169;

        assert_eq!(expected, burrow.min_energy().unwrap().1);
    }
}
