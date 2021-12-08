#![allow(dead_code)]
use std::str::FromStr;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input = input
            .lines()
            .map(|l| l.parse().unwrap())
            // .filter_map(|line| line.parse().ok())
            .collect();
        Box::new(Day::new(input))
    }
}

type Input = Vec<Digits>;

#[derive(Clone, Debug)]
struct Digits {
    input: Vec<Digit>,
    output: Vec<Digit>,
}

impl Digits {
    fn outputs<'a>(&'a self) -> impl Iterator<Item = &Digit> + 'a {
        self.output.iter()
    }
    fn inputs<'a>(&'a self) -> impl Iterator<Item = &Digit> + 'a {
        self.input.iter()
    }
}

impl FromStr for Digits {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (input, output) = s
            .split_once(" | ")
            .ok_or_else(|| anyhow::anyhow!("can't split"))?;
        let input: Result<Vec<_>, _> = input.split(' ').map(|d| d.parse()).collect();
        let output: Result<Vec<_>, _> = output.split(' ').map(|d| d.parse()).collect();
        Ok(Self {
            input: input?,
            output: output?,
        })
    }
}

#[derive(Clone, Debug)]
struct Digit {
    wires: Vec<Wire>,
}
impl Digit {
    fn len(&self) -> usize {
        self.wires.len()
    }
}

impl FromStr for Digit {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        use Wire::*;
        let wires: Result<Vec<_>, _> = s
            .chars()
            .map(|c| {
                Ok(match c {
                    'a' => A,
                    'b' => B,
                    'c' => C,
                    'd' => D,
                    'e' => E,
                    'f' => F,
                    'g' => G,
                    _ => return Err(anyhow::anyhow!("invalid wire letter")),
                })
            })
            .collect();
        Ok(Self { wires: wires? })
    }
}
impl Digit {}

#[derive(Copy, Clone, Debug)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

struct Day {
    input: Input,
}

impl Day {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.input
            .iter()
            .flat_map(Digits::outputs)
            .filter(|o| [2, 4, 3, 7].contains(&o.len()))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}
