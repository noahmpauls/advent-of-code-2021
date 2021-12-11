pub type Octopus = u32;
type Coord = (usize, usize);

pub struct OctopusGrid {
    grid: Vec<Vec<Octopus>>,
    rows: usize,
    cols: usize,
    flashes: u32,
}

impl OctopusGrid {
    pub fn new(octopi: &Vec<Vec<Octopus>>) -> OctopusGrid {
        let grid = octopi.clone();
        let rows = octopi.len();
        let cols = octopi[0].len();
        OctopusGrid { grid, rows, cols, flashes: 0 }
    }

    // returns whether octopi are synchronized
    pub fn step(&mut self) -> bool {
        let mut flashes: Vec<Vec<bool>> = self.grid.iter().map(|row| row.iter().map(|_| false).collect()).collect();

        for r in 0..self.grid.len() {
            for c in 0..self.grid[r].len() {
                self.energize((r, c), &mut flashes);
            }
        }

        flashes.iter().map(|row| row.iter().copied().reduce(|a, b| a && b).unwrap()).reduce(|a, b| a && b).unwrap()
    }
    
    fn neighbors(num: usize, max: usize) -> Box<[usize]> {
        match num {
            0 => Box::new([0, 1]),
            v if v == max => Box::new([max - 1, max]),
            v if v < max => Box::new([num-1, num, num + 1]),
            _ => Box::new([]),
        }
    }

    fn energize(&mut self, cd: Coord, flashes: &mut Vec<Vec<bool>>) {
        if !flashes[cd.0][cd.1] {
            self.grid[cd.0][cd.1] += 1;
            if self.grid[cd.0][cd.1] > 9 {
                self.grid[cd.0][cd.1] = 0;
                flashes[cd.0][cd.1] = true;
                self.flashes += 1;
                for r in Self::neighbors(cd.0, self.rows - 1).iter().copied() {
                    for c in Self::neighbors(cd.1, self.cols -1).iter().copied() {
                        if (r, c) != cd { self.energize((r, c), flashes) }
                    }
                }
            }
        }
    }

    pub fn flashes(&self) -> u32 {
        self.flashes
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
