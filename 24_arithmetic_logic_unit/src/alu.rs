use std::vec::IntoIter;

#[derive(Debug, PartialEq, Eq, Hash)]
struct MemoEntry {
    block: usize,
    input: Reg,
    z: Reg,
}

#[derive(Debug, Clone)]
pub struct Program {
    instructions: Vec<Op>
}

impl Program {
    pub fn parse(code: &str) -> Self {
        Self {
            instructions: code.lines().map(|op| Op::from_str(op)).collect(),
        }
    }

    pub fn new(instructions: &[Op]) -> Self {
        Self {
            instructions: instructions.iter().copied().collect()
        }
    }

    pub fn instructions(&'_ self) -> impl Iterator<Item=&Op> + '_{
        self.instructions.iter()
    }

    pub fn into_blocks(&self) -> Vec<Self> {
        let mut result = Vec::new();

        let mut block = Vec::new();
        for (i, op) in self.instructions.iter().enumerate() {
            if i > 0 {
                if let Op::Inp(_) = op {
                    result.push(block.clone());
                    block = Vec::new();
                    block.push(op.clone());
                } else {
                    block.push(op.clone());
                }
            } else {
                block.push(op.clone());
            }
        }
        result.push(block.clone());

        result.into_iter().map(|v| Self::new(&v)).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Op {
    Inp(Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Div(Arg, Arg),
    Mod(Arg, Arg),
    Eql(Arg, Arg),
}

pub type Reg = i64;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Arg {
    Var(char),
    Num(Reg),
}

impl Arg {
    pub fn arg(s: &str) -> Self {
        if let Ok(num) = s.parse::<Reg>() {
            Arg::Num(num)
        } else {
            Arg::Var(s.chars().next().unwrap())
        }
    }

    pub fn var(s: &str) -> Self {
        assert!(s.parse::<Reg>().is_err());
        Arg::Var(s.chars().next().unwrap())
    }
}

impl Op {
    pub fn from_str(s: &str) -> Self {
        let s: Vec<_> = s.split_whitespace().collect();
        match s[0] {
            "inp" => Op::Inp(Arg::var(s[1])),
            "add" => Op::Add(Arg::var(s[1]), Arg::arg(s[2])),
            "mul" => Op::Mul(Arg::var(s[1]), Arg::arg(s[2])),
            "div" => Op::Div(Arg::var(s[1]), Arg::arg(s[2])),
            "mod" => Op::Mod(Arg::var(s[1]), Arg::arg(s[2])),
            "eql" => Op::Eql(Arg::var(s[1]), Arg::arg(s[2])),
            _ => panic!("invalid operation")
        }
    }
}

pub struct ALU {
    input: IntoIter<Reg>,
    w: Reg,
    x: Reg,
    y: Reg,
    z: Reg,
}

impl ALU {
    pub fn new(input: &[Reg]) -> Self {
        Self {
            input: input.iter().copied().collect::<Vec<_>>().into_iter(),
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn run(&mut self, program: &Program) {
        for op in program.instructions() {
            self.op(op);
        }
    }

    pub fn set_state(&mut self, state: &Registers) {
        self.w = state.w;
        self.x = state.x;
        self.y = state.y;
        self.z = state.z;
    }

    fn op(&mut self, op: &Op) {
        match op {
            Op::Inp(var) => *self.var(var) = self.next_input(),
            Op::Add(var, arg) => *self.var(var) = self.val(var) + self.val(arg),
            Op::Mul(var, arg) => *self.var(var) = self.val(var) * self.val(arg),
            Op::Div(var, arg) => *self.var(var) = self.val(var) / self.val(arg),
            Op::Mod(var, arg) => *self.var(var) = self.val(var) % self.val(arg),
            Op::Eql(var, arg) => *self.var(var) =  if self.val(var) == self.val(arg) { 1 } else { 0 },
        }
    }

    fn next_input(&mut self) -> Reg {
        // this should probably utilize a Result
        self.input.next().unwrap_or_else(|| panic!("not enough input"))
    }

    fn val(&self, arg: &Arg) -> Reg {
        match arg {
            Arg::Num(num) => *num,
            Arg::Var(var) => match var {
                'w' => self.w,
                'x' => self.x,
                'y' => self.y,
                'z' => self.z,
                _ => panic!("bad variable '{}'", var)
            },
        }
    }

    fn var(&mut self, arg: &Arg) -> &mut Reg {
        match arg {
            Arg::Num(num) => panic!("'{}' is not variable", num),
            Arg::Var(var) => match var {
                'w' => &mut self.w,
                'x' => &mut self.x,
                'y' => &mut self.y,
                'z' => &mut self.z,
                _ => panic!("bad variable '{}'", var)
            },
        }
    }

    pub fn w(&self) -> Reg {
        self.w
    }

    pub fn x(&self) -> Reg {
        self.x
    }

    pub fn y(&self) -> Reg {
        self.y
    }

    pub fn z(&self) -> Reg {
        self.z
    }

    pub fn state(&self) -> Registers {
        Registers {
            w: self.w,
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Registers {
    pub w: Reg,
    pub x: Reg,
    pub y: Reg,
    pub z: Reg,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_parse() {
        let op = Op::from_str("add w 1");
        assert_eq!(Op::Add(Arg::Var('w'), Arg::Num(1)), op);
    }

    #[test]
    fn test_alu_basic() {
        // z = (z*3 == x)
        let program = Program::new(&vec![
            "inp z",
            "inp x",
            "mul z 3",
            "eql z x",
        ].into_iter().map(|op| Op::from_str(op)).collect::<Vec<_>>());

        for test in [(vec![3, 9], 1), (vec![2, 7], 0)] {
            let mut alu = ALU::new(&test.0);
            alu.run(&program);
            assert_eq!(test.1, alu.z())
        }
    }

    #[test]
    fn test_alu_binary() {
        // input num -> binary
        let program = Program::new(&vec![
            "inp w",
            "add z w",
            "mod z 2",
            "div w 2",
            "add y w",
            "mod y 2",
            "div w 2",
            "add x w",
            "mod x 2",
            "div w 2",
            "mod w 2",
        ].into_iter().map(|op| Op::from_str(op)).collect::<Vec<_>>());

        for test in [
            (vec![0], (0, 0, 0, 0)),
            (vec![1], (0, 0, 0, 1)),
            (vec![2], (0, 0, 1, 0)),
            (vec![3], (0, 0, 1, 1)),
            (vec![4], (0, 1, 0, 0)),
            (vec![5], (0, 1, 0, 1)),
            (vec![6], (0, 1, 1, 0)),
            (vec![7], (0, 1, 1, 1)),
            (vec![8], (1, 0, 0, 0)),
            (vec![9], (1, 0, 0, 1)),

        ] {
            let mut alu = ALU::new(&test.0);
            alu.run(&program);
            assert_eq!(test.1.0, alu.w());
            assert_eq!(test.1.1, alu.x());
            assert_eq!(test.1.2, alu.y());
            assert_eq!(test.1.3, alu.z());
        }
    }
}
