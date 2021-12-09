use std::collections::HashMap;
use std::str::FromStr;

use anyhow::anyhow;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input = input
            .split('\n')
            .filter_map(|line| line.parse().ok())
            .collect();
        Box::new(Day::new(input))
    }
}

type Input = Vec<Line>;
#[derive(Debug)]
struct Line {
    a: (i64, i64),
    b: (i64, i64),
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.a.0 == self.b.0
    }
    fn is_horizontal(&self) -> bool {
        self.a.1 == self.b.1
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(l: &str) -> anyhow::Result<Self> {
        let (a, b) = l
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("can't split points"))?;
        let a = a
            .split_once(',')
            .ok_or_else(|| anyhow!("can't split {}", a))?;
        let b = b
            .split_once(',')
            .ok_or_else(|| anyhow!("can't split {}", b))?;
        Ok(Self {
            a: (a.0.parse()?, a.1.parse()?),
            b: (b.0.parse()?, b.1.parse()?),
        })
    }
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
        let mut field: HashMap<(i64, i64), i64> = HashMap::new();
        self.input
            .iter()
            .filter(|l| l.is_vertical() || l.is_horizontal())
            .for_each(|l| {
                let dx = (l.b.0 - l.a.0).signum();
                let dy = (l.b.1 - l.a.1).signum();
                let d = (l.b.0 - l.a.0).abs().max((l.b.1 - l.a.1).abs());
                for i in 0..=d {
                    let x = l.a.0 + dx * i;
                    let y = l.a.1 + dy * i;
                    let key = (x, y);
                    *field.entry(key).or_insert(0) += 1;
                }
            });

        field.into_values().filter(|v| *v > 1).count().to_string()
    }

    fn part2(&self) -> String {
        let mut field: HashMap<(i64, i64), i64> = HashMap::new();
        self.input.iter().for_each(|l| {
            let dx = (l.b.0 - l.a.0).signum();
            let dy = (l.b.1 - l.a.1).signum();
            let d = (l.b.0 - l.a.0).abs().max((l.b.1 - l.a.1).abs());
            for i in 0..=d {
                let x = l.a.0 + dx * i;
                let y = l.a.1 + dy * i;
                let key = (x, y);
                *field.entry(key).or_insert(0) += 1;
            }
        });

        field.into_values().filter(|v| *v > 1).count().to_string()
    }
}
