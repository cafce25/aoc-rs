#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    str::FromStr,
};

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

    fn value(&self) -> u64 {
        let mut input: HashSet<_> = self.input.clone().into_iter().collect();
        let mut map = HashMap::new();

        if let Some(digit) = input.iter().find(|e| e.len() == 2) {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(1, digit);
        };

        if let Some(digit) = input.iter().find(|e| e.len() == 4) {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(4, digit);
        };

        if let Some(digit) = input.iter().find(|e| e.len() == 3) {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(7, digit);
        };

        if let Some(digit) = input.iter().find(|e| e.len() == 7) {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(8, digit);
        };

        if let Some(digit) = input
            .iter()
            .find(|e| e.len() == 5 && map[&1].wires.iter().all(|w| e.wires.contains(w)))
        {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(3, digit);
        }

        if let Some(digit) = input
            .iter()
            .find(|e| e.len() == 6 && map[&4].wires.iter().all(|w| e.wires.contains(w)))
        {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(9, digit);
        }

        if let Some(digit) = input
            .iter()
            .find(|e| e.len() == 5 && e.wires.iter().all(|w| map[&9].wires.contains(w)))
        {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(5, digit);
        }

        if let Some(digit) = input.iter().find(|e| e.len() == 5) {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(2, digit);
        }

        if let Some(digit) = input
            .iter()
            .find(|e| map[&5].wires.iter().all(|w| e.wires.contains(w)))
        {
            let digit = digit.clone();
            input.remove(&digit);
            map.insert(6, digit);
        }

        input.drain().for_each(|e| {
            map.insert(0, e);
        });

        let map: HashMap<_, _> = map.into_iter().map(|(k, v)| (v, k)).collect();
        self.output.iter().fold(0, |n, d| map[d] + 10*n)
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Digit {
    wires: HashSet<Wire>,
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
        let wires: Result<HashSet<_>, _> = s
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

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Digit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.wires.iter().map(|a| 1 << (*a as u8)).sum::<u8>())
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
        self.input
            .iter()
            .map(Digits::value)
            .sum::<u64>()
            .to_string()
    }
}
