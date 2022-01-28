use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    r: usize,
    c: usize,
}

impl Coord {
    fn new(r: usize, c: usize) -> Self {
        Self { r, c }
    }
}

pub struct Herd {
    rows: usize,
    cols: usize,
    east: HashSet<Coord>,
    south: HashSet<Coord>,
}

impl Herd {
    pub fn parse(herd: &str) -> Self {
        let lines: Vec<_> = herd.lines().collect();
        let rows = lines.len();
        let cols = lines[0].chars().count();

        let (mut east, mut south) = (HashSet::new(), HashSet::new());
        for (r, line) in lines.into_iter().enumerate() {
            assert_eq!(cols, line.chars().count(), "inconsistent row length in input");
            for (c, char) in line.chars().enumerate() {
                match char {
                    '>' => east.insert(Coord::new(r, c)),
                    'v' => south.insert(Coord::new(r, c)),
                    _ => continue,
                };
            }
        }

        Self {
            rows, cols, east, south
        }
    }

    pub fn steps_to_stasis(&mut self) -> u32 {
        let mut steps = 0;

        while !self.step() {
            steps += 1;
        }

        steps + 1
    }

    fn step(&mut self) -> bool {
        let (mut neast, mut nsouth) = (HashSet::new(), HashSet::new());
        let mut stasis = true;

        // east first
        for sc in self.east.iter() {
            let step = self.move_east(*sc);
            if self.east.contains(&step) || self.south.contains(&step) {
                neast.insert(*sc);
            } else {
                neast.insert(step);
                stasis = false;
            }
        }

        self.east = neast;

        // south last
        for sc in self.south.iter() {
            let step = self.move_south(*sc);
            if self.east.contains(&step) || self.south.contains(&step) {
                nsouth.insert(*sc);
            } else {
                nsouth.insert(step);
                stasis = false;
            }
        }

        self.south = nsouth;

        stasis
    }

    fn move_east(&self, coord: Coord) -> Coord {
        Coord::new(coord.r, (coord.c + 1) % self.cols)
    }

    fn move_south(&self, coord: Coord) -> Coord {
        Coord::new((coord.r + 1) % self.rows, coord.c)
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
