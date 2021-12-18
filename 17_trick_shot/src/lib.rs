use std::ops::RangeInclusive;

pub fn max_y(target: Target) -> i32 {
    let max_dy = target.bottom().abs() - 1;
    let max_y = gauss_sum(max_dy);
    max_y
}

/*
The range of Dx values to test:
- max: a Dx where the first step shoots beyond the target max x
- min: a Dx where sum(1..Dx) is less than the target min x

The range of Dy values to test:
- max: calculated in part 1: Dy where speed at first upward step greater than target max y is >= the distance between origin and target min y?
- min: a Dy where the first step causes probe to shoot below target min y
*/
pub fn all_starts(target: Target) -> Vec<(i32, i32)> {
    let max_dx = target.right();
    let min_dx = {
        let mut min = max_dx;
        loop {
            let new_min = min - 1;
            if gauss_sum(new_min) < target.left() {
                break min
            } else {
                min = new_min
            }
        }
    };

    let max_dy = target.bottom().abs() - 1;
    let min_dy = target.bottom();

    let launcher = ProbeLauncher::new(target);

    let mut result = Vec::new();
    for dx in min_dx..=max_dx {
        for dy in min_dy..=max_dy {
            let mut probe = launcher.launch((dx, dy));
            if probe.step_to_result() {
                result.push((dx, dy));
            }
        }
    }

    result
}

pub fn gauss_sum(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

struct ProbeLauncher {
    target: Target,
}

impl ProbeLauncher {
    fn new(target: Target) -> ProbeLauncher {
        ProbeLauncher { target }
    }

    fn launch(&self, velocity: (i32, i32)) -> Probe {
        Probe {
            x: 0,
            y: 0,
            dx: velocity.0,
            dy: velocity.1,
            target: self.target.clone()
        }
    }
}

struct Probe {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    target: Target,
}

impl Probe {
    fn step(&mut self) -> Option<bool> {
        if let Err(offby) = self.target.test(self.x, self.y) {
            if offby.overshoot() { return None }
        }

        self.x += self.dx;
        self.y += self.dy;
        
        if self.dx > 0 {
            self.dx -= 1;
        } else if self.dx < 0 {
            self.dx += 1;
        }

        self.dy -= 1;

        match self.target.test(self.x, self.y) {
            Ok(()) => Some(true),
            Err(offby) => {
                if offby.overshoot() {
                    None
                } else {
                    Some(false)
                }
            }
        }
    }

    fn step_to_result(&mut self) -> bool {
        loop {
            match self.step() {
                Some(true) => return true,
                Some(false) => continue,
                None => return false,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl Target {
    pub fn new(x: RangeInclusive<i32>, y: RangeInclusive<i32>) -> Target {
        assert!(*x.start() > 0 && *y.end() < 0, "invalid target location");
        Target {
            x: x,
            y: y,
        }
    }

    pub fn test(&self, x: i32, y: i32) -> Result<(), OffBy> {
        if self.x.contains(&x) && self.y.contains(&y) {
            Ok(())
        } else {
            let offx = match self.x.contains(&x) {
                true => 0,
                false => {
                    if x < self.left() {
                        self.left() - x
                    } else if x > self.right() {
                        self.right() - x
                    } else  {
                        0
                    }
                }
            };
            let offy = match self.y.contains(&y) {
                true => 0,
                false => {
                    if y > self.top() {
                        y - self.top()
                    } else if y < self.bottom() {
                        y - self.bottom()
                    } else  {
                        0
                    }
                }
            };
            Err(OffBy{ x: offx, y: offy })
        }
    }

    pub fn left(&self) -> i32 {
        *self.x.start()
    }

    pub fn right(&self) -> i32 {
        *self.x.end()
    }

    pub fn top(&self) -> i32 {
        *self.y.end()
    }

    pub fn bottom(&self) -> i32 {
        *self.y.start()
    }
}

pub struct OffBy { x: i32, y: i32 }

impl OffBy {
    pub fn overshoot(&self) -> bool {
        self.x < 0 || self.y < 0
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
