use std::{
    ops::{Add, Sub},
    str::FromStr, fmt::Debug,
};

use itertools::iproduct;

type Instructions = Vec<Instruction>;

type Instruction = (bool, Cuboid);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cuboid {
    x_range: [i64; 2],
    y_range: [i64; 2],
    z_range: [i64; 2],
}

impl Cuboid {
    fn intersects(&self, other: &Self) -> bool {
        self.x_range[1] >= other.x_range[0]
            && self.x_range[0] <= other.x_range[1]
            && self.y_range[1] >= other.y_range[0]
            && self.y_range[0] <= other.y_range[1]
            && self.z_range[1] >= other.z_range[0]
            && self.z_range[0] <= other.z_range[1]
    }

    fn size(&self) -> i64 {
        (self.x_range[1] - self.x_range[0] + 1)
            * (self.y_range[1] - self.y_range[0] + 1)
            * (self.z_range[1] - self.z_range[0] + 1)
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        self.intersects(other).then(|| Cuboid {
            x_range: [
                self.x_range[0].max(other.x_range[0]),
                self.x_range[1].min(other.x_range[1]),
            ],
            y_range: [
                self.y_range[0].max(other.y_range[0]),
                self.y_range[1].min(other.y_range[1]),
            ],
            z_range: [
                self.z_range[0].max(other.z_range[0]),
                self.z_range[1].min(other.z_range[1]),
            ],
        })
    }
}


impl Add for Cuboid {
    type Output = Vec<Cuboid>;

    fn add(self, other: Self) -> Self::Output {
        if !self.intersects(&other) {
            return vec![self, other];
        }

        let mut res = self - other;
        res.push(other);
        res
    }
}
impl Sub for Cuboid {
    type Output = Vec<Cuboid>;

    fn sub(self, other: Self) -> Self::Output {
        if !self.intersects(&other) {
            return vec![self];
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        let ranges: Vec<_> = [
            (self.x_range, other.x_range),
            (self.y_range, other.y_range),
            (self.z_range, other.z_range),
        ]
        .into_iter()
        .map(|(self_range, other_range)| {
            [
                (self_range[0] < other_range[0]).then(|| [self_range[0], other_range[0] - 1]),
                Some([
                    self_range[0].max(other_range[0]),
                    self_range[1].min(other_range[1]),
                ]),
                (self_range[1] > other_range[1]).then(|| [other_range[1] + 1, self_range[1]]),
            ]
        })
        .collect();
        iproduct!(0..3, 0..3, 0..3)
            .filter_map(|(x, y, z)| {
                let x_range = ranges[0][x]?;
                let y_range = ranges[1][y]?;
                let z_range = ranges[2][z]?;
                (x != 1 || y != 1 || z != 1).then(|| Cuboid {
                    x_range,
                    y_range,
                    z_range,
                })
            })
            .collect()
    }
}

struct Day {
    instructions: Instructions,
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let universe = self
            .instructions
            .iter()
            .copied()
            .filter_map(|(s, c)| {
                Some((
                    s,
                    c.intersection(&Cuboid {
                        x_range: [-50, 50],
                        y_range: [-50, 50],
                        z_range: [-50, 50],
                    })?,
                ))
            })
            .fold(Vec::new(), |universe, (status, cuboid)| {
                let mut universe: Vec<_> = universe
                    .into_iter()
                    .flat_map(|c: Cuboid| (c - cuboid).into_iter())
                    .collect();
                if status {
                    universe.push(cuboid);
                }
                universe
            });

        universe
            .into_iter()
            .map(|c| c.size())
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self) -> String {
        let universe =
            self.instructions
                .iter()
                .copied()
                .fold(Vec::new(), |universe, (status, cuboid)| {
                    let mut universe: Vec<_> = universe
                        .into_iter()
                        .flat_map(|c: Cuboid| (c - cuboid).into_iter())
                        .collect();
                    if status {
                        universe.push(cuboid);
                    }
                    universe
                });
        universe
            .into_iter()
            .map(|c| c.size())
            .sum::<i64>()
            .to_string()
    }
}

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let day = Day::from_str(input);
        match day {
            Ok(day) => Box::new(day),
            Err(e) => panic!("{}", e),
        }
    }
}

