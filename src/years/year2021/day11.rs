use std::{collections::HashMap, fmt};

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

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Oktopi;

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, c)| ((row, col), c as u8 - '0' as u8))
            })
            .flatten()
            .collect();
        Self {
            input: Oktopi(input),
        }
    }
}

#[derive(Clone)]
struct Oktopi(HashMap<(usize, usize), u8>);

impl std::ops::Deref for Oktopi {
    type Target = HashMap<(usize, usize), u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Oktopi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Oktopi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..10 {
            for c in 0..10 {
                f.write_str(&self.0[&(r, c)].to_string())?
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

fn grow_all(map: &mut Oktopi) -> usize {
    let mut to_be_grown: Vec<_> = map.0.keys().copied().collect();
    while !to_be_grown.is_empty() {
        let grow @ (x, y) = to_be_grown.pop().unwrap();
        if let Some(oktopus) = map.get_mut(&grow) {
            if {
                *oktopus += 1;
                *oktopus == 10
            } {
                let mut neighbours = ADJACENT
                    .iter()
                    .map(|(dx, dy)| ((x as i8 + dx) as usize, (y as i8 + dy) as usize))
                    .collect();
                to_be_grown.append(&mut neighbours);
            }
        }
    }
    map.values_mut()
        .filter_map(|o| (*o > 9).then(|| *o = 0))
        .count()
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut map = self.input.clone();
        let mut flashes = 0;
        for _ in 0..100 {
            flashes += grow_all(&mut map);
        }
        flashes.to_string()
    }

    fn part2(&self) -> String {
        let mut map = self.input.clone();
        let mut days: u32 = 0;
        loop {
            days += 1;
            if grow_all(&mut map) >= map.len() {
                break days;
            }
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day as _;

    #[test]
    fn part1_test() {
        let day = Day::from_str(crate::YEARS[&2021][&11].1);
        assert_eq!(day.part1(), "1705");
    }

    #[test]
    fn part2_test() {
        let day = Day::from_str(crate::YEARS[&2021][&11].1);
        assert_eq!(day.part2(), "265");
    }
}
