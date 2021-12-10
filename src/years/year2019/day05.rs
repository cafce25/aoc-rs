use super::intcode::Machine;
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::new(input))
    }
}

type Input = Vec<i64>;

struct Day {
    input: Input,
}

impl Day {
    pub fn new(input: &str) -> Self {
        let input = input
            .trim()
            .split(',')
            .filter_map(|line| line.parse().ok())
            .collect();
        Self { input }
    }
    pub fn run(&self, input: i64) -> i64 {
        let mut machine = Machine::from(&self.input);
        machine.input(vec![input]);
        machine.run();
        *machine.output().iter().last().unwrap()
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

mod tests {
    // for some reason rust analyzer doesn't recognize this is being used
    #![allow(unused_imports)]
    use super::Day;

    #[test]
    fn equals_position_mode_test() {
        let day = Day::new("3,9,8,9,10,9,4,9,99,-1,8");
        assert_eq!(day.run(7), 0);
        assert_eq!(day.run(8), 1);
        assert_eq!(day.run(9), 0);
    }

    #[test]
    fn less_than_position_mode_test() {
        let day = Day::new("3,9,7,9,10,9,4,9,99,-1,8");
        assert_eq!(day.run(7), 1);
        assert_eq!(day.run(8), 0);
        assert_eq!(day.run(9), 0);
    }

    #[test]
    fn equals_immediate_mode_test() {
        let day = Day::new("3,3,1108,-1,8,3,4,3,99");
        assert_eq!(day.run(7), 0);
        assert_eq!(day.run(8), 1);
        assert_eq!(day.run(9), 0);
    }

    #[test]
    fn less_than_immediate_mode_test() {
        let day = Day::new("3,3,1107,-1,8,3,4,3,99");
        assert_eq!(day.run(7), 1);
        assert_eq!(day.run(8), 0);
        assert_eq!(day.run(9), 0);
    }

    #[test]
    fn jump_position_mode_test() {
        let day = Day::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        assert_eq!(day.run(0), 0);
        assert_eq!(day.run(1), 1);
        assert_eq!(day.run(-1), 1);
    }

    #[test]
    fn jump_immediate_mode_test() {
        let day = Day::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        assert_eq!(day.run(0), 0);
        assert_eq!(day.run(1), 1);
        assert_eq!(day.run(-1), 1);
    }

    #[test]
    fn larger_example_test() {
        let day = Day::new(concat!(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,",
            "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,",
            "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        ));
        assert_eq!(day.run(5), 999);
        assert_eq!(day.run(8), 1000);
        assert_eq!(day.run(10), 1001);
    }

    #[test]
    fn parts_test() {
        use crate::Day as _;
        let day = Day::new(crate::YEARS[&2019][&5].1);
        assert_eq!(day.part1(), "9025675".to_string());
        assert_eq!(day.part2(), "11981754".to_string());
    }
}
