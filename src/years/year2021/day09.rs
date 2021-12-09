use std::collections::{BTreeMap, HashSet};
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
        let mut basins: Vec<HashSet<(usize, usize)>> =
            self.input.iter().fold(Vec::new(), |mut sets, (c, v)| {
                if *v != 9 {
                    let mut adjacent_sets: Vec<HashSet<(usize, usize)>> = sets
                        .drain_filter(|s| s.iter().any(|c2| adjacent(*c, *c2)))
                        .collect();

                    sets.push(if adjacent_sets.len() == 1 {
                        let mut s = adjacent_sets.remove(0);
                        s.insert(*c);
                        s
                    } else if adjacent_sets.is_empty() {
                        let mut s = HashSet::new();
                        s.insert(*c);
                        s
                    } else {
                        let mut s = adjacent_sets.swap_remove(0);
                        s.insert(*c);
                        for s2 in adjacent_sets.into_iter() {
                            s2.into_iter().for_each(|c2| {
                                s.insert(c2);
                            });
                        }
                        s
                    })
                }
                sets
            });
        basins.sort_by_key(HashSet::len);
        let last3: usize = basins.iter().rev().take(3).map(HashSet::len).product();

        format!("{:?}", last3)
    }
}

fn adjacent((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> bool {
    (ay == by && (ax as i8 - bx as i8).abs() == 1) || (ax == bx && (ay as i8 - by as i8).abs() == 1)
}
