use anyhow::{anyhow, Error, Result};
use itertools::Itertools as _;
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum SnailNumber {
    Pair {
        left: Box<SnailNumber>,
        right: Box<SnailNumber>,
    },
    Literal(u8),
}

impl SnailNumber {
    fn join(self, rhs: Self) -> Self {
        SnailNumber::Pair {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }

    fn reduce(&mut self) -> bool {
        self.explode() || self.split()
    }

    fn explode(&mut self) -> bool {
        let mut distribute = None;
        self.explode_rec(0, &mut distribute);

        distribute.is_some()
    }

    fn explode_rec(&mut self, i: u8, distribute: &mut Option<(u8, u8)>) {
        if i > 4 {
            unreachable!();
        }
        if let Some((left, right)) = distribute {
            if *left == 0 && *right == 0 {
                return;
            }
            match self {
                SnailNumber::Literal(inner) => {
                    *inner += *left + *right;
                    *left = 0;
                    *right = 0;
                }
                SnailNumber::Pair {
                    left: left_s,
                    right: right_s,
                } => {
                    if left > right {
                        right_s.explode_rec(0, distribute);
                        left_s.explode_rec(0, distribute);
                    } else {
                        left_s.explode_rec(0, distribute);
                        right_s.explode_rec(0, distribute);
                    }
                }
            }
            return;
        }
        if i == 4 {
            match self {
                SnailNumber::Pair { left, right } => {
                    let ret = (
                        left.as_ref().try_into().unwrap(),
                        right.as_ref().try_into().unwrap(),
                    );
                    *self = SnailNumber::Literal(0);
                    *distribute = Some(ret)
                }
                SnailNumber::Literal(_) => {}
            }
            return;
        }
        match self {
            SnailNumber::Pair { left, right } => {
                left.explode_rec(i + 1, distribute);

                if distribute.is_some() {
                    let mut sub_distri = Some((0, distribute.unwrap().1));
                    right.explode_rec(i + 1, &mut sub_distri);
                    *distribute = Some((distribute.unwrap().0, sub_distri.unwrap().1));
                    return;
                } else {
                    right.explode_rec(i + 1, distribute);
                    if distribute.is_some() {
                        let mut sub_distri = Some((distribute.unwrap().0, 0));
                        left.explode_rec(i + 1, &mut sub_distri);
                        *distribute = Some((sub_distri.unwrap().0, distribute.unwrap().1));
                        return;
                    }
                }
            }
            SnailNumber::Literal(_) => {}
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailNumber::Literal(n) if n > &mut 9 => {
                *self = SnailNumber::Pair {
                    left: Box::new(SnailNumber::Literal(*n / 2)),
                    right: Box::new(SnailNumber::Literal((*n + 1) / 2)),
                };
                true
            }
            SnailNumber::Pair { left, right } => left.split() || right.split(),
            SnailNumber::Literal(_) => false,
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailNumber::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
            SnailNumber::Literal(n) => *n as u32,
        }
    }
}

impl std::fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNumber::Pair { left, right } => {
                f.write_str("[")?;
                left.fmt(f)?;
                f.write_str(",")?;
                right.fmt(f)?;
                f.write_str("]")?;
            }
            SnailNumber::Literal(n) => f.write_fmt(format_args!("{}", n))?,
        }
        Ok(())
    }
}
impl TryFrom<&SnailNumber> for u8 {
    type Error = Error;

    fn try_from(value: &SnailNumber) -> Result<Self, Self::Error> {
        if let SnailNumber::Literal(value) = value {
            Ok(*value)
        } else {
            Err(anyhow!("cannot convert non literal to u8"))
        }
    }
}

impl TryFrom<SnailNumber> for u8 {
    type Error = Error;

    fn try_from(value: SnailNumber) -> Result<Self, Self::Error> {
        if let SnailNumber::Literal(value) = value {
            Ok(value)
        } else {
            Err(anyhow!("cannot convert non literal to u8"))
        }
    }
}

impl TryFrom<SnailNumber> for (u8, u8) {
    type Error = Error;

    fn try_from(value: SnailNumber) -> Result<Self, Self::Error> {
        if let SnailNumber::Pair { left, right } = value {
            if let (SnailNumber::Literal(left), SnailNumber::Literal(right)) =
                (left.as_ref(), right.as_ref())
            {
                Ok((*left, *right))
            } else {
                Err(anyhow!("either is not a literal"))
            }
        } else {
            Err(anyhow!("missing one or two literals"))
        }
    }
}

impl From<(SnailNumber, SnailNumber)> for SnailNumber {
    fn from((left, right): (SnailNumber, SnailNumber)) -> Self {
        left.join(right)
    }
}

impl From<(u8, u8)> for SnailNumber {
    fn from((left, right): (u8, u8)) -> SnailNumber {
        SnailNumber::Pair {
            left: Box::new(SnailNumber::Literal(left)),
            right: Box::new(SnailNumber::Literal(right)),
        }
    }
}

impl From<u8> for SnailNumber {
    fn from(val: u8) -> Self {
        SnailNumber::Literal(val)
    }
}

impl std::ops::Add for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = SnailNumber::join(self, rhs);
        while res.reduce() {}
        res
    }
}

impl std::iter::Sum for SnailNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, o| acc + o).unwrap_or(0u8.into())
    }
}

struct Day {
    numbers: Vec<SnailNumber>,
}

