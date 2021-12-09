use std::collections::BTreeMap;
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input = input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x, y), c as i64 - '0' as i64))
            })
            .collect();
        Box::new(Day::new(input))
    }
}

type Input = BTreeMap<(usize, usize), i64>;

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
        self.input
            .iter()
            .filter_map(|((x, y), v)| {
                let v = *v + 1;
                let x = *x;
                let y = *y;
                let min = [
                    self.input.get(&(x.wrapping_sub(1), y)),
                    self.input.get(&(x + 1, y)),
                    self.input.get(&(x, y + 1)),
                    self.input.get(&(x, y.wrapping_sub(1))),
                    Some(&v),
                ]
                .into_iter()
                .filter_map(std::convert::identity)
                .min();

                (Some(&v) == min).then(|| v)
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}
