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
        format!(
            "{}",
            self.input.iter().map(|mass| mass / 3 - 2).sum::<i64>()
        )
    }

    fn part2(&self) -> String {
        format!(
            "{}",
            self.input
                .iter()
                .map(|mass| {
                    let mut mass = *mass;
                    let mut module_fuel = 0;
                    loop {
                        let fuel = mass / 3 - 2;
                        if fuel > 0 {
                            module_fuel += fuel;
                            mass = fuel;
                        } else {
                            break module_fuel;
                        }
                    }
                })
                .sum::<i64>()
        )
    }
}
