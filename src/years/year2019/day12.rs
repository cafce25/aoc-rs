#![allow(dead_code, unused_variables)]

use std::{collections::HashMap, str::FromStr};
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(input.parse::<Day>().unwrap())
    }
}

type Input = Vec<Vec<i32>>;

struct Day {
    input: Input,
}

impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, input) = parsers::input(s).map_err(|e| e.to_string())?;
        Ok(Day { input })
    }
}

impl Day {
    fn run(&self, steps: usize) -> i32 {
        let n = self.input.len();
        let mut universe: Vec<(Vec<i32>, Vec<i32>)> = self
            .input
            .iter()
            .map(|p| (p.clone(), vec![0i32; p.len()]))
            .collect();
        for _ in 0..steps {
            for i in 0..n {
                for j in 0..n - i - 1 {
                    let mut it = universe.iter_mut();
                    let (pi, vi) = it.nth(i).unwrap();
                    let (pj, vj) = it.nth(j).unwrap();
                    let p_iter = pi.iter().zip(pj.iter());
                    let v_iter = vi.iter_mut().zip(vj.iter_mut());
                    p_iter.zip(v_iter).for_each(|((pi, pj), (vi, vj))| {
                        if pi > pj {
                            *vi -= 1;
                            *vj += 1;
                        } else if pj > pi {
                            *vi += 1;
                            *vj -= 1;
                        }
                    });
                }
                let (p, v) = &mut universe[i];
                p.iter_mut().zip(v.iter()).for_each(|(p, v)| {
                    *p += v;
                });
            }
        }
        universe
            .iter()
            .map(|(pos, vel)| {
                pos.iter().map(|x| x.abs()).sum::<i32>() * vel.iter().map(|x| x.abs()).sum::<i32>()
            })
            .sum::<i32>()
    }
    fn loops(&self) -> (Vec<usize>, Vec<usize>) {
        let mut cycle = vec![];
        let mut cycle_start: Vec<usize> = vec![];
        for coord in 0..3 {
            let mut visited = HashMap::new();
            let mut pos: Vec<_> = self.input.iter().map(|pos| pos[coord]).collect();
            let mut vel = vec![0; pos.len()];
            let mut i = 0;
            while !visited.contains_key(&(pos.clone(), vel.clone())) {
                visited.insert((pos.clone(), vel.clone()), i);
                i += 1;
                for i in 0..pos.len() {
                    for j in i + 1..pos.len() {
                        if pos[i] > pos[j] {
                            vel[i] -= 1;
                            vel[j] += 1;
                        }
                        if pos[j] > pos[i] {
                            vel[j] -= 1;
                            vel[i] += 1;
                        }
                    }
                }

                for i in 0..pos.len() {
                    pos[i] += vel[i]
                }
            }
            cycle.push(i);
            cycle_start.push(visited.remove(&(pos, vel)).unwrap());
        }
        (cycle, cycle_start)
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.run(1000).to_string()
    }

    fn part2(&self) -> String {
        let (lengths, starts) = self.loops();
        let length = lengths
            .iter()
            .fold(1usize, |acc, l| num::integer::lcm(acc, *l));

        length.to_string()
    }
}

mod parsers {
    use super::Input;
    use nom::{
        branch::permutation,
        bytes::complete::{is_a, tag},
        character::complete::char,
        combinator::{all_consuming, map_res, opt, recognize},
        multi::separated_list0,
        sequence::{delimited, preceded, terminated},
    };
    fn num(s: &str) -> nom::IResult<&str, i32> {
        map_res(
            recognize(preceded(opt(tag("-")), is_a("0123456789"))),
            |e: &str| e.parse::<i32>(),
        )(s)
    }

    fn axis<'a>(
        c: char,
    ) -> impl FnMut(&'a str) -> Result<(&'a str, i32), nom::Err<nom::error::Error<&'a str>>> {
        preceded(char(c), preceded(char('='), num))
    }

    fn coord(s: &str) -> nom::IResult<&str, Vec<i32>> {
        let (r, (x, y, z)) = (delimited(
            char('<'),
            permutation((
                terminated(axis('x'), opt(tag(", "))),
                terminated(axis('y'), opt(tag(", "))),
                terminated(axis('z'), opt(tag(", "))),
            )),
            char('>'),
        ))(s)?;
        Ok((r, vec![x, y, z]))
    }

    pub fn input(s: &str) -> nom::IResult<&str, Input> {
        all_consuming(separated_list0(char('\n'), coord))(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_s1_test() {
        let input = concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>",
        );

        let day = input.parse::<Day>().unwrap();
        assert_eq!(day.run(10), 179);
    }

    extern crate test;
    #[bench]
    fn part2(b: &mut test::Bencher) {
        let input = concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>",
        );

        let day = input.parse::<Day>().unwrap();
        let mut output = day.loops();
        b.iter(|| {
            output = day.loops();
        });
        assert_eq!(output, (vec![18, 28, 44], vec![0;3]) )
    }
}
