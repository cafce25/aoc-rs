use anyhow::{Error, Result};
use std::{
    collections::VecDeque,
    fmt::Debug,
    iter::IntoIterator,
    ops::{Deref, DerefMut, Index, IndexMut},
    str::FromStr,
};

type Atom = i64;

#[derive(Debug, Clone)]
pub struct Intcode(Vec<Atom>);

impl FromStr for Intcode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Intcode(
            s.split(',')
                .map(|n| n.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl From<Vec<Atom>> for Intcode {
    fn from(ic: Vec<Atom>) -> Self {
        Self(ic)
    }
}

impl Deref for Intcode {
    type Target = Vec<Atom>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Intcode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
    pub memory: Intcode,
    ip: usize,
    pub halt: bool,
    input: VecDeque<Atom>,
    output: VecDeque<Atom>,
    relative_base: Atom,
    empty: Atom,
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
                #[cfg(test)]
                {
                    println!(
                        "{:5}: add {:?}({}) {:?}({}) {:?}({})",
                        ip,
                        in1,
                        self[(in1, ip + 1)],
                        in2,
                        self[(in2, ip + 2)],
                        out,
                        self[(out, ip + 3)]
                    );
                }
                self[(out, ip + 3)] = self[(in1, ip + 1)] + self[(in2, ip + 2)];
            }
            Mul(in1, in2, out) => {
                #[cfg(test)]
                {
                    println!(
                        "{:5}: mul {:?}({}) {:?}({}) {:?}({})",
                        ip,
                        in1,
                        self[(in1, ip + 1)],
                        in2,
                        self[(in2, ip + 2)],
                        out,
                        self[(out, ip + 3)]
                    );
                }
                self[(out, ip + 3)] = self[(in1, ip + 1)] * self[(in2, ip + 2)];
            }
            Halt => {
                #[cfg(test)]
                {
                    println!("{:5}: hcf", ip);
                }
                self.halt = true;
            }
            Input(dest) => {
                #[cfg(test)]
                {
                    println!("{:5}: in {:?}({})", ip, dest, self[(dest, ip + 1)]);
                }
                if let Some(val) = self.input.pop_front() {
                    self[(dest, ip + 1)] = val
                }
            }
            Output(source) => {
                #[cfg(test)]
                {
                    println!("{:5}: out {:?}({})", ip, source, self.memory[ip + 1]);
                }
                self.output.push_back(self[(source, ip + 1)])
            }
            JumpIfTrue(b, t) => {
                #[cfg(test)]
                {
                    println!(
                        "{:5}: jt {:?}({}) {:?}({})",
                        ip,
                        b,
                        self[(b, ip + 1)],
                        t,
                        self[(t, ip + 2)]
                    );
                }
                if self[(b, ip + 1)] != 0 {
                    self.ip = self[(t, ip + 2)] as usize;
                }
            }
            JumpIfFalse(b, t) => {
                #[cfg(test)]
                {
                    println!(
                        "{:5}: jf {:?}({}) {:?}({})",
                        ip,
                        b,
                        self[(b, ip + 1)],
                        t,
                        self[(t, ip + 2)]
                    );
                }
                if self[(b, ip + 1)] == 0 {
                    self.ip = self[(t, ip + 2)] as usize;
                }
            }
            LessThan(in1, in2, out) => {
                #[cfg(test)]
                {
                    println!(
                        "{:5}: lt {:?}({}) {:?}({}) {:?}({})",
                        ip,
                        in1,
                        self[(in1, ip + 1)],
                        in2,
                        self[(in2, ip + 2)],
                        out,
                        self[(out, ip + 3)]
                    );
                }
                self[(out, ip + 3)] = if self[(in1, ip + 1)] < self[(in2, ip + 2)] {
                    1
                } else {
                    0
                }
            }
            Equals(in1, in2, out) => {
                #[cfg(test)]
                {
                    println!(
                        "{:5}: eq {:?}({}) {:?}({}) {:?}({})",
                        ip,
                        in1,
                        self[(in1, ip + 1)],
                        in2,
                        self[(in2, ip + 2)],
                        out,
                        self[(out, ip + 3)]
                    );
                }
                self[(out, ip + 3)] = if self[(in1, ip + 1)] == self[(in2, ip + 2)] {
                    1
                } else {
                    0
                }
            }
            AdjustRelativeBase(adj) => {
                #[cfg(test)]
                {
                    println!("{:5}: adj {:?}({})", ip, adj, self.memory[ip + 1]);
                }
                self.relative_base += self[(adj, ip + 1)];
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

    pub fn set_input<V: Into<VecDeque<i64>>>(&mut self, v: V) {
        self.input = v.into();
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

    fn reserve(&mut self, idx: usize) {
        if self.memory.len() <= idx {
            let required = idx - self.memory.len();
            self.memory.reserve(required);
            while self.memory.len() <= idx {
                self.memory.push(0);
            }
        }
    }
    fn idx(&self, mode: PMode, idx: usize) -> usize {
        let idx = match mode {
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
        };

        idx
    }
}

impl Index<(PMode, usize)> for Machine {
    type Output = i64;
    fn index(&self, (mode, idx): (PMode, usize)) -> &i64 {
        let idx = self.idx(mode, idx);
        if idx >= self.memory.len() {
            &self.empty
        } else {
            &self.memory[idx]
        }
    }
}

impl IndexMut<(PMode, usize)> for Machine {
    fn index_mut(&mut self, (mode, idx): (PMode, usize)) -> &mut i64 {
        let idx = self.idx(mode, idx);
        self.reserve(idx);
        &mut self[idx]
    }
}

impl Index<usize> for Machine {
    type Output = i64;
    fn index(&self, idx: usize) -> &i64 {
        &self.memory[idx]
    }
}
impl IndexMut<usize> for Machine {
    fn index_mut(&mut self, idx: usize) -> &mut i64 {
        &mut self.memory[idx]
    }
}

impl From<Intcode> for Machine {
    fn from(memory: Intcode) -> Self {
        Self {
            memory,
            ip: 0,
            halt: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
            relative_base: 0,
            empty: 0,
        }
    }
}

impl From<&Intcode> for Machine {
    fn from(memory: &Intcode) -> Self {
        Self::from(memory.to_owned())
    }
}

impl From<Vec<Atom>> for Machine {
    fn from(memory: Vec<Atom>) -> Self {
        Self {
            memory: memory.into(),
            ip: 0,
            halt: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
            relative_base: 0,
            empty: 0,
        }
    }
}

impl From<&[Atom]> for Machine {
    fn from(memory: &[Atom]) -> Self {
        Self::from(memory.to_owned())
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(machine.next(), Some(1));
        let mut machine = Machine::from(&memory[..]);
        machine.input(-1);
        machine.run();
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

    #[test]
    fn quine_test() {
        let memory = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let machine = Machine::from(&memory[..]);
        let output: Vec<_> = machine.collect();
        assert_eq!(output, memory);
    }

    #[test]
    fn large_product_test() {
        let memory = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let machine = Machine::from(&memory[..]);
        let output: Vec<_> = machine.collect();
        assert_eq!(output, [1219070632396864]);
    }
    #[test]
    fn large_number_test() {
        let memory = [104, 1125899906842624, 99];
        let mut machine = Machine::from(&memory[..]);
        assert_eq!(machine.next(), Some(memory[1]));
    }
}
