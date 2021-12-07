use super::intcode::Machine;
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input = input
            .split(',')
            .filter_map(|line| line.parse().ok())
            .collect();
        Box::new(Day::new(input))
    }
}

type Input = Vec<i64>;

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
        let mut machine = Machine::from(&self.input);
        machine.input(vec![1i64]);
        machine.run();
        format!("{}", machine.output().iter().last().unwrap())
    }

    fn part2(&self) -> String {
        todo!()
    }
}
