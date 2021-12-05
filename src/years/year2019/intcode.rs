pub struct Machine {
    pub memory: Vec<i64>,
    ip: usize,
    pub halt: bool,
}

impl Machine {
    pub fn step(&mut self) {
        let opcode = self.memory[self.ip];
        let (in1, in2, out) = (
            self.memory[self.ip + 1] as usize,
            self.memory[self.ip + 2] as usize,
            self.memory[self.ip + 3] as usize,
        );
        self.ip += 4;
        match opcode {
            1 => self.memory[out] = self.memory[in1] + self.memory[in2],
            2 => self.memory[out] = self.memory[in1] * self.memory[in2],
            99 => self.halt = true,
            _ => unimplemented!(),
        }
    }
    pub fn run(&mut self) {
        while !self.halt {
            self.step();
        }
    }
    pub fn input(&mut self, input1: i64, input2: i64) {
        self.memory[1] = input1;
        self.memory[2] = input2;
    }
    pub fn output(&self) -> i64 {
        self.memory[0]
    }
}
impl From<Vec<i64>> for Machine {
    fn from(memory: Vec<i64>) -> Self {
        Self {
            memory,
            ip: 0,
            halt: false,
        }
    }
}
impl From<&Vec<i64>> for Machine {
    fn from(memory: &Vec<i64>) -> Self {
        Self {
            memory: memory.to_owned(),
            ip: 0,
            halt: false,
        }
    }
}
