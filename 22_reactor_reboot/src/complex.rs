use std::cmp::{min,max};
use std::ops::RangeBounds;
use std::ops::Bound::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coord {
    fn axis(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}

#[derive(Copy, Clone)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Cuboid {
    s: Coord,
    e: Coord,
}

impl Cuboid {
    pub fn from_ranges(x: impl RangeBounds<i32>, y: impl RangeBounds<i32>, z: impl RangeBounds<i32>) -> Self {
        let start = Coord {
            x: Self::range_to_start(&x),
            y: Self::range_to_start(&y),
            z: Self::range_to_start(&z),
        };

        let end = Coord {
            x: Self::range_to_end(&x),
            y: Self::range_to_end(&y),
            z: Self::range_to_end(&z),
        };

        Self { s: start, e: end }
    }

    fn range_to_start(r: &impl RangeBounds<i32>) -> i32 {
        match r.start_bound() {
            Unbounded => panic!("unbounded range not allowed"),
            Included(&s) => s,
            Excluded(&s) => s + 1,
        }
    }

    fn range_to_end(r: &impl RangeBounds<i32>) -> i32 {
        match r.end_bound() {
            Unbounded => panic!("unbounded range not allowed"),
            Included(&e) => e + 1,
            Excluded(&e) => e,
        }
    }

    pub fn from_pairs(x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> Self {
        for pair in [x, y, z] {
            assert!(pair.0 < pair.1);
        }

        Self {
            s: Coord { x: x.0, y: y.0, z: z.0 },
            e: Coord { x: x.1, y: y.1, z: z.1 },
        }
    }

    pub fn size(&self) -> u128 {
        ((self.e.x - self.s.x) as u128 * (self.e.y - self.s.y) as u128 * (self.e.z - self.s.z) as u128).try_into().unwrap()
    }

    pub fn overlap(&self, other: &Self) -> Option<Self> {
        let x = self.axis_overlap(other, Axis::X);
        let y = self.axis_overlap(other, Axis::Y);
        let z = self.axis_overlap(other, Axis::Z);

        if x.is_some() && y.is_some() && z.is_some() {
            Some(Self::from_pairs(x.unwrap(), y.unwrap(), z.unwrap()))
        } else {
            None
        }
    }

    fn axis_overlap(&self, other: &Self, axis: Axis) -> Option<(i32, i32)> {
        let (this, that) = ((self.s.axis(axis), self.e.axis(axis)), (other.s.axis(axis), other.e.axis(axis)));

        if this.0 < that.1 && this.1 > that.0 {
            Some((max(this.0, that.0), min(this.1, that.1)))
        } else if that.0 < this.1 && that.1 > this.0 {
            Some((max(this.0, that.0), min(this.1, that.1)))
        } else {
            None
        }
    }

    pub fn break_around(&self, other: &Self) -> Option<Vec<Self>> {

        let overlap = self.overlap(other);
        if overlap.is_none() {
            return None;
        }

        let overlap = overlap.unwrap();

        let mut xyz_breaks: [Vec<i32>; 3] = [
            vec![self.s.x],
            vec![self.s.y],
            vec![self.s.z],
        ];

        for (i, axis) in [Axis::X, Axis::Y, Axis::Z].into_iter().enumerate() {
            let axis_overlap = self.axis_overlap(other, axis).unwrap();
            for bound in [axis_overlap.0, axis_overlap.1] {
                if bound != self.s.axis(axis) && bound != self.e.axis(axis) {
                    xyz_breaks[i].push(bound);
                }
            }
        }

        xyz_breaks[0].push(self.e.x);
        xyz_breaks[1].push(self.e.y);
        xyz_breaks[2].push(self.e.z);

        let shards = xyz_breaks[0].windows(2).flat_map(|x| {
            xyz_breaks[1].windows(2).flat_map(|y| {
                xyz_breaks[2].windows(2).map(|z| {
                    Cuboid::from_pairs(
                        (x[0], x[1]),
                        (y[0], y[1]),
                        (z[0], z[1]),
                    )
                })
            })
        }).filter(|shard| *shard != overlap).collect();

        Some(shards)
    }

    pub fn is_initialization(&self) -> bool {
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if self.s.axis(axis) < -50 || self.e.axis(axis) > 51 {
                return false;
            }
        }
        
        true
    }
}

pub struct Reactor {
    non_overlapping: Vec<Cuboid>,
}

impl Reactor {
    pub fn new() -> Self {
        Self {
            non_overlapping: Vec::new(),
        }
    }

    pub fn turn_on(&mut self, cuboid: Cuboid) {
        let mut broken_cuboid = vec![cuboid];
        for overlapping in self.non_overlapping.iter() {
            let mut new_broken_cuboid = Vec::new();
            for shard in broken_cuboid {
                if let Some(mut shards) = shard.break_around(overlapping) {
                    new_broken_cuboid.append(&mut shards);
                } else {
                    new_broken_cuboid.push(shard);
                }
            }
            broken_cuboid = new_broken_cuboid;
        }
        self.non_overlapping.append(&mut broken_cuboid);
    }

