use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord { x, y, z }
    }

    pub fn translate(&self, d: &Coord) -> Coord {
        Coord {
            x: self.x + d.x,
            y: self.y + d.y,
            z: self.z + d.z,
        }
    }

    pub fn delta(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn manhattan(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() +
        (self.y - other.y).abs() +
        (self.z - other.z).abs()
    }
}

pub type Beacon = Coord;

mod scanner {
    use ndarray::{arr2, Array, Array2, ArrayView};
    use crate::Beacon;

    const ROT_X: [[i32; 3]; 3] = [[1, 0,  0],
                                  [0, 0, -1],
                                  [0, 1,  0],
    ];
    
    const ROT_Y: [[i32; 3]; 3] = [[ 0, 0, 1],
                                  [ 0, 1, 0],
                                  [-1, 0, 0],
    ];
    
    const ROT_Z: [[i32; 3]; 3] = [[0, -1, 0],
                                  [1,  0, 0],
                                  [0,  0, 1],
    ];

    #[derive(Clone)]
    pub struct Scanner {
        beacons: Array2<i32>,
    }
    
    impl Scanner {
        pub fn from(beacons: &Vec<Beacon>) -> Scanner {
            let mut beacons_matrix = Array::zeros((0, 3));
            beacons.iter().for_each(|b| {
                beacons_matrix.push_row(ArrayView::from(&[b.x, b.y, b.z])).unwrap();
            });
    
            Scanner { beacons: beacons_matrix }
        }
    
        pub fn overlaps(&self, other: &Scanner, min: usize) -> Option<Beacon> {
            let this_beacons: Vec<_> = self.beacons();
            let that_beacons: Vec<_> = other.beacons();

            let test_count = this_beacons.len() - min + 1;

            for this in this_beacons.iter().take(test_count) {
                for that in that_beacons.iter() {
                    let delta = this.delta(that);
                    let overlap_count = that_beacons
                        .iter().cloned()
                        .map(|b| b.translate(&delta))
                        .filter(|b| this_beacons.contains(b))
                        .count();

                    if overlap_count >= min {
                        return Some(delta)
                    }
                }
            }
    
            None
        }
    
        pub fn beacons(&self) -> Vec<Beacon> {
            self.beacons.rows().into_iter().map(|row| {
                let mut r = row.iter().cloned();
                let x = r.next().unwrap();
                let y = r.next().unwrap();
                let z = r.next().unwrap();
                Beacon { x, y, z }
            }).collect()
        }
    
        pub fn perms(&self) -> Vec<Scanner> {
            let mut result = Vec::new();
            let mut matrix = self.beacons.clone();
            
            for _ in 0..3 {
                matrix = matrix.dot(&arr2(&ROT_X));
                for _ in 0..4 {
                    matrix = matrix.dot(&arr2(&ROT_Z));
                    result.push(Scanner { beacons: matrix.clone() })
                }
                matrix = matrix.dot(&arr2(&ROT_Y));
                for _ in 0..4 {
                    matrix = matrix.dot(&arr2(&ROT_Z));
                    result.push(Scanner { beacons: matrix.clone() })
                }
            }

            result
        }
    }
}

pub use scanner::Scanner;

pub struct ScannerSet {
    placements: Vec<(Scanner, Coord)>,
}

