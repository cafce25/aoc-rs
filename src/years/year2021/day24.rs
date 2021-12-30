use std::cmp::Ordering;

use itertools::iproduct;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Alu::from_str(input))
    }
}

use Instruction::*;
#[derive(Copy, Clone, Debug)]
enum Instruction {
    Inp(Reg),
    Add(Reg, Result<Reg, i64>),
    Mul(Reg, Result<Reg, i64>),
    Div(Reg, Result<Reg, i64>),
    Mod(Reg, Result<Reg, i64>),
    Eql(Reg, Result<Reg, i64>),
}

#[derive(Copy, Clone, Debug)]
enum Reg {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}
impl std::str::FromStr for Reg {
    type Err = i64;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Reg::*;
        Ok(match s {
            "w" | "W" => W,
            "x" | "X" => X,
            "y" | "Y" => Y,
            "z" | "Z" => Z,
            _ => return Err(s.parse().unwrap()),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Memory([i64; 4]);

impl Memory {
    fn new() -> Self {
        Memory([0; 4])
    }
}

impl std::ops::Index<Reg> for Memory {
    fn index(&self, index: Reg) -> &i64 {
        &self.0[index as usize]
    }

    type Output = i64;
}

impl std::ops::IndexMut<Reg> for Memory {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("w", &self[Reg::W])
            .field("x", &self[Reg::X])
            .field("y", &self[Reg::Y])
            .field("z", &self[Reg::Z])
            .finish()
    }
}

struct Alu {
    program: Vec<Instruction>,
}

impl Alu {
    pub fn from_str(input: &str) -> Self {
        let program = input
            .lines()
            .map(|l| {
                let (instruction, parameters) = l.split_once(' ').unwrap();
                let instruction = match instruction {
                    "inp" => return Inp(parameters.parse().unwrap()),
                    "add" => Add,
                    "mul" => Mul,
                    "div" => Div,
                    "mod" => Mod,
                    "eql" => Eql,
                    _ => panic!("invalid instruction {:?}", instruction),
                };
                let (param_a, param_b) = parameters.split_once(' ').unwrap();
                instruction(param_a.parse().unwrap(), param_b.parse())
            })
            .collect();
        Self { program }
    }

    fn run(&self, input: &[i64]) -> i64 {
        let mut input = input.iter();
        let mut memory = Memory::new();

        for instruction in &self.program {
            match instruction {
                Inp(a) => {
                    memory[*a] = *input.next().unwrap();
                }
                Add(a, b) => memory[*a] += b.map_or_else(|i| i, |r| memory[r]),
                Mul(a, b) => memory[*a] *= b.map_or_else(|i| i, |r| memory[r]),
                Div(a, b) => memory[*a] /= b.map_or_else(|i| i, |r| memory[r]),
                Mod(a, b) => memory[*a] %= b.map_or_else(|i| i, |r| memory[r]),
                Eql(a, b) => {
                    memory[*a] = (memory[*a] == b.map_or_else(|i| i, |r| memory[r])) as i64
                }
            }
        }

        memory[Reg::Z]
    }
    fn pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        let mut openers = Vec::new();
        let mut i = 0;
        for ins in self.program.iter().copied().filter(|ins| {
            if let Inp(_) = ins {
                return true;
            }
            if let Div(_, Err(26)) = ins {
                return true;
            }
            false
        }) {
            if let Div(..) = ins {
                let b = openers.pop().unwrap();
                pairs.push((openers.pop().unwrap(), b))
            } else {
                openers.push(i);
                i += 1;
            }
        }
        pairs.sort_by_key(|(a, b)| b - a);
        pairs
    }

    fn first_valid<It>(&self, it: It) -> String
    where
        It: Iterator<Item = (i64, i64)> + Clone,
    {
        let pairs = self.pairs();
        let mut digits = vec![9; LEN];

        for (ia, ib) in pairs {
            let mut z_pre = self.run(&digits);
            let (mut a_pre, mut b_pre) = (9, 9);
            for (a, b) in it.clone() {
                digits[ia] = a;
                digits[ib] = b;
                let z = self.run(&digits);
                match (z as f64)
                    .log(26.)
                    .ceil()
                    .partial_cmp(&(z_pre as f64).log(26.).ceil())
                {
                    Some(Ordering::Less) => {
                        break;
                    }
                    Some(Ordering::Greater) => {
                        digits[ia] = a_pre;
                        digits[ib] = b_pre;
                        break;
                    }
                    _ => {}
                }
                z_pre = z;
                a_pre = a;
                b_pre = b;
            }
        }

        digits.iter().map(i64::to_string).collect()
    }
}

struct Digits {
    num: i64,
    radix: i64,
    pos: u32,
}

impl Iterator for Digits {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 0 {
            return None;
        }
        self.pos -= 1;

        Some((self.num / self.radix.pow(self.pos)) % 10)
    }
}

impl Digits {
    #[allow(dead_code)]
    fn new(num: i64, radix: i64) -> Self {
        let mut pos = 1;
        while num / radix.pow(pos) > 0 {
            pos += 1;
        }
        Self { pos, num, radix }
    }
}

const LEN: usize = 14;
impl crate::Day for Alu {
    fn part1(&self) -> String {
        self.first_valid(iproduct!((1..=9).rev(), (1..=9).rev()))
    }

    fn part2(&self) -> String {
        self.first_valid(iproduct!(1..=9, 1..=9))
    }
}
