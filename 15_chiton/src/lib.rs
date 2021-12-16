use std::cmp::{Reverse};
use priority_queue::PriorityQueue;


pub type Risk = u8;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Coord {
        Coord{ row, col }
    }

    pub fn adj(&self) -> Box<[Coord]> {
        let row_adj = Self::adj_axis(self.row);
        let row_adj = row_adj.iter().map(|r| Coord::new(*r, self.col));
        let col_adj = Self::adj_axis(self.col);
        let col_adj = col_adj.iter().map(|c| Coord::new(self.row, *c));
        let adj = row_adj.chain(col_adj).collect();
        adj
    }

    pub fn adj_bounded(&self, max_row: usize, max_col: usize) -> Box<[Coord]> {
        let row_adj = Self::adj_axis_bounded(self.row, max_row);
        let row_adj = row_adj.iter().map(|r| Coord::new(*r, self.col));
        let col_adj = Self::adj_axis_bounded(self.col, max_col);
        let col_adj = col_adj.iter().map(|c| Coord::new(self.row, *c));
        let adj = row_adj.chain(col_adj).collect();
        adj
    }

    fn adj_axis(coord: usize) -> Box<[usize]> {
        match coord {
            0 => Box::new([1]),
            c => Box::new([c-1, c+1]),
        }
    }

    fn adj_axis_bounded(coord: usize, bound: usize) -> Box<[usize]> {
        let bound = bound - 1;
        match coord {
            0 => Box::new([1]),
            c if c == bound => Box::new([c-1]),
            c => Box::new([c-1, c+1]),
        }
    }
}

pub struct RiskGrid {
    list: Vec<Risk>,
    rows: usize,
    cols: usize,
}

impl RiskGrid {
    // pub fn from_grid(grid: &[&[Risk]]) -> RiskGrid {
    //     assert!(grid.len() > 0, "grid is empty");

    //     let rows = grid.len();
    //     let cols = grid[0].len();

    //     let list = grid.iter().flat_map(|row| {
    //         assert_eq!(cols, row.len(), "inconsistent row size");
    //         row.iter().copied()
    //     }).collect();

    //     RiskGrid { rows, cols, list }
    // }

    pub fn from_list(list: &[Risk], row_size: usize) -> RiskGrid {
        assert!(list.len() % row_size == 0, "cannot fill all rows");
        assert!(list.len() > 0, "list is empty");

        let rows = list.len() / row_size;
        let cols = row_size;
        let list = list.iter().copied().collect();

        RiskGrid{ rows, cols, list }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Risk> {
        self.get_coord(Coord::new(row, col))
    }

    fn get_coord(&self, c: Coord) -> Option<Risk> {
        if let Some(i) = self.ci(c) {
            Some(self.list[i])
        } else {
            None
        }
    }

    fn ci(&self, c: Coord) -> Option<usize> {
        let i = (self.rows * c.row) + c.col;
        if i < self.list.len() {
            Some(i)
        } else {
            None
        }
    }

    fn ic(&self, index: usize) -> Option<Coord> {
        if index < self.list.len() {
            Some(Coord::new(index / self.rows, index % self.rows))
        } else {
            None
        }
    }

    pub fn safest_path(&self) -> Vec<Coord> {
        let start = self.ci(Coord::new(0, 0)).unwrap();
        let end = self.ci(Coord::new(self.rows - 1, self.cols - 1)).unwrap();

        let mut distances: Vec<Option<u32>> = (0..self.list.len()).map(|_| None).collect();
        let mut previous: Vec<Option<usize>> = (0..self.list.len()).map(|_| None).collect();
        let mut priority = PriorityQueue::new();

        distances[start] = Some(0);
        priority.push(start, Reverse(0));

        'dijkstra: while let Some((current, Reverse(risk))) = priority.pop() {
            if current == end { break 'dijkstra; }

            for neighbor in self.ic(current).unwrap().adj_bounded(self.rows, self.cols).iter().map(|c| self.ci(*c).unwrap()) {
                let new: u32 = risk as u32 + self.list[neighbor] as u32;
                if let Some(old) = distances[neighbor] {
                    if new < old {
                        distances[neighbor] = Some(new);
                        previous[neighbor] = Some(current);
                        if let None = priority.change_priority(&neighbor, Reverse(new)) {
                            priority.push(neighbor, Reverse(new));
                        }
                    }
                } else {
                    distances[neighbor] = Some(new);
                    previous[neighbor] = Some(current);
                    priority.push(neighbor, Reverse(new));
                }
            }
        }

        let mut path = Vec::new();
        let mut trace = Some(end);
        
        while let Some(current) = trace {
            path.push(self.ic(current).unwrap());
            trace = previous[current];
        } 

        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn from_list() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let grid = RiskGrid::from_list(&list[..], 3);

        assert_eq!(5, grid.get(1, 1).unwrap());
    }

    #[test]
    fn coord_neighbors_bounded() {
        let c = Coord::new(7, 5);
        let neighbors = c.adj_bounded(100, 100);
        let expected = vec![
            Coord::new(7, 4),
            Coord::new(7, 6),
            Coord::new(6, 5),
            Coord::new(8, 5),
        ];

        assert_eq!(expected.len(), neighbors.len());

        for e in expected {
            assert!(neighbors.contains(&e), "does not contain {:?}, was {:?}", e, neighbors);
        }
    }

    #[test]
    fn coord_neighbors_bounded_max_row() {
        let c = Coord::new(7, 5);
        let neighbors = c.adj_bounded(8, 100);
        let expected = vec![
            Coord::new(7, 4),
            Coord::new(7, 6),
            Coord::new(6, 5),
        ];

        assert_eq!(expected.len(), neighbors.len());

        for e in expected {
            assert!(neighbors.contains(&e), "does not contain {:?}", e);
        }
    }

    #[test]
    fn coord_neighbors_bounded_max_col() {
        let c = Coord::new(4, 6);
        let neighbors = c.adj_bounded(17, 7);
        let expected = vec![
            Coord::new(3, 6),
            Coord::new(5, 6),
            Coord::new(4, 5),
        ];

        assert_eq!(expected.len(), neighbors.len());

        for e in expected {
            assert!(neighbors.contains(&e), "does not contain {:?}", e);
        }
    }
}
