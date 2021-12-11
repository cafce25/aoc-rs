use std::{
    collections::VecDeque,
    fmt::Debug,
    iter::IntoIterator,
    ops::{Index, IndexMut},
};

type Atom = i64;

pub type Intcode = Vec<Atom>;

#[derive(Copy, Clone, Debug)]
enum PMode {
    Immediate,
    Positional,
    Relative,
}

impl From<u8> for PMode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => Self::Positional,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => unimplemented!("invalid parameter mode"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Add(PMode, PMode, PMode),
    Mul(PMode, PMode, PMode),
    Input(PMode),
    Output(PMode),
    JumpIfTrue(PMode, PMode),
    JumpIfFalse(PMode, PMode),
    LessThan(PMode, PMode, PMode),
    Equals(PMode, PMode, PMode),
    AdjustRelativeBase(PMode),
    Halt,
}

impl OpCode {
    fn len(&self) -> usize {
        match self {
            Self::Add(..) | Self::Mul(..) | Self::LessThan(..) | Self::Equals(..) => 4,
            Self::Input(_) | Self::Output(_) | Self::AdjustRelativeBase(_) => 2,
            Self::Halt => 0,
            Self::JumpIfTrue(..) | Self::JumpIfFalse(..) => 3,
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    pub memory: Vec<Atom>,
    ip: usize,
    pub halt: bool,
    input: VecDeque<Atom>,
    output: VecDeque<Atom>,
    relative_base: i64,
}

impl Iterator for Machine {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.halt && self.output.is_empty() {
            self.step();
        }
        self.output.pop_front()
    }
}

impl Machine {
    pub fn step(&mut self) {
        let op_code = self.op_code(self.ip);
        let ip = self.ip;
        self.ip += op_code.len();
        use OpCode::*;
        match op_code {
            Add(in1, in2, out) => {
                self[(out, ip + 3)] = self[(in1, ip + 1)] + self[(in2, ip + 2)];
            }
            Mul(in1, in2, out) => {
                self[(out, ip + 3)] = self[(in1, ip + 1)] * self[(in2, ip + 2)];
            }
            Halt => {
                self.halt = true;
            }
            Input(out) => {
                if let Some(val) = self.input.pop_front() {
                    self[(out, ip + 1)] = val
                }
            }
            Output(input) => self.output.push_back(self[(input, ip + 1)]),
            JumpIfTrue(b, t) => {
                if self[(b, ip + 1)] != 0 {
                    self.ip = self[(t, ip + 2)] as usize;
                }
            }
            JumpIfFalse(b, t) => {
                if self[(b, ip + 1)] == 0 {
                    self.ip = self[(t, ip + 2)] as usize;
                }
            }
            LessThan(in1, in2, out) => {
                self[(out, ip + 3)] = if self[(in1, ip + 1)] < self[(in2, ip + 2)] {
                    1
                } else {
                    0
                }
            }
            Equals(in1, in2, out) => {
                self[(out, ip + 3)] = if self[(in1, ip + 1)] == self[(in2, ip + 2)] {
                    1
                } else {
                    0
                }
            }
            AdjustRelativeBase(input) => {
                self.relative_base = self[(input, ip + 1)];
            }
        };
    }

    pub fn run(&mut self) {
        while !self.halt {
            self.step();
        }
    }

    pub fn output(&mut self) -> Vec<Atom> {
        self.output.drain(..).collect()
    }

    pub fn input(&mut self, input: Atom) {
        self.input.push_back(input)
    }

    pub fn input_iter<I>(&mut self, input: I)
    where
        I: IntoIterator<Item = Atom>,
    {
        let it = input.into_iter();
        let (lower, _) = it.size_hint();
        self.input.reserve(lower);
        it.for_each(|v| self.input.push_back(v));
    }

    fn op_code(&self, ip: usize) -> OpCode {
        if ip >= self.memory.len() {
            return OpCode::Halt;
        }
        let op_code = self.memory[ip] % 100;
        let param_mode1 = (self.memory[ip] / 100 % 10) as u8;
        let param_mode2 = (self.memory[ip] / 1000 % 10) as u8;
        let param_mode3 = (self.memory[ip] / 10000 % 10) as u8;
        match op_code {
            1 => OpCode::Add(
                PMode::from(param_mode1),
                PMode::from(param_mode2),
                PMode::from(param_mode3),
            ),
            2 => OpCode::Mul(
                PMode::from(param_mode1),
                PMode::from(param_mode2),
                PMode::from(param_mode3),
            ),
            3 => OpCode::Input(PMode::from(param_mode1)),
            4 => OpCode::Output(PMode::from(param_mode1)),
            5 => OpCode::JumpIfTrue(PMode::from(param_mode1), PMode::from(param_mode2)),
            6 => OpCode::JumpIfFalse(PMode::from(param_mode1), PMode::from(param_mode2)),
            7 => OpCode::LessThan(
                PMode::from(param_mode1),
                PMode::from(param_mode2),
                PMode::from(param_mode3),
            ),
            8 => OpCode::Equals(
                PMode::from(param_mode1),
                PMode::from(param_mode2),
                PMode::from(param_mode3),
            ),
            9 => OpCode::AdjustRelativeBase(PMode::from(param_mode1)),
            99 => OpCode::Halt,
            _ => unimplemented!(),
        }
    }

