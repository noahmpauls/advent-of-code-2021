use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
pub struct Cuboid {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Cuboid {
    pub fn new(x: RangeInclusive<i32>, y: RangeInclusive<i32>, z: RangeInclusive<i32>) -> Self {
        Cuboid { x, y, z }
    }

    pub fn x(&self) -> RangeInclusive<i32> {
        self.x.clone()
    }

    pub fn y(&self) -> RangeInclusive<i32> {
        self.y.clone()
    }

    pub fn z(&self) -> RangeInclusive<i32> {
        self.z.clone()
    }

    pub fn is_initialization(&self) -> bool {
        for r in [&self.x, &self.y, &self.z] {
            if r.start() < &-51 || r.end() > &50 {
                return false;
            }
        }
        
        true
    }
}

pub struct Reactor {
    on: HashSet<Coord>,
}

impl Reactor {
    pub fn new() -> Self {
        Reactor { on: HashSet::new() }
    }

    pub fn turn_on(&mut self, cuboid: Cuboid) {
        for x in cuboid.x() {
            for y in cuboid.y() {
                for z in cuboid.z() {
                    self.on.insert(Coord { x, y, z});
                }
            }
        }
    }

    pub fn turn_off(&mut self, cuboid: Cuboid) {
        for x in cuboid.x() {
            for y in cuboid.y() {
                for z in cuboid.z() {
                    self.on.remove(&Coord { x, y, z});
                }
            }
        }
    }

    pub fn on_count(&self) -> usize {
        self.on.len()
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