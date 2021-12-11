use super::intcode::{Machine, Intcode};
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input = input
            .split(',')
            .filter_map(|line| line.trim().parse().ok())
            .collect();
        Box::new(Day::new(input))
    }
}

type Input = Intcode;

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
        let mut machine = Machine::from(&self.input[..]);
        machine.memory[1] = 12;
        machine.memory[2] = 2;
        machine.run();
        format!("{}", machine.memory[0])
    }

    fn part2(&self) -> String {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut machine = Machine::from(&self.input[..]);
                machine.memory[1] = noun;
                machine.memory[2] = verb;
                machine.run();
                if machine.memory[0] == 19690720 {
                    return format!("{}", 100 * noun + verb);
                }
            }
        }
        unreachable!();
    }
}
