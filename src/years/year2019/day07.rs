use super::intcode::Machine;
use itertools::Itertools as _;
use std::{cell::RefCell, ops::ControlFlow};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Vec<i64>;

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .split(',')
            .filter_map(|line| line.parse().ok())
            .collect();
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        (0..=4)
            .permutations(5)
            .map(|phases| {
                phases
                    .into_iter()
                    .map(|phase| {
                        let mut m = Machine::from(&self.input);
                        m.input(phase);
                        m
                    })
                    .fold(0i64, |input, mut amp| {
                        amp.input(input);
                        amp.last().unwrap()
                    })
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self) -> String {
        (5i64..=9)
            .permutations(5)
            .map(|phases| {
                let mut amps: Vec<_> = phases
                    .into_iter()
                    .map(|p| {
                        let mut m = Machine::from(&self.input);
                        m.input(p);
                        m
                    })
                    .collect();

                match amps
                    .iter_mut()
                    .map(RefCell::new)
                    .collect::<Vec<_>>()
                    .iter()
                    .cycle()
                    .try_fold(0, |input, amp| {
                        let mut amp = amp.borrow_mut();
                        amp.input(input);
                        if let Some(n) = amp.next() {
                            ControlFlow::Continue(n)
                        } else {
                            ControlFlow::Break(input)
                        }
                    }) {
                    ControlFlow::Continue(n) | ControlFlow::Break(n) => n,
                }
            })
            .max()
            .unwrap()
            .to_string()
    }
}
