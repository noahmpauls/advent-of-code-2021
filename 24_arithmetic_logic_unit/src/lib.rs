mod alu;
pub use alu::{Program, ALU};

use alu::{Registers, Reg};
use std::collections::HashSet;

pub fn monad_max(program: &Program) -> Option<String> {
    let blocks = program.into_blocks();
    let state = Registers { w:0, x:0, y:0, z:0 };
    let mut memo = HashSet::new();
    monad_max_block(&blocks, 0, state, &mut memo).map(|result| {
        result.into_iter().rev()
            .map(|digit| digit.to_string())
            .collect()
    })
}

fn monad_max_block(blocks: &[Program], block: usize, state: Registers, memo: &mut Memo) -> Option<Vec<Reg>> {
    for input in (1..10).rev() {
        if memo.contains(&MemoEntry{ block, input, z: state.z }) {
            continue;
        }

        let mut alu = ALU::new(&[input]);
        alu.set_state(&state);
        alu.run(&blocks[block]);
        let output = alu.state();
        
        // blocks left to analyze
        if block + 1 < blocks.len() {
            if let Some(mut digits) = monad_max_block(blocks, block + 1, output, memo) {
                digits.push(input);
                return Some(digits);
            } else {
                memo.insert(MemoEntry{ block, input, z: state.z });
            }
        // end reached
        } else {
            if output.z == 0 {
                return Some(vec![input]);
            } else {
                memo.insert(MemoEntry{ block, input, z: state.z });
            }
        }
    }

    None
}

pub fn monad_min(program: &Program) -> Option<String> {
    let blocks = program.into_blocks();
    let state = Registers { w:0, x:0, y:0, z:0 };
    let mut memo = HashSet::new();
    monad_min_block(&blocks, 0, state, &mut memo).map(|result| {
        result.into_iter().rev()
            .map(|digit| digit.to_string())
            .collect()
    })
}

fn monad_min_block(blocks: &[Program], block: usize, state: Registers, memo: &mut Memo) -> Option<Vec<Reg>> {
    for input in 1..10 {
        if memo.contains(&MemoEntry{ block, input, z: state.z }) {
            continue;
        }

        let mut alu = ALU::new(&[input]);
        alu.set_state(&state);
        alu.run(&blocks[block]);
        let output = alu.state();
        
        // blocks left to analyze
        if block + 1 < blocks.len() {
            if let Some(mut digits) = monad_min_block(blocks, block + 1, output, memo) {
                digits.push(input);
                return Some(digits);
            } else {
                memo.insert(MemoEntry{ block, input, z: state.z });
            }
        // end reached
        } else {
            if output.z == 0 {
                return Some(vec![input]);
            } else {
                memo.insert(MemoEntry{ block, input, z: state.z });
            }
        }
    }

    None
}

// Memo only tracks input and z register per block, as w is always set to input,
// and x/y are always set to 0 before being used in every MONAD block.
#[derive(Debug, PartialEq, Eq, Hash)]
struct MemoEntry {
    block: usize,
    input: Reg,
    z: Reg,
}

type Memo = HashSet<MemoEntry>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monad_max() {
        let result = "7";
        let program = Program::parse(&vec![
            "inp w",
            "mul x 0",
            "add x z",
            "mod x 26",
            "div z 1",
            &format!("add x {}", result),
            "eql x w",
            "eql x 0",
            "mul y 0",
            "add y 25",
            "mul y x",
            "add y 1",
            "mul z y",
            "mul y 0",
            "add y w",
            "add y 8",
            "mul y x",
            "add z y",
        ].join("\n"));

        assert_eq!(result, monad_max(&program).unwrap())
    }
}
