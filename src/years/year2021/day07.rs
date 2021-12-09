pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input = input
            .trim()
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
        let mut sorted = self.input.clone();
        sorted.sort_unstable();
        let median = sorted[sorted.len() / 2];
        let fuel: i64 = self.input.iter().map(|p| (p - median).abs()).sum();
        fuel.to_string()
    }

    fn part2(&self) -> String {
        let min = *self.input.iter().min().unwrap();
        let max = *self.input.iter().max().unwrap();
        let fuel: i64 = (min..max)
            .map(|i| {
                self.input
                    .iter()
                    .map(|p| (p - i).abs())
                    .map(|d| d * (d + 1) / 2)
                    .sum()
            })
            .min()
            .unwrap();
        fuel.to_string()
    }
}
