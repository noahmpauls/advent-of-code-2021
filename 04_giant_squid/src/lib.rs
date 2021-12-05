use std::collections::HashMap;

const BOARD_SIDE: usize = 5;

#[derive(Debug)]
struct Coord {
    r: usize,
    c: usize,
}

impl Coord {
    fn new (r: usize, c: usize) -> Coord {
        Coord { r, c }
    }

    fn from_index(index: usize, row_size: usize) -> Coord {
        let r = index / row_size;
        let c = index % row_size;

        Coord{ r, c }
    }

    fn to_index(&self, row_size: usize) -> usize {
        (self.r * row_size) + self.c
    }
}

#[derive(Debug)]
pub struct BingoBoard {
    coords: HashMap<u32, Coord>,
    marks: [bool; BOARD_SIDE.pow(2)],
    bingo: bool,
}

impl BingoBoard {
    pub fn new(board: &Vec<u32>) -> BingoBoard {
        assert_eq!(BOARD_SIDE.pow(2), board.len());

        let coords = board.iter().enumerate().map(|(i, v)| (*v, Coord::from_index(i, 5))).collect();
        let marks = [false; BOARD_SIDE.pow(2)];

        BingoBoard { coords, marks, bingo: false, }
    }

    pub fn mark(&mut self, num: u32) -> MarkResult {
        if let Some(coord) = self.coords.get(&num) {
            self.marks[coord.to_index(BOARD_SIDE)] = true;
            if self.check_win(coord) {
                self.bingo = true;
                MarkResult::Bingo
            } else {
                MarkResult::Continue
            }
        } else {
            MarkResult::Continue
        }
    }

    pub fn unmarked(&self) -> Vec<u32> {
        self.coords.iter()
            .filter_map(|(num, coord)| if !self.marks[coord.to_index(BOARD_SIDE)] { Some(*num) } else { None })
            .collect()
    }

    pub fn bingo(&self) -> bool {
        self.bingo
    }

    fn check_win(&self, start: &Coord) -> bool {
        let row = start.r;
        let col = start.c;

        // check row bingo
        let row_bingo = (0..BOARD_SIDE)
            .map(|r| {
                let index = Coord::new(r, col).to_index(BOARD_SIDE);
                self.marks[index]
            })
            .reduce(|a, b| a && b).unwrap();
        if row_bingo { return true; }

        // check col bingo
        let col_bingo = (0..BOARD_SIDE)
            .map(|c| {
                let index = Coord::new(row, c).to_index(BOARD_SIDE);
                self.marks[index]
            })
            .reduce(|a, b| a && b).unwrap();
        if col_bingo { return true; }

        // // check NW SE diagonal bingo
        // if row == col {
        //     let diag_bingo = (0..BOARD_SIDE)
        //         .map(|d| {
        //             let index = Coord::new(d, d).to_index(BOARD_SIDE);
        //             self.marks[index]
        //         })
        //         .reduce(|a, b| a && b).unwrap();
        //     if diag_bingo { return true; }
        // }
        
        // // check SW NE diagonal bingo
        // if row + col == BOARD_SIDE - 1 {
        //     let diag_bingo = (0..BOARD_SIDE)
        //         .map(|d| {
        //             let index = Coord::new(BOARD_SIDE - (d + 1), d).to_index(BOARD_SIDE);
        //             self.marks[index]
        //         })
        //         .reduce(|a, b| a && b).unwrap();
        //     if diag_bingo { return true; }
        // }

        return false;
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MarkResult {
    Bingo,
    Continue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bingo_board_row_bingo() {
        let board = vec![
            1,  2,  3,  4,  5,
            6,  7,  8,  9,  10,
            11, 12, 13, 14, 15,
            16, 17, 18, 19, 20,
            21, 22, 23, 24, 25,
        ];

        let mut board = BingoBoard::new(&board);
        for num in [11, 12, 13, 14] {
            assert_eq!(MarkResult::Continue, board.mark(num));
        }

        assert_eq!(MarkResult::Bingo, board.mark(15));
    }

    #[test]
    fn bingo_board_col_bingo() {
        let board = vec![
            1,  2,  3,  4,  5,
            6,  7,  8,  9,  10,
            11, 12, 13, 14, 15,
            16, 17, 18, 19, 20,
            21, 22, 23, 24, 25,
        ];

        let mut board = BingoBoard::new(&board);
        for num in [5, 15, 20, 25] {
            assert_eq!(MarkResult::Continue, board.mark(num));
        }

        assert_eq!(MarkResult::Bingo, board.mark(10));
    }

    #[test]
    fn bingo_board_diag_no_bingo() {
        let board = vec![
            1,  2,  3,  4,  5,
            6,  7,  8,  9,  10,
            11, 12, 13, 14, 15,
            16, 17, 18, 19, 20,
            21, 22, 23, 24, 25,
        ];

        let mut board = BingoBoard::new(&board);
        for num in [21, 17, 13, 5] {
            assert_eq!(MarkResult::Continue, board.mark(num));
        }

        assert_eq!(MarkResult::Continue, board.mark(9));
    }

    #[test]
    fn bingo_board_no_bingo() {
        let board = vec![
            14, 21, 17, 24,  4,
            10, 16, 15,  9, 19,
            18,  8, 23, 26, 20,
            22, 11, 13,  6,  5,
             2,  0, 12,  3,  7,
        ];

        let mut board = BingoBoard::new(&board);
        for num in [7, 4, 9, 5, 11, 17, 23] {
            assert_eq!(MarkResult::Continue, board.mark(num));
        }

        assert_eq!(MarkResult::Continue, board.mark(2));
    }
}
