use super::intcode::{Intcode, Machine};
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

struct Day {
    input: Intcode,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        Self { input: input.parse().unwrap() }
    }
    fn run(&self, input: i64) -> Vec<i64> {
        let mut machine = Machine::from(&self.input[..]);
        machine.input(input);
        machine.collect()
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        format!("{}", self.run(1)[0])
    }

    fn part2(&self) -> String {
        format!("{}", self.run(2)[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boost_test() {
        let day = Day::from_str(crate::YEARS[&2019][&9].1);
        let output = day.run(1);
        assert_eq!(output,  [2350741403]);
    }
}
