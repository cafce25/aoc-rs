use std::{collections::{HashMap, LinkedList, linked_list::CursorMut}, str::FromStr};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

struct Input {
    template: String,
    insertions: HashMap<(char, char), char>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (template, insertions) = s
            .split_once("\n\n")
            .ok_or_else(|| "Could not find \n\n in input".to_string())?;
        let rule_err = || "Invalid rule".to_string();
        let template = template.to_string();
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
            .collect::<Result<HashMap<_,_>,String>>()?;
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

fn peek2(cur: &mut CursorMut<char>) -> Option<(char, char)> {
    let first = *cur.current()?;
    let second = *cur.peek_next()?;
    Some((first, second))
}

impl Day {
    fn run(&self, steps: usize) -> usize {
        let mut poly: LinkedList<char> = self.input.template.chars().collect();
        for _ in 0..steps {
            let mut cur = poly.cursor_front_mut();
            while let Some(pair) = peek2(&mut cur) {
                if let Some(ins) = self.input.insertions.get(&pair) {
                    cur.insert_after(*ins);
                    cur.move_next();
                }
                cur.move_next();
            }
        }
        let mut hist :HashMap<char, usize> = HashMap::new();
        poly.iter().for_each(|c| *hist.entry(*c).or_insert(0) += 1);
        let (min, max) = hist.into_values().fold((usize::MAX, 0), |(min, max), n| {
            (min.min(n), max.max(n))
        });
        max - min
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.run(10).to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::Day as _;
    use super::*;

    #[test]
    fn part1_test() {
        let input = crate::YEARS[&2021][&14].2;
        let day = Day::from_str(input);
        assert_eq!("1588", day.part1());
    }
}