impl Day {
    pub fn from_str(numbers: &str) -> Self {
        Self {
            numbers: parsers::parse(numbers),
        }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.numbers
            .iter()
            .cloned()
            .sum::<SnailNumber>()
            .magnitude()
            .to_string()
    }

    fn part2(&self) -> String {
        self.numbers
            .iter()
            .cloned()
            .permutations(2)
            .map(|mut pair| (pair.pop().unwrap() + pair.pop().unwrap()).magnitude())
            .max()
            .unwrap()
            .to_string()
    }
}

mod parsers {
    use super::*;
    use nom::{
        branch::alt,
        character::complete::char,
        combinator::{all_consuming, into},
        multi::separated_list0,
        sequence::{delimited, separated_pair},
        IResult,
    };

    pub fn parse(input: &str) -> Vec<SnailNumber> {
        all_consuming(numbers)(input).unwrap().1
    }

    fn numbers(input: &str) -> IResult<&str, Vec<SnailNumber>> {
        separated_list0(char('\n'), number)(input)
    }

    pub fn number(input: &str) -> IResult<&str, SnailNumber> {
        alt((into(u8), into(pair)))(input)
    }

    fn u8(input: &str) -> IResult<&str, u8> {
        nom::character::complete::u8(input)
    }

    fn pair(input: &str) -> IResult<&str, (SnailNumber, SnailNumber)> {
        delimited(
            char('['),
            separated_pair(number, char(','), number),
            char(']'),
        )(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn literal_test() {
            for i in 0..=9 {
                let (_, number) = number(&format!("{}", i)).unwrap();
                assert_eq!(SnailNumber::Literal(i), number);
            }
        }

        #[test]
        fn pair_test() {
            for i in 0..=8 {
                let (_, number) = number(&format!("[{},{}]", i, i + 1)).unwrap();
                assert_eq!(
                    SnailNumber::Pair {
                        left: Box::new(SnailNumber::Literal(i)),
                        right: Box::new(SnailNumber::Literal(i + 1)),
                    },
                    number
                );
            }
        }

        #[test]
        fn nested_test() {
            let (_, number) = number("[[1,2],[3,4]]").unwrap();
            assert_eq!(
                SnailNumber::Pair {
                    left: Box::new(SnailNumber::Pair {
                        left: Box::new(SnailNumber::Literal(1)),
                        right: Box::new(SnailNumber::Literal(2)),
                    }),
                    right: Box::new(SnailNumber::Pair {
                        left: Box::new(SnailNumber::Literal(3)),
                        right: Box::new(SnailNumber::Literal(4)),
                    }),
                },
                number
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dont_explode_test() {
        let (_, mut before) = parsers::number("[[[[0,9],2],3],4]").unwrap();
        let (_, after) = parsers::number("[[[[0,9],2],3],4]").unwrap();
        assert!(!before.explode());
        assert_eq!(before, after);
    }

    #[test]
    fn explode1_test() {
        let (_, mut before) = parsers::number("[[[[[9,8],1],2],3],4]").unwrap();
        let (_, after) = parsers::number("[[[[0,9],2],3],4]").unwrap();
        assert!(before.explode());
        assert_eq!(before, after);
    }

    #[test]
    fn explode2_test() {
        let (_, mut before) = parsers::number("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let (_, after) = parsers::number("[7,[6,[5,[7,0]]]]").unwrap();
        assert!(before.explode());
        assert_eq!(before, after);
    }

    #[test]
    fn explode3_test() {
        let (_, mut before) = parsers::number("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let (_, after) = parsers::number("[[6,[5,[7,0]]],3]").unwrap();
        assert!(before.explode());
        assert_eq!(before, after);
    }

    #[test]
    fn explode4_test() {
        let (_, mut before) = parsers::number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let (_, after) = parsers::number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        assert!(before.explode());
        assert_eq!(before, after);
    }

    #[test]
    fn explode5_test() {
        let (_, mut before) = parsers::number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        let (_, after) = parsers::number("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap();
        assert!(before.explode());
        assert_eq!(before, after);
    }

    #[test]
    fn split_single_test() {
        let (_, mut before) = parsers::number("10").unwrap();
        let (_, after) = parsers::number("[5,5]").unwrap();
        assert!(before.split());
        assert_eq!(before, after);
    }

    #[test]
    fn split_uneven_test() {
        let (_, mut before) = parsers::number("11").unwrap();
        let (_, after) = parsers::number("[5,6]").unwrap();
        assert!(before.split());
        assert_eq!(before, after);
    }

    #[test]
    fn addition_steps_test() {
        let (_, a) = parsers::number("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let (_, b) = parsers::number("[1,1]").unwrap();
        let mut c = a.join(b);
        let (_, res) = parsers::number("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        assert_eq!(c, res);
        c.explode();
        let (_, res) = parsers::number("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        assert_eq!(c, res);
        c.explode();
        let (_, res) = parsers::number("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
        assert_eq!(c, res);
        c.split();
        let (_, res) = parsers::number("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();
        assert_eq!(c, res);
        c.split();
        let (_, res) = parsers::number("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap();
        assert_eq!(c, res);
        c.explode();
        let (_, res) = parsers::number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(c, res);
    }

    #[test]
    fn addition_test() {
        let (_, a) = parsers::number("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let (_, b) = parsers::number("[1,1]").unwrap();
        let c = a + b;
        let (_, result) = parsers::number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(c, result);
    }
}
