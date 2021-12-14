use std::{collections::HashMap, str::FromStr};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

struct Input {
    template: Vec<char>,
    insertions: HashMap<(char, char), char>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (template, insertions) = s
            .split_once("\n\n")
            .ok_or_else(|| "Could not find \"\\n\\n\" in input".to_string())?;
        let rule_err = || "Invalid rule".to_string();
        let template = template.chars().collect();
        let insertions = insertions
            .lines()
            .map(|l| {
                let mut iter = l.chars();
                Ok((
                    (
                        iter.next().ok_or_else(rule_err)?,
                        iter.next().ok_or_else(rule_err)?,
                    ),
                    iter.nth(4).ok_or_else(rule_err)?,
                ))
            })
            .collect::<Result<HashMap<_, _>, String>>()?;
        Ok(Self {
            template,
            insertions,
        })
    }
}

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input: Input = input.parse().expect("parse input");
        Self { input }
    }
}

impl Day {
    fn run(&self, steps: usize) -> usize {
        let mut pairs = HashMap::new();
        self.input.template.windows(2).for_each(|pair| *pairs.entry((pair[0], pair[1])).or_insert(0) += 1);
        let pairs = (0..steps).fold(pairs, |pairs, _| {
            let mut n_pairs = HashMap::new();
            pairs.iter().for_each(|(p@(a, b), v)| {
                if let Some(i) = self.input.insertions.get(p) {
                    *n_pairs.entry((*a, *i)).or_insert(0) += v;
                    *n_pairs.entry((*i, *b)).or_insert(0) += v;
                }
                else {
                    *n_pairs.entry((*a, *b)).or_insert(0) += v;
                }
            });
            n_pairs
        });

        let mut hist = HashMap::new();
        pairs.iter().for_each(|((k, _), v)| *hist.entry(*k).or_insert(0) += v);

        *hist
            .entry(*self.input.template.last().unwrap())
            .or_insert(0) += 1;

        let (min, max) = hist
            .into_values()
            .fold((usize::MAX, 0), |(min, max), n| (min.min(n), max.max(n)));
        max - min
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.run(10).to_string()
    }

    fn part2(&self) -> String {
        self.run(40).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day as _;

    extern crate test;

    #[test]
    fn part1_test() {
        let input = crate::YEARS[&2021][&14].2;
        let day = Day::from_str(input);
        assert_eq!("1588", day.part1());
    }

    #[bench]
    fn part2(b: &mut test::Bencher) {
        let input = crate::YEARS[&2021][&14].2;
        let day = Day::from_str(input);
        b.iter(|| {
            day.part2();
        })
    }
}