    fn idx(&self, mode: PMode, idx: usize) -> usize {
        match mode {
            PMode::Immediate => idx,
            PMode::Positional => {
                let val = self.memory[idx];
                if val < 0 {
                    panic!("negative number for positional index")
                }
                val as usize
            }
            PMode::Relative => {
                let val = self.memory[idx] + self.relative_base;
                if val < 0 {
                    panic!("negative number for relative index")
                }
                val as usize
            }
        }
    }
}

impl Index<(PMode, usize)> for Machine {
    type Output = i64;
    fn index(&self, (mode, idx): (PMode, usize)) -> &i64 {
        &self.memory[self.idx(mode, idx)]
    }
}

impl IndexMut<(PMode, usize)> for Machine {
    fn index_mut(&mut self, (mode, idx): (PMode, usize)) -> &mut i64 {
        let idx = self.idx(mode, idx);
        &mut self.memory[idx]
    }
}

impl From<Vec<Atom>> for Machine {
    fn from(memory: Vec<Atom>) -> Self {
        Self {
            memory,
            ip: 0,
            halt: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
            relative_base: 0,
        }
    }
}

impl From<&[Atom]> for Machine {
    fn from(memory: &[Atom]) -> Self {
        Self::from(memory.to_owned())
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn input_output_test() {
        let memory = [3, 0, 4, 0, 99];
        let mut machine = Machine::from(&memory[..]);
        machine.input(1);
        assert_eq!(machine.next(), Some(1));
        // assert_eq!(machine.memory[..], vec![1, 0, 4, 0, 99]);
    }

    #[test]
    fn parameter_mode_test() {
        let memory = [1002, 4, 3, 4, 33];
        let mut machine = Machine::from(&memory[..]);
        machine.run();
        assert_eq!(machine.memory[..], vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn equals_position_mode_test() {
        let memory = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut machine = Machine::from(&memory[..]);
        machine.input(7);
        assert_eq!(machine.next(), Some(0));
        let mut machine = Machine::from(&memory[..]);
        machine.input(8);
        assert_eq!(machine.next(), Some(1));
        let mut machine = Machine::from(&memory[..]);
        machine.input(9);
        assert_eq!(machine.next(), Some(0));
    }

    #[test]
    fn less_than_position_mode_test() {
        let memory = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut machine = Machine::from(&memory[..]);
        machine.input(7);
        assert_eq!(machine.next(), Some(1));
        let mut machine = Machine::from(&memory[..]);
        machine.input(8);
        assert_eq!(machine.next(), Some(0));
        let mut machine = Machine::from(&memory[..]);
        machine.input(9);
        assert_eq!(machine.next(), Some(0));
    }

    #[test]
    fn equals_immediate_mode_test() {
        let memory = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut machine = Machine::from(&memory[..]);
        machine.input(7);
        assert_eq!(machine.next(), Some(0));
        let mut machine = Machine::from(&memory[..]);
        machine.input(8);
        assert_eq!(machine.next(), Some(1));
        let mut machine = Machine::from(&memory[..]);
        machine.input(9);
        assert_eq!(machine.next(), Some(0));
    }

    #[test]
    fn less_than_immediate_mode_test() {
        let memory = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut machine = Machine::from(&memory[..]);
        machine.input(7);
        assert_eq!(machine.next(), Some(1));
        let mut machine = Machine::from(&memory[..]);
        machine.input(8);
        assert_eq!(machine.next(), Some(0));
        let mut machine = Machine::from(&memory[..]);
        machine.input(9);
        assert_eq!(machine.next(), Some(0));
    }

    #[test]
    fn jump_position_mode_test() {
        let memory = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut machine = Machine::from(&memory[..]);
        machine.input(0);
        assert_eq!(machine.next(), Some(0));
        let mut machine = Machine::from(&memory[..]);
        machine.input(1);
        machine.run();
        dbg!(&machine.memory[..]);
        assert_eq!(machine.next(), Some(1));
        let mut machine = Machine::from(&memory[..]);
        machine.input(-1);
        machine.run();
        dbg!(&machine.memory[..]);
        assert_eq!(machine.next(), Some(1));
    }

    #[test]
    fn jump_immediate_mode_test() {
        let memory = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut machine = Machine::from(&memory[..]);
        machine.input(0);
        assert_eq!(machine.next(), Some(0));
        let mut machine = Machine::from(&memory[..]);
        machine.input(1);
        assert_eq!(machine.next(), Some(1));
        let mut machine = Machine::from(&memory[..]);
        machine.input(-1);
        assert_eq!(machine.next(), Some(1));
    }

    #[test]
    fn larger_example_test() {
        let memory = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut machine = Machine::from(&memory[..]);
        machine.input(5);
        assert_eq!(machine.next(), Some(999));
        let mut machine = Machine::from(&memory[..]);
        machine.input(8);
        assert_eq!(machine.next(), Some(1000));
        let mut machine = Machine::from(&memory[..]);
        machine.input(10);
        assert_eq!(machine.next(), Some(1001));
    }
}
