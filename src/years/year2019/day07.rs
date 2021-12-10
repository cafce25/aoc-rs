use super::intcode::Machine;
use itertools::Itertools as _;
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
        let input = input.split(',').filter_map(|line| line.parse().ok()).collect();
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        (0..=4).permutations(5).map(|phases| {
            let amps = vec![Machine::from(&self.input);5];
            amps.into_iter().zip(phases).fold(0i64, |input, (mut amp, phase)| {
                amp.input(vec![phase, input]);
                amp.run();
                amp.output()[0]
            })
        }).max().unwrap().to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}
