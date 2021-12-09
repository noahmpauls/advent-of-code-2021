use std::collections::{HashSet};

pub type Height = u32;
type Coord = (usize, usize);

type Basin = HashSet<Coord>;

pub struct Heightmap {
    map: Vec<Vec<Height>>,
    rows: usize,
    cols: usize,
}

impl Heightmap {
    pub fn new(heights: &Vec<Vec<Height>>) -> Heightmap {
        let map = heights.clone();
        let rows = heights.len();
        let cols = heights[0].len();
        Heightmap { map, rows, cols }
    }

    fn low_points(&self) -> Vec<(Coord, Height)> {
        let mut low_points = Vec::new();

        for (r, row) in self.map.iter().enumerate() {
            for (c, _) in row.iter().enumerate() {
                if let Some(height) = self.test_low_point(r, c) {
                    low_points.push(((r, c), height));
                }
            }
        }

        low_points
    }

    fn low_coords(&self) -> Vec<Coord>{
        self.low_points().iter().map(|(coord, _)| *coord).collect()
    }

    pub fn low_heights(&self) -> Vec<Height> {
        self.low_points().iter().map(|(_, height)| *height).collect()
    }

    pub fn risk_level(&self) -> u32 {
        self.low_heights().iter().map(|v| v + 1).sum()
    }

    fn test_low_point(&self, row: usize, col: usize) -> Option<Height> {
        let height = self.map[row][col];
        for r in Self::neighbors(row, self.rows - 1).iter().copied() {
            if r == row { continue; }
            if self.map[r][col] <= height {
                return None
            }
        }

        for c in Self::neighbors(col, self.cols - 1).iter().copied() {
            if c == col { continue; }
            if self.map[row][c] <= height {
                return None;
            }
        }

        return Some(height);
    }

    fn neighbors(num: usize, max: usize) -> Box<[usize]> {
        match num {
            0 => Box::new([0, 1]),
            v if v == max => Box::new([max - 1, max]),
            v if v < max => Box::new([num-1, num, num + 1]),
            _ => Box::new([]),
        }
    }

    pub fn basins(&self) -> Vec<Basin> {
        self.low_coords().iter().map(|p| self.basin(p.0, p.1)).collect()
    }

    fn basin(&self, row: usize, col: usize) -> Basin {
        let mut basin = Basin::new();
        let mut next = Vec::new();
        next.push((row, col));

        'construct: while next.len() > 0 {
            let (row, col) = next.pop().unwrap();
            let height = self.map[row][col];

            let mut neighbors = Vec::new();

            for r in Self::neighbors(row, self.rows - 1).iter().copied() {
                if r == row { continue; }
                let rheight = self.map[r][col];
                if basin.contains(&(r, col)) {
                    // no action
                } else if rheight >= height {
                    if rheight < 9 {
                        neighbors.push((r, col));
                    }
                } else {
                    continue 'construct;
                }
            }
    
            for c in Self::neighbors(col, self.cols - 1).iter().copied() {
                if c == col { continue; }
                let cheight = self.map[row][c];
                if basin.contains(&(row, c)) {
                    // no action
                } else if cheight >= height {
                    if cheight < 9 {
                        neighbors.push((row, c));
                    }
                } else {
                    continue 'construct;
                }
            }

            basin.insert((row, col));
            next.append(&mut neighbors);
        }

        basin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let heightmap = vec![
            vec![9, 1, 1, 9, 9],
            vec![9, 1, 0, 1, 9],
            vec![9, 1, 9, 1, 9],
            vec![1, 1, 1, 1, 9],
            vec![9, 9, 1, 9, 9],
        ];

        let basin_size = heightmap.iter().flatten().filter(|v| **v == 0 || **v == 1).count();

        let heightmap = Heightmap::new(&heightmap);
        let basins = heightmap.basins();

        assert_eq!(1, basins.len());

        let basin = &basins[0];

        assert_eq!(basin_size, basin.len());
    }
}
