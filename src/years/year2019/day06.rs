use std::collections::HashMap;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input<'a>(&'a self, input: &'a str) -> Box<dyn crate::Day + 'a> {
        Box::new(Day::from_str(input))
    }
}

type Input<'a> = HashMap< &'a str, &'a str>;

struct Day<'a> {
    input: Input<'a>,
}

impl<'a> Day<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let input = input
            .trim()
            .lines()
            .filter_map(|s| {
                s.split_once(")")
                    .map(|(center, satelite)| (satelite, center))
            })
            .collect();
        Self { input }
    }
}

impl<'a> crate::Day for Day<'a> {
    fn part1(&self) -> String {
        self.input.keys().map(|mut satelite| {
            let mut orbits = 0;
            while let Some(sat) = self.input.get(satelite) {
                satelite = sat;
                orbits += 1;
            }
            orbits
        }).sum::<usize>().to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}
