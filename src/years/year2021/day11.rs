use std::{collections::HashMap, fmt};
pub struct DayGen;

static ADJACENT: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Oktopi;

struct Day {
    input: Input,
}

#[derive(Clone)]
struct Oktopus {
    energy: u8,
    flashed: bool,
}

impl Oktopus {
    fn grow(&mut self) {
        self.energy += 1;
    }
    fn flash(&mut self) -> bool {
        if !self.flashed && self.energy > 9 {
            self.flashed = true;
            return true;
        }
        false
    }
    fn reset(&mut self) {
        if self.flashed {
            self.flashed = false;
            self.energy = 0
        }
    }
}

impl fmt::Debug for Oktopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.energy))
    }
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    (
                        (row, col),
                        Oktopus {
                            energy: c as u8 - '0' as u8,
                            flashed: false,
                        },
                    )
                })
            })
            .flatten()
            .collect();
        Self {
            input: Oktopi(input),
        }
    }
}

#[derive(Clone)]
struct Oktopi(HashMap<(usize, usize), Oktopus>);

impl fmt::Debug for Oktopi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..10 {
            for c in 0..10 {
                f.write_str(&self.0[&(r, c)].energy.to_string())?
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}
impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut map = self.input.clone();
        let mut flashes = 0;
        for _ in 0..100 {
            map.0.values_mut().for_each(|v| v.grow());
            loop {
                let flashed_keys: Vec<(usize, usize)> = map
                    .0
                    .iter_mut()
                    .filter_map(|(k, v)| if v.flash() { Some(k.clone()) } else { None })
                    .collect();
                if flashed_keys.is_empty() {
                    break;
                } else {
                    flashes += flashed_keys.len();
                    flashed_keys
                        .iter()
                        .flat_map(|k| {
                            ADJACENT.iter().map(|dk| {
                                ((k.0 as i8 + dk.0) as usize, (k.1 as i8 + dk.1) as usize)
                            })
                        })
                        .for_each(|k| {
                            map.0.get_mut(&k).map(|v| v.grow());
                        });
                }
            }
            map.0.values_mut().for_each(|v| v.reset());
        }
        flashes.to_string()
    }

    fn part2(&self) -> String {
        let mut map = self.input.clone();
        let mut i = 0;
        loop {
            map.0.values_mut().for_each(|v| v.grow());
            loop {
                let flashed_keys: Vec<(usize, usize)> = map
                    .0
                    .iter_mut()
                    .filter_map(|(k, v)| if v.flash() { Some(k.clone()) } else { None })
                    .collect();
                if flashed_keys.is_empty() {
                    break;
                } else {
                    flashed_keys
                        .iter()
                        .flat_map(|k| {
                            ADJACENT.iter().map(|dk| {
                                ((k.0 as i8 + dk.0) as usize, (k.1 as i8 + dk.1) as usize)
                            })
                        })
                        .for_each(|k| {
                            map.0.get_mut(&k).map(|v| v.grow());
                        });
                }
            }
            i += 1;
            if map.0.values().all(|o| o.flashed) {
                break i;
            }
            map.0.values_mut().for_each(|v| v.reset());
        }
        .to_string()
    }
}
