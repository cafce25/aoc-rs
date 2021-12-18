use std::ops::RangeInclusive;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

struct Day {
    input: (RangeInclusive<i32>, RangeInclusive<i32>),
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        Self {
            input: parsers::parse_input(input),
        }
    }

    fn simulate(&self, mut vy: i32, mut vx: i32) -> bool {
        let mut y = 0;
        let mut x = 0;
        while y >= *self.input.0.start() && x <= *self.input.1.end() {
            if self.input.0.contains(&y) && self.input.1.contains(&x) {
                return true;
            }
            y += vy;
            x += vx;
            vy -= 1;
            if vx > 0 {
                vx -= 1;
            }
        }
        false
    }

    #[inline(always)]
    fn bounds(&self) -> (i32, i32, i32, i32) {
        (
            *self.input.0.start(),
            -*self.input.0.start(),
            ((*self.input.1.start() as f32 * 2. + 0.25).sqrt()).ceil() as i32,
            *self.input.1.end(),
        )
    }
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(i32),
    {
        let (min_vy, max_vy, min_vx, max_vx) = self.bounds();
        for vx in min_vx..=max_vx {
            for vy in min_vy..=max_vy {
                if self.simulate(vy, vx) {
                    f(vy);
                }
            }
        }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut max_y = 0;
        self.run(|vy| max_y = max_y.max((vy * (vy + 1)) / 2));
        max_y.to_string()
    }

    fn part2(&self) -> String {
        let mut n = 0;
        self.run(|_| n += 1);
        n.to_string()
    }
}

mod parsers {
    use std::ops::RangeInclusive;

    use nom::{
        branch::permutation,
        bytes::complete::tag,
        character::complete::{char, i32},
        combinator::{all_consuming, map},
        sequence::{preceded, separated_pair, tuple},
        IResult,
    };

    pub fn parse_input(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
        all_consuming(parse)(input).unwrap().1
    }

    fn parse(input: &str) -> IResult<&str, (RangeInclusive<i32>, RangeInclusive<i32>)> {
        map(
            preceded(
                tag("target area: "),
                permutation((range('y'), tag(", "), range('x'))),
            ),
            |(a, _, b)| (a, b),
        )(input)
    }

    fn range(c: char) -> impl FnMut(&str) -> IResult<&str, RangeInclusive<i32>> {
        move |input| {
            map(
                preceded(
                    tuple((char(c), char('='))),
                    separated_pair(i32, tag(".."), i32),
                ),
                |(from, to): (i32, i32)| -> RangeInclusive<i32> { from..=to },
            )(input)
        }
    }
}
