type Input = Vec<i32>;
struct Day {
    input: Input,
}

pub struct DayGen;

impl Day {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input: Vec<_> = input
            .split('\n')
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();
        Box::new(Day::new(input))
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut acc = 0;
        let mut prev = None;
        self.input.iter().for_each(|n| {
            if let Some(p) = prev {
                if n > p {
                    acc += 1;
                }
            }
            prev = Some(n);
        });
        format!("{}", acc)
    }

    fn part2(&self) -> String {
        let input = self.input.iter();
        let mut acc = 0;
        let mut window = std::collections::VecDeque::new();
        let mut window_sum = 0;
        input.for_each(|n| {
            if window.len() < 3 {
                window.push_back(n);
                window_sum += n;
            } else {
                let old = window_sum;
                let f = window.pop_front().unwrap();
                window.push_back(n);
                window_sum = window_sum + n - f;
                if old < window_sum {
                    acc += 1;
                }
            }
        });
        format!("{}", acc)
    }
}