impl ScannerSet {
    pub fn assemble(scanners: Vec<Scanner>, overlap_min: usize) -> ScannerSet {
        /*
        take the first scanner in the list and placed at (0, 0, 0);
        remaining scanners are everything but the first

        while remanining scanners.len() > 0 {
            for each remaining scanner
                for each perm of that scanner
                    for each scanner in placed
                        if placed.overlaps(perm)
                            add placement of placed to placement of overlap
                            remove perm from overlap
                            restart
        }
        */

        let mut placements: Vec<(Scanner, Coord)> = Vec::new();
        let mut remaining = scanners;
        let initial = remaining.remove(0);
        placements.push((initial, Coord::new(0, 0, 0)));

        while remaining.len() > 0 {
            let mut placement = None;
            'outer: for (i, scanner) in remaining.iter().enumerate() {
                for perm in scanner.perms() {
                    for (placed, position) in placements.iter() {
                        if let Some(delta) = placed.overlaps(&perm, overlap_min) {
                            placement = Some((perm, position.translate(&delta)));
                            remaining.remove(i);
                            break 'outer;
                        }
                    }
                }
            }
            placements.push(placement.expect("didn't find an overlap"));
        }

        ScannerSet { placements }
    }

    pub fn unqiue_beacons(&self) -> Vec<Beacon> {
        let mut uniques = HashSet::new();

        for (scanner, origin) in self.placements.iter() {
            for beacon in scanner.beacons() {
                uniques.insert(beacon.translate(&origin));
            }
        }

        uniques.into_iter().collect()
    }

    pub fn max_manhattan(&self) -> i32 {
        self.placements.iter().map(|&(_, a)| {
            self.placements.iter().map(|&(_, b)| a.manhattan(&b)).max().unwrap()
        }).max().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::scanner::*;

    #[test]
    fn overlaps_small() {
        let s0 = Scanner::from(&[
            (1, 1, 1),
            (2, 3, 4),
            (799, 44, -3),
        ].iter().map(|b| Beacon { x: b.0, y: b.1, z: b.2 }).collect());
        let s1 = Scanner::from(&[
            (2, 2, 2),
            (3, 4, 5),
            (800, 45, -2)
        ].iter().map(|b| Beacon { x: b.0, y: b.1, z: b.2 }).collect());

        let expected = Beacon::new(-1, -1, -1);

        for perm in s1.perms().iter() {
            if let Some(delta) = s0.overlaps(perm, 3) {    
                assert_eq!(expected, delta);
                return;
            }
        }

        panic!("did not find overlap");
    }

    #[test]
    fn overlaps_big() {
        let s0 = Scanner::from(&[
            (404,-588,-901),
            (528,-643,409),
            (-838,591,734),
            (390,-675,-793),
            (-537,-823,-458),
            (-485,-357,347),
            (-345,-311,381),
            (-661,-816,-575),
            (-876,649,763),
            (-618,-824,-621),
            (553,345,-567),
            (474,580,667),
            (-447,-329,318),
            (-584,868,-557),
            (544,-627,-890),
            (564,392,-477),
            (455,729,728),
            (-892,524,684),
            (-689,845,-530),
            (423,-701,434),
            (7,-33,-71),
            (630,319,-379),
            (443,580,662),
            (-789,900,-551),
            (459,-707,401),
        ].iter().map(|b| Beacon { x: b.0, y: b.1, z: b.2 }).collect());
        let s1 = Scanner::from(&[
            (686,422,578),
            (605,423,415),
            (515,917,-361),
            (-336,658,858),
            (95,138,22),
            (-476,619,847),
            (-340,-569,-846),
            (567,-361,727),
            (-460,603,-452),
            (669,-402,600),
            (729,430,532),
            (-500,-761,534),
            (-322,571,750),
            (-466,-666,-811),
            (-429,-592,574),
            (-355,545,-477),
            (703,-491,-529),
            (-328,-685,520),
            (413,935,-424),
            (-391,539,-444),
            (586,-435,557),
            (-364,-763,-893),
            (807,-499,-711),
            (755,-354,-619),
            (553,889,-390),
        ].iter().map(|b| Beacon { x: b.0, y: b.1, z: b.2 }).collect());

        let expected = Beacon::new(68,-1246,-43);

        for perm in s1.perms().iter() {
            if let Some(delta) = s0.overlaps(perm, 12) {    
                assert_eq!(expected, delta);
                return;
            }
        }

        panic!("did not find overlap");
    }
}