    pub fn turn_off(&mut self, cuboid: Cuboid) {
        let mut new_non_overlapping = Vec::new();
        for on in self.non_overlapping.iter() {
            if let Some(mut shards) = on.break_around(&cuboid) {
                new_non_overlapping.append(&mut shards);
            } else {
                new_non_overlapping.push(*on);
            }
        }
        self.non_overlapping = new_non_overlapping;
    }

    pub fn on_count(&self) -> u128 {
        self.non_overlapping.iter().map(|cuboid| cuboid.size()).sum()
    }

    pub fn on_count_initialization(&self) -> u128 {
        let non_overlapping: Vec<Cuboid> = self.non_overlapping.iter()
            .filter_map(|cuboid| cuboid.overlap(&Cuboid::from_ranges(-50..=50, -50..=50, -50..=50)))
            .collect();

        non_overlapping.iter().map(|cuboid| cuboid.size()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuboid_overlap_none() {
        let this = Cuboid::from_pairs((0, 11), (0, 11), (0, 11));
        let that = Cuboid::from_pairs((11, 12), (11, 12), (11, 12));

        assert_eq!(None, this.overlap(&that));
    }

    #[test]
    fn test_cuboid_overlap_partial() {
        let this = Cuboid::from_pairs((0, 11), (0, 11), (0, 11));
        let that = Cuboid::from_pairs((-4, 1), (5, 6), (10, 14));

        let expected = Cuboid::from_pairs((0, 1), (5, 6), (10, 11));

        assert_eq!(expected, this.overlap(&that).unwrap());
    }

    #[test]
    fn test_cuboid_overlap_across() {
        let this = Cuboid::from_ranges(0..51, 0..51, 0..51);   
        let that = Cuboid::from_ranges(0..51, 0..51, 30..36);

        assert_eq!(that, this.overlap(&that).unwrap());
    }

    #[test]
    fn test_cuboid_overlap_other_contains_self() {
        let this = Cuboid::from_pairs((0, 11), (0, 11), (0, 11));
        let that = Cuboid::from_pairs((-100, 101), (-100, 101), (-100, 101));

        assert_eq!(this, this.overlap(&that).unwrap());
    }

    #[test]
    fn test_cuboid_overlap_self_contains_other() {
        let this = Cuboid::from_pairs((0, 11), (0, 11), (0, 11));
        let that = Cuboid::from_pairs((7, 8), (7, 8), (7, 8));

        assert_eq!(that, this.overlap(&that).unwrap());
    }

    #[test]
    fn test_cuboid_break_around_none() {
        let to_break = Cuboid::from_pairs((0, 51), (0, 51), (0, 51));
        let breaker = Cuboid::from_pairs((100, 101), (100, 101), (100, 101));

        assert_eq!(None, to_break.break_around(&breaker));
    }

    #[test]
    fn test_cuboid_break_around_corner() {
        let to_break = Cuboid::from_ranges(0..51, 0..51, 0..51);  
        let breaker = Cuboid::from_ranges(0..21, 0..21, 0..21);

        let expected: Vec<Cuboid> = vec![
            (21..51, 0..21, 0..21),
            (0..21, 21..51, 0..21),
            (0..21, 0..21, 21..51),
            (0..21, 21..51, 21..51),
            (21..51, 0..21, 0..21),
            (21..51, 21..51, 0..21),
            (21..51, 21..51, 21..51),
        ].into_iter().map(|ranges| Cuboid::from_ranges(ranges.0, ranges.1, ranges.2)).collect();

        let actual = to_break.break_around(&breaker).unwrap();

        assert_eq!(expected.len(), actual.len());
        for shard in expected {
            assert!(actual.contains(&shard), "break did not generate shard {:?}", shard)
        }
    }

    #[test]
    fn test_cuboid_break_around_edge() {
        let to_break = Cuboid::from_ranges(0..51, 0..51, 0..51); 
        let breaker = Cuboid::from_ranges(0..51, 0..11, 0..51);

        let expected: Vec<Cuboid> = vec![
            (0..51, 11..51, 0..51),
        ].into_iter().map(|ranges| Cuboid::from_ranges(ranges.0, ranges.1, ranges.2)).collect();

        let actual = to_break.break_around(&breaker).unwrap();

        assert_eq!(expected.len(), actual.len());
        for shard in expected {
            assert!(actual.contains(&shard), "break did not generate shard {:?}", shard)
        }   
    }

    #[test]
    fn test_cuboid_break_around_across() {
        let to_break = Cuboid::from_ranges(0..51, 0..51, 0..51);   
        let breaker = Cuboid::from_ranges(0..51, 0..51, 30..36);  

        let expected: Vec<Cuboid> = vec![
            (0..51, 0..51, 0..30),
            (0..51, 0..51, 36..51),
        ].into_iter().map(|ranges| Cuboid::from_ranges(ranges.0, ranges.1, ranges.2)).collect();

        let actual = to_break.break_around(&breaker).unwrap();

        assert_eq!(expected.len(), actual.len());
        for shard in expected {
            assert!(actual.contains(&shard), "break did not generate shard {:?}", shard)
        }   
    }

    #[test]
    fn test_cuboid_break_around_middle() {
        let to_break = Cuboid::from_ranges(0..51, 0..51, 0..51);   
        let breaker = Cuboid::from_ranges(4..5, 10..16, 27..39);  

        // let expected: Vec<Cuboid> = vec![

        // ].into_iter().map(|ranges| Cuboid::from_ranges(ranges.0, ranges.1, ranges.2)).collect();

        let actual = to_break.break_around(&breaker).unwrap();

        assert_eq!(26, actual.len());
        // for shard in expected {
        //     assert!(actual.contains(&shard), "break did not generate shard {:?}", shard)
        // }   
    }

    #[test]
    fn test_cuboid_break_around_all() {
        let to_break = Cuboid::from_ranges(0..51, 0..51, 0..51);   
        let breaker = Cuboid::from_ranges(-100..100, -100..100, -100..100); 

        let actual = to_break.break_around(&breaker).unwrap();

        assert_eq!(0, actual.len());   
    }

    #[test]
    fn test_reactor_on_count_simple() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..2, 0..2, 0..2));
        reactor.turn_on(Cuboid::from_ranges(1..3, 1..3, 1..3));
        reactor.turn_off(Cuboid::from_ranges(2..4, 2..4, 2..4));

