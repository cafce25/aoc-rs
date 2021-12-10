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
                s.split_once(')')
                    .map(|(center, satelite)| (satelite, center))
            })
            .collect();
        Self { input }
    }

    pub fn suborbits(&self) -> HashMap<&str, Vec<&str>> {
        let mut sorbits = HashMap::new();
        self.input.keys().for_each(|outmost| {
            if !sorbits.contains_key(outmost) {
                let mut rev_orbit = Vec::new();
                let mut current = outmost;
                rev_orbit.push(*current);
                while let Some(c) = self.input.get(current) {
                    current = c;
                    rev_orbit.push(current);
                }
                sorbits.insert(*outmost, rev_orbit.into_iter().skip(1).rev().collect());
            }
        });
        sorbits
    }
}

impl<'a> crate::Day for Day<'a> {
    fn part1(&self) -> String {
        self.suborbits().values().map(Vec::len).sum::<usize>().to_string()
    }

    fn part2(&self) -> String {
        let orbits = self.suborbits();
        let san = &orbits["SAN"];
        let you = &orbits["YOU"];
        let mut i = 0;
        while san[0..=i] == you[0..=i] { i += 1 }
        (san.len() + you.len() - 2 * i).to_string()
    }
}
