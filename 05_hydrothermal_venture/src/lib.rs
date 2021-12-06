#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new (x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;
    fn add(self, _rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Compass {
    N, S, E, W, NW, SW, NE, SE,
}

impl Compass {
    fn delta(&self) -> Coord {
        match self {
            Compass::N  => Coord { x:  0, y:  1},
            Compass::S  => Coord { x:  0, y: -1},
            Compass::E  => Coord { x:  1, y:  0},
            Compass::W  => Coord { x: -1, y:  0},
            Compass::NW => Coord { x: -1, y:  1},
            Compass::SW => Coord { x: -1, y: -1},
            Compass::NE => Coord { x:  1, y:  1},
            Compass::SE => Coord { x:  1, y: -1},
        }
    }
}

// A line segment that is either horizontal, vertical, or has a slope of exactly
// +/-1.
pub struct CompassLineSegment {
    slope: Compass,
    integer_coords: Vec<Coord>,
}

impl CompassLineSegment {
    pub fn new(c1: Coord, c2: Coord) -> CompassLineSegment {
        assert_ne!(c1, c2);

        let mut integer_coords = vec![c1];
        let slope = Self::find_slope(&c1, &c2);
        let delta = slope.delta();
        loop {
            let next = *integer_coords.last().unwrap() + delta;
            integer_coords.push(next);
            if next == c2 {
                break;
            }
        }

        CompassLineSegment{ slope, integer_coords }
    }

    pub fn integer_coords(&self) -> Vec<Coord> {
        self.integer_coords.clone()
    }

    pub fn slope(&self) -> Coord {
        self.slope.delta()
    }

    pub fn is_up_down(&self) -> bool {
        match self.slope {
            Compass::N | Compass::S | Compass::E | Compass::W => true,
            _ => false,
        }
    }

    // Integer slope of line from c1 to c2
    fn find_slope(c1: &Coord, c2: &Coord) -> Compass {
        if c1.x == c2.x {
            if c1.y > c2.y {
                Compass::S
            } else {
                Compass::N
            }
        } else if c1.y == c2.y {
            if c1.x > c2.x {
                Compass::W
            } else {
                Compass::E
            }
        } else {
            assert_eq!((c2.y - c1.y).abs(), (c2.x - c1.x).abs(), "line is not a compass line");
            let m = (c2.y - c1.y) / (c2.x - c1.x);
            if m == 1 {
                if c1.y > c2.y {
                    Compass::SW
                } else {
                    Compass::NE
                }
            } else if m == -1 {
                if c1.y > c2.y {
                    Compass::SE
                } else {
                    Compass::NW
                }
            } else {
                panic!("slope is not a compass direction")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_north() {
        let (c1, c2) = (Coord::new(0, 0), Coord::new(0, 5));
        let line = CompassLineSegment::new(c1, c2);
        let expected: Vec<Coord> = (0..=5).map(|v| Coord::new(0, v)).collect();

        assert_eq!(expected, line.integer_coords());
    }

    #[test]
    fn line_west() {
        let (c1, c2) = (Coord::new(7, -7), Coord::new(-10, -7));
        let line = CompassLineSegment::new(c1, c2);
        let expected: Vec<Coord> = (-10..=7).rev().map(|v| Coord::new(v, -7)).collect();

        assert_eq!(expected, line.integer_coords());
    }

    #[test]
    fn line_southeast() {
        let (c1, c2) = (Coord::new(0, 0), Coord::new(3, -3));
        let line = CompassLineSegment::new(c1, c2);
        let expected: Vec<Coord> = (0..=3).map(|v| Coord::new(v, -v)).collect();

        assert_eq!(expected, line.integer_coords());
    }
}