        let expected = 14;

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_no_overlap() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..2, 0..2, 0..2));
        reactor.turn_on(Cuboid::from_ranges(4..6, 4..6, 4..6));
        reactor.turn_off(Cuboid::from_ranges(2..4, 2..4, 2..4));

        let expected = 16;

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_total_overlap() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..10, 0..10, 0..10));
        reactor.turn_on(Cuboid::from_ranges(4..6, 4..6, 4..6));

        let expected = 10 * 10 * 10;

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_middle_off() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..10, 0..10, 0..10));
        reactor.turn_off(Cuboid::from_ranges(4..6, 4..6, 4..6));

        let expected = 1000 - 8;

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_on_corner_overlap() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..10, 0..10, 0..10));
        reactor.turn_on(Cuboid::from_ranges(5..15, 5..15, 5..15));

        let expected = 2000 - 125;

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_on_off_edge_overlap() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..10, 0..10, 0..10));
        reactor.turn_on(Cuboid::from_ranges(5..15, 0..10, 0..10));
        reactor.turn_off(Cuboid::from_ranges(13..20, 0..10, 0..10));

        let expected = 2000 - 500 - (10*10*2);

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_many_on_corner_overlaps() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..10, 0..10, 0..10));
        reactor.turn_on(Cuboid::from_ranges(7..17, 7..17, 7..17));
        reactor.turn_on(Cuboid::from_ranges(14..24, 14..24, 14..24));
        reactor.turn_on(Cuboid::from_ranges(21..31, 21..31, 21..31));

        let expected = (1000 * 4) - (27 * 3);

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_many_on_corner_overlaps_big() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..4, 0..4, 0..4));
        reactor.turn_on(Cuboid::from_ranges(3..7, 3..7, 3..7));
        reactor.turn_on(Cuboid::from_ranges(6..10, 6..10, 6..10));
        reactor.turn_on(Cuboid::from_ranges(9..13, 9..13, 9..13));

        let expected = (64 * 4) - (1 * 3);

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_many_on_edge_overlaps() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..3, 0..1, 0..1));
        reactor.turn_on(Cuboid::from_ranges(2..5, 0..1, 0..1));
        reactor.turn_on(Cuboid::from_ranges(4..7, 0..1, 0..1));
        reactor.turn_on(Cuboid::from_ranges(6..9, 0..1, 0..1));

        let expected = (3 * 4) - (1 * 3);

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_turn_off_multiple() {
        let mut reactor = Reactor::new();
        reactor.turn_on(Cuboid::from_ranges(0..3, 0..3, 0..1));
        reactor.turn_on(Cuboid::from_ranges(0..3, 5..8, 0..1));
        reactor.turn_on(Cuboid::from_ranges(5..8, 0..3, 0..1));
        reactor.turn_on(Cuboid::from_ranges(5..8, 5..8, 0..1));
        reactor.turn_off(Cuboid::from_ranges(2..6, 2..6, 0..1));

        let expected = (9 * 4) - 4;

        assert_eq!(expected, reactor.on_count());
    }

    #[test]
    fn test_reactor_on_count_multiple_total_overlap() {
        let mut reactor = Reactor::new();
        for n in [0, 2, 4, 6, 8, 10, 12] {
            reactor.turn_on(Cuboid::from_ranges(n..100, n..100, n..100));
        }

        let expected = 100 * 100 * 100;

        assert_eq!(expected, reactor.on_count());
    }
}