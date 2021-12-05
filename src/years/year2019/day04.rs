use itertools::Itertools;
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let (start, finish) = input.trim().split_once('-').unwrap();
        Box::new(Day::new((start.parse().unwrap(), finish.parse().unwrap())))
    }
}

type Input = (i64, i64);

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
        (self.input.0..=self.input.1)
            .into_iter()
            .map(|i| i.to_string())
            .filter(|i| i.chars().tuple_windows().any(|(a, b)| a == b))
            .filter(|i| i.chars().tuple_windows().all(|(a, b)| a <= b))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        (self.input.0..=self.input.1)
            .into_iter()
            .map(|i| {
                i.to_string()
                    .chars()
                    .fold(Vec::new(), |mut v: Vec<(char, usize)>, c| {
                        match v.iter_mut().last() {
                            Some(last) if last.0 == c => last.1 += 1,
                            _ => v.push((c, 1)),
                        }
                        v
                    })
            })
            .filter(|i| i.iter().any(|(_, a)| *a == 2))
            .filter(|i| i.iter().tuple_windows().all(|((a, _), (b, _))| a < b))
            .count()
            .to_string()
    }
}