impl Debug for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x={}..{},y={}..{},z={}..{}",
                self.x_range[0],
                self.x_range[1],
                self.y_range[0],
                self.y_range[1],
                self.z_range[0],
                self.z_range[1],
        ))
    }
}

mod parsers {
    use std::str::FromStr;

    use crate::years::year2021::day22::Cuboid;

    use super::{Day, Instruction, Instructions};
    use anyhow::{anyhow, Error, Result};
    use nom::{
        branch::{alt, permutation},
        bytes::complete::tag,
        character::complete::{char, i64},
        combinator::{all_consuming, map, opt},
        multi::separated_list1,
        sequence::{preceded, separated_pair, tuple},
        IResult,
    };

    impl FromStr for Day {
        fn from_str(i: &str) -> Result<Self> {
            let cuboids = input(i)?;
            Ok(Self {
                instructions: cuboids,
            })
        }

        type Err = Error;
    }

    pub(crate) fn input(input: &str) -> Result<Instructions> {
        Ok(all_consuming(instructions)(input)
            .map_err(|e| anyhow!(format!("{}", e)))?
            .1)
    }

    fn instructions(input: &str) -> IResult<&str, Instructions> {
        separated_list1(char('\n'), instruction)(input)
    }
    fn instruction(input: &str) -> IResult<&str, Instruction> {
        map(
            separated_pair(
                action,
                char(' '),
                permutation((range('x'), range('y'), range('z'))),
            ),
            |(a, (x_range, y_range, z_range))| {
                (
                    a,
                    Cuboid {
                        x_range,
                        y_range,
                        z_range,
                    },
                )
            },
        )(input)
    }

    fn action(input: &str) -> IResult<&str, bool> {
        alt((map(tag("on"), |_| true), map(tag("off"), |_| false)))(input)
    }

    fn range(c: char) -> impl Fn(&str) -> IResult<&str, [i64; 2]> {
        move |i: &str| {
            preceded(
                tuple((opt(char(',')), char(c), char('='))),
                map(separated_pair(i64, tag(".."), i64), |(start, end)| {
                    [start, end]
                }),
            )(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn intersection_test() {
        let a = Cuboid {
            x_range: [0, 10],
            y_range: [0, 7],
            z_range: [0, 10],
        };
        let b = Cuboid {
            x_range: [5, 10],
            y_range: [1, 10],
            z_range: [2, 10],
        };
        let c = a.intersection(&b).unwrap();
        assert_eq!(
            Cuboid {
                x_range: [5, 10],
                y_range: [1, 7],
                z_range: [2, 10],
            },
            c
        );
    }

    #[test]
    fn addition_test() {
        let a = Cuboid {
            x_range: [0, 5],
            y_range: [0, 5],
            z_range: [0, 5],
        };
        let b = Cuboid {
            x_range: [4, 10],
            y_range: [4, 10],
            z_range: [4, 10],
        };
        let c: HashSet<_> = (a + b).into_iter().collect();
        assert_eq!(
            HashSet::from([
                Cuboid {
                    x_range: [0, 3],
                    y_range: [0, 3],
                    z_range: [0, 3],
                },
                Cuboid {
                    x_range: [0, 3],
                    y_range: [4, 5],
                    z_range: [4, 5],
                },
                Cuboid {
                    x_range: [4, 5],
                    y_range: [0, 3],
                    z_range: [4, 5],
                },
                Cuboid {
                    x_range: [0, 3],
                    y_range: [0, 3],
                    z_range: [4, 5],
                },
                Cuboid {
                    x_range: [4, 5],
                    y_range: [4, 5],
                    z_range: [0, 3],
                },
                Cuboid {
                    x_range: [0, 3],
                    y_range: [4, 5],
                    z_range: [0, 3],
                },
                Cuboid {
                    x_range: [4, 5],
                    y_range: [0, 3],
                    z_range: [0, 3],
                },
                b
            ]),
            c
        );
    }

    #[test]
    fn mini_test() {
        let input = concat!(
            "on x=10..12,y=10..12,z=10..12\n",
            "on x=11..13,y=11..13,z=11..13\n",
            "off x=9..11,y=9..11,z=9..11\n",
            "on x=10..10,y=10..10,z=10..10",
        );
        let day = Day::from_str(input).unwrap();
        assert_eq!(crate::Day::part1(&day), "39");
    }
}
