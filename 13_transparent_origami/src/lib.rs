use std::collections::{HashSet};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Dot {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

pub struct Fold {
    pub axis: Axis,
    pub line: i32,
}

pub struct DottedPaper {
    dots: HashSet<Dot>,
}

impl DottedPaper {
    pub fn new(dots: &Vec<Dot>) -> DottedPaper {
        let dots = dots.iter().copied().collect();
        DottedPaper { dots }
    }

    pub fn fold(&mut self, fold: &Fold) {
        self.dots = self.dots.iter().map(|d| Self::fold_dot(d, fold)).collect();
    }

    pub fn dot_count(&self) -> usize {
        self.dots.len()
    }

    fn fold_dot(dot: &Dot, fold: &Fold) -> Dot {
        let x = if fold.axis == Axis::X && dot.x > fold.line {
            (2 * fold.line) - dot.x
        } else { 
            dot.x 
        };

        let y = if fold.axis == Axis::Y && dot.y > fold.line {
            (2 * fold.line) - dot.y
        } else { 
            dot.y 
        };

        Dot { x, y }
    }

    fn max(&self) -> Dot {
        let mut dot_iter = self.dots.iter();
        let mut max = *dot_iter.next().unwrap();
        for dot in dot_iter {
            max.x = std::cmp::max(dot.x, max.x);
            max.y = std::cmp::max(dot.y, max.y);
        }
        max
    }
}

impl fmt::Display for DottedPaper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let dimensions = self.max();
        for r in 0..=dimensions.y {
            for c in 0..=dimensions.x {
                if self.dots.contains(&Dot{ x: c, y: r }) {
                    result.push('█');
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }


        write!(f, "{}", result.trim())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_fold() {
        let mut paper = DottedPaper::new(
            &vec![
                Dot { x: 1, y: 0 },
                Dot { x: 9, y: 0 },
            ]
        );

        let fold = Fold { axis: Axis::X, line: 5 };
        paper.fold(&fold);

        assert_eq!(1, paper.dot_count());
    }

    #[test]
    fn y_fold() {
        let mut paper = DottedPaper::new(
            &vec![
                Dot { x: 4, y: 5 },
                Dot { x: 4, y: 3 },
                Dot { x: 7, y: 0 },
                Dot { x: 7, y: 8 },
            ]
        );

        let fold = Fold { axis: Axis::Y, line: 4 };
        paper.fold(&fold);

        assert_eq!(2, paper.dot_count());
    }

    #[test]
    fn two_folds() {
        let mut paper = DottedPaper::new(
            &vec![
                Dot { x: 102, y: 1 },
                Dot { x: 54, y: 5 }
            ]
        );

        let folds = vec! [
            Fold { axis: Axis::Y, line: 3 },
            Fold { axis: Axis::X, line: 78 },
        ];

        for fold in folds {
            paper.fold(&fold);
        }

        assert_eq!(1, paper.dot_count());
    }
}
