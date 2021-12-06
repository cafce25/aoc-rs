use std::collections::HashMap;
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let mut map = HashMap::new();
        input.trim().split(',').for_each(|line| {
            map.entry(line.parse().unwrap())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        });
        Box::new(Day::new(map))
    }
}

type Input = HashMap<i8, usize>;

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
        (0..80).fold(self.input.to_owned(), |map, _| {
            let mut map: Input = map.into_iter().map(|(k, v)| (k-1, v)).collect();
            if let Some(n) = map.remove(&-1) {
                map.entry(6).and_modify(|v| *v += n).or_insert(n);
                map.insert(8, n);
            }
            map
        }).values().sum::<usize>().to_string()
    }

    fn part2(&self) -> String {
        (0..256).fold(self.input.to_owned(), |map, _| {
            let mut map: Input = map.into_iter().map(|(k, v)| (k-1, v)).collect();
            if let Some(n) = map.remove(&-1) {
                map.entry(6).and_modify(|v| *v += n).or_insert(n);
                map.insert(8, n);
            }
            map
        }).values().sum::<usize>().to_string()
    }
}
