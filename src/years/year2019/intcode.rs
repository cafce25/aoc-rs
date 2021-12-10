use std::{
    collections::VecDeque,
    fmt::Debug,
    iter::IntoIterator,
    ops::{Index, IndexMut},
};

type Atom = i64;

#[derive(Clone, Debug)]
pub struct Memory(Vec<Atom>);

impl Memory {
    fn get(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Immediate(i) => i,
            Parameter::Position(p) => self.0[p],
        }
    }

    fn get_mut(&mut self, param: Parameter) -> &mut i64 {
        match param {
            Parameter::Position(p) => &mut self.0[p],
            _ => unimplemented!(),
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Memory {
    type Output = Atom;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Copy, Clone, Debug)]
enum Parameter {
    Immediate(Atom),
    Position(usize),
}

impl Parameter {
    fn new(param_mode: u8, ip: Atom) -> Parameter {
        match param_mode {
            0 if ip >= 0 => Self::Position(ip as usize),
            0 => panic!("ip smaller than 0"),
            1 => Self::Immediate(ip),
            _ => unimplemented!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt,
}

impl OpCode {
    fn len(&self) -> usize {
        match self {
            Self::Add(..) | Self::Mul(..) | Self::LessThan(..) | Self::Equals(..) => 4,
            Self::Input(..) | Self::Output(..) => 2,
            Self::Halt => 0,
            Self::JumpIfTrue(..) | Self::JumpIfFalse(..) => 3,
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    pub memory: Memory,
    ip: usize,
    pub halt: bool,
    input: VecDeque<Atom>,
    output: VecDeque<Atom>,
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
        self.ip += op_code.len();
        use OpCode::*;
        match op_code {
            Add(in1, in2, out) => {
                *self.memory.get_mut(out) = self.memory.get(in1) + self.memory.get(in2);
            }
            Mul(in1, in2, out) => {
                *self.memory.get_mut(out) = self.memory.get(in1) * self.memory.get(in2);
            }
            Halt => {
                self.halt = true;
            }
            Input(out) => {
                if let Some(val) = self.input.pop_front() {
                    *self.memory.get_mut(out) = val
                }
            }
            Output(input) => self.output.push_back(self.memory.get(input)),
            JumpIfTrue(b, t) => {
                if self.memory.get(b) != 0 {
                    self.ip = self.memory.get(t) as usize;
                }
            }
            JumpIfFalse(b, t) => {
                if self.memory.get(b) == 0 {
                    self.ip = self.memory.get(t) as usize;
                }
            }
            LessThan(in1, in2, out) => {
                *self.memory.get_mut(out) = if self.memory.get(in1) < self.memory.get(in2) {
                    1
                } else {
                    0
                }
            }
            Equals(in1, in2, out) => {
                *self.memory.get_mut(out) = if self.memory.get(in1) == self.memory.get(in2) {
                    1
                } else {
                    0
                }
            }
        };
    }
    pub fn run(&mut self) {
        while !self.halt {
            self.step();
        }
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
                Parameter::new(param_mode1, self.memory[ip + 1]),
                Parameter::new(param_mode2, self.memory[ip + 2]),
                Parameter::new(param_mode3, self.memory[ip + 3]),
            ),
            2 => OpCode::Mul(
                Parameter::new(param_mode1, self.memory[ip + 1]),
                Parameter::new(param_mode2, self.memory[ip + 2]),
                Parameter::new(param_mode3, self.memory[ip + 3]),
            ),
            3 => OpCode::Input(Parameter::new(param_mode1, self.memory[ip + 1])),
            4 => OpCode::Output(Parameter::new(param_mode1, self.memory[ip + 1])),
            5 => OpCode::JumpIfTrue(
                Parameter::new(param_mode1, self.memory[ip + 1]),
                Parameter::new(param_mode2, self.memory[ip + 2]),
            ),
            6 => OpCode::JumpIfFalse(
                Parameter::new(param_mode1, self.memory[ip + 1]),
                Parameter::new(param_mode2, self.memory[ip + 2]),
            ),
            7 => OpCode::LessThan(
                Parameter::new(param_mode1, self.memory[ip + 1]),
                Parameter::new(param_mode2, self.memory[ip + 2]),
                Parameter::new(param_mode3, self.memory[ip + 3]),
            ),
            8 => OpCode::Equals(
                Parameter::new(param_mode1, self.memory[ip + 1]),
                Parameter::new(param_mode2, self.memory[ip + 2]),
                Parameter::new(param_mode3, self.memory[ip + 3]),
            ),
            99 => OpCode::Halt,
            _ => unimplemented!(),
        }
    }
}

impl From<Vec<Atom>> for Machine {
    fn from(memory: Vec<Atom>) -> Self {
        Self {
            memory: Memory(memory),
            ip: 0,
            halt: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
}

impl From<&Vec<Atom>> for Machine {
    fn from(memory: &Vec<Atom>) -> Self {
        Self::from(memory.to_owned())
    }
}
