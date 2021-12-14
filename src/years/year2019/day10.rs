use std::collections::HashSet;
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input<'a>(&self, input: &'a str) -> Box<dyn crate::Day + 'a> {
        Box::new(Day::from_str(input))
    }
}

type Input = HashSet<(i64, i64)>;

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(c, ast)| (ast == '#').then(|| (r as i64, c as i64)))
            })
            .collect();
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.input
            .iter()
            .map(|(row, col)| {
                self.input
                    .iter()
                    .filter(|(r, c)| {
                        if row == r && col == c {
                            //self
                            return false;
                        }
                        let mut dr = r - row;
                        let mut dc = c - col;
                        let gcd = if dr == 0 {
                            dc.abs()
                        } else if dc == 0 {
                            dr.abs()
                        } else {
                            num::integer::gcd(dr, dc)
                        };
                        dr /= gcd;
                        dc /= gcd;
                        for i in 1..gcd {
                            let coords = (i * dr + row, i * dc + col);
                            if self.input.contains(&coords) {
                                return false;
                            }
                        }
                        true
                    })
                    .count()
            })
            .max()
            .unwrap_or(0)
            .to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day as _;

    #[test]
    fn part1_tiny_test() {
        let input = concat![".#..#\n", ".....\n", "#####\n", "....#\n", "...##"];
        let day = Day::from_str(input);
        assert_eq!(day.part1(), "8");
    }
}
