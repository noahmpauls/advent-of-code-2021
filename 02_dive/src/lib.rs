#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, z: i32) -> Position {
        Position { x, z }
    }

    pub fn from(command: Command) -> Position {
        match command {
            Command::Forward(m) => Position { x: m, z: 0 },
            Command::Up(m) => Position { x: 0, z: -m },
            Command::Down(m) => Position { x: 0, z: m},
        }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            z: self.z + other.z,
        }
    }
}

pub enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Command {
    pub fn from(direction: &str, magnitude: i32) -> Command {
        match direction {
            "forward" => Command::Forward(magnitude),
            "up" => Command::Up(magnitude),
            "down" => Command::Down(magnitude),
            _ => panic!("cannot parse direction {} to command", direction)
        }
    }
}

pub struct Submarine {
    pub position: Position,
    aim: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            position: Position::new(0, 0),
            aim: 0,
        }
    }
    
    pub fn command(&mut self, command: Command) {
        match command {
            Command::Forward(m) => {
                let delta = Position {
                    x: m,
                    z: self.aim*m
                };
                self.position = self.position + delta;
            },
            Command::Up(m) => self.aim -= m,
            Command::Down(m) => self.aim += m,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let start = Position::new(0, 0);
        let add = Position::new(3, 4);
        let result = start + add;
        assert_eq!(Position::new(3, 4), result);
    }
}
