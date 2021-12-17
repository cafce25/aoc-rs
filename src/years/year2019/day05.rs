use super::intcode::{Machine, Intcode};
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::new(input))
    }
}

struct Day {
    input: Intcode,
}

impl Day {
    pub fn new(input: &str) -> Self {
        let input = input.parse().unwrap();
        Self { input }
    }
    pub fn run(&self, input: i64) -> i64 {
        let mut machine = Machine::from(&self.input[..]);
        machine.input(input);
        machine.run();
        machine.last().unwrap()
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.run(1i64).to_string()
    }

    fn part2(&self) -> String {
        self.run(5i64).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Day;
    #[test]
    fn parts_test() {
        use crate::Day as _;
        let day = Day::new(crate::YEARS[&2019][&5].1);
        assert_eq!(day.part1(), "9025675".to_string());
        assert_eq!(day.part2(), "11981754".to_string());
    }
}
