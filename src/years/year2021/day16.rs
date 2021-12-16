use itertools::Itertools as _;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Vec<u8>;

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .chars()
            .chunks(2)
            .into_iter()
            .map(|c| {
                c.into_iter()
                    .fold(0u8, |acc, c| acc << 4 | c.to_digit(16).unwrap() as u8)
            })
            .collect();
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let p = parsers::parse(&self.input);
        let mut sum = 0;
        let mut to_visit = vec![p];
        while let Some(pack) = to_visit.pop() {
            sum += pack.version as u64;
            match pack.data {
                Data::Sum(mut packets)
                | Data::Product(mut packets)
                | Data::Min(mut packets)
                | Data::Max(mut packets) => to_visit.append(&mut packets),
                Data::Literal(..) => {}
                Data::Greater(a, b) | Data::Less(a, b) | Data::Equal(a, b) => {
                    to_visit.push(*a);
                    to_visit.push(*b);
                }
            }
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        let p = parsers::parse(&self.input);
        p.value().to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Packet {
    version: u8,
    data: Data,
}
impl Packet {
    fn value(&self) -> u64 {
        match &self.data {
            Data::Literal(v) => *v,
            Data::Sum(others) => others.iter().map(|p| p.value()).sum(),
            Data::Product(others) => others.iter().map(|p| p.value()).product(),
            Data::Min(others) => others.iter().map(|p| p.value()).min().unwrap_or(u64::MAX),
            Data::Max(others) => others.iter().map(|p| p.value()).max().unwrap_or(0),
            Data::Greater(a, b) => (a.value() > b.value()) as u64,
            Data::Less(a, b) => (a.value() < b.value()) as u64,
            Data::Equal(a, b) => (a.value() == b.value()) as u64,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Data {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Literal(u64),
    Greater(Box<Packet>, Box<Packet>),
    Less(Box<Packet>, Box<Packet>),
    Equal(Box<Packet>, Box<Packet>),
}

#[derive(Debug)]
enum LengthType {
    Len,
    Size,
}

impl LengthType {
    fn bits(&self) -> usize {
        match self {
            LengthType::Len => 11,
            LengthType::Size => 15,
        }
    }
}

impl TryFrom<u8> for LengthType {
    type Error = String;
    fn try_from(raw: u8) -> Result<Self, Self::Error> {
        match raw {
            0 => Ok(Self::Size),
            1 => Ok(Self::Len),
            _ => Err(format!("invalid length type {}", raw)),
        }
    }
}

mod parsers {
    use super::{Data, LengthType, Packet};
    use nom::{
        bits::{
            bits,
            complete::{tag, take},
        },
        combinator::map_res,
        error::ErrorKind,
        multi::{count, many0},
        sequence::{preceded, tuple},
        IResult,
    };

    fn version(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
        take(3usize)(input)
    }

    fn type_id(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
        take(3usize)(input)
    }

    fn final_nibble(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
        preceded(tag(0, 1usize), take(4usize))(input)
    }

    fn continuation_nibble(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
        preceded(tag(1, 1usize), take(4usize))(input)
    }

    fn literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), Data> {
        let (rest, (mut nibbles, final_nibble)): (_, (Vec<u8>, u8)) =
            tuple((many0(continuation_nibble), final_nibble))(input)?;
        nibbles.push(final_nibble);
        let num = nibbles.into_iter().fold(0u64, |acc, n| acc << 4 | n as u64);
        Ok((rest, Data::Literal(num)))
    }

    fn length_type(input: (&[u8], usize)) -> IResult<(&[u8], usize), LengthType> {
        map_res(take(1usize), |length_type: u8| length_type.try_into())(input)
    }

    fn data(type_id: u8) -> impl Fn((&[u8], usize)) -> IResult<(&[u8], usize), Data> {
        move |input: (&[u8], usize)| -> IResult<(&[u8], usize), Data> {
            if let 4 = type_id {
                literal(input)
            } else {
                let (input, length_type) = length_type(input)?;
                let (input, length) = take(length_type.bits())(input)?;
                let (input, data) = if let LengthType::Size = length_type {
                    let (rest, (mut inp, l_inp)): (_, (Vec<u8>, u8)) =
                        tuple((count(take(8usize), length / 8), take(length % 8)))(input)?;
                    if length % 8 != 0 {
                        inp.push(l_inp << (8 - length % 8));
                    }
                    let (_, packets) = many0(packet)((&inp[..], 0)).unwrap();
                    (rest, packets)
                } else {
                    count(packet, length)(input)?
                };
                Ok((
                    input,
                    match type_id {
                        0 => Data::Sum(data),
                        1 => Data::Product(data),
                        2 => Data::Min(data),
                        3 => Data::Max(data),
                        5 => {
                            let mut data = data;
                            let b = data
                                .pop()
                                .expect("At least 2 subpackets in greater packets");
                            let a = data
                                .pop()
                                .expect("At least 2 subpackets in greater packets");
                            Data::Greater(Box::new(a), Box::new(b))
                        }
                        6 => {
                            let mut data = data;
                            let b = data
                                .pop()
                                .expect("At least 2 subpackets in greater packets");
                            let a = data
                                .pop()
                                .expect("At least 2 subpackets in greater packets");
                            Data::Less(Box::new(a), Box::new(b))
                        }
                        7 => {
                            let mut data = data;
                            let b = data
                                .pop()
                                .expect("At least 2 subpackets in greater packets");
                            let a = data
                                .pop()
                                .expect("At least 2 subpackets in greater packets");
                            Data::Equal(Box::new(a), Box::new(b))
                        }
                        _ => {
                            return Err(nom::Err::Failure(nom::error::make_error(
                                input,
                                ErrorKind::Alt,
                            )))
                        }
                    },
                ))
            }
        }
    }

    fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        let (input, (version, type_id)) = tuple((version, type_id))(input)?;

        let (rest, data) = data(type_id)(input)?;
        Ok((rest, Packet { version, data }))
    }
    fn parse_input(input: &[u8]) -> IResult<&[u8], Packet> {
        bits(packet)(input)
    }
    pub fn parse(input: &[u8]) -> Packet {
        parse_input(input).unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day as _;

    #[test]
    fn part1_s1_test() {
        let input = "8A004A801A8002F478";
        let day = Day::from_str(input);
        assert_eq!("16", day.part1());
    }

    #[test]
    fn literal_packet_test() {
        let input = "D2FE28";
        let day = Day::from_str(input);
        assert_eq!(
            Packet {
                version: 6,
                data: Data::Literal(2021)
            },
            parsers::parse(&day.input[..])
        );
    }

    #[test]
    fn operator_packet_01_test() {
        let input = "38006F45291200";
        let day = Day::from_str(input);
        assert_eq!(
            Packet {
                version: 1,
                data: Data::Less(
                    Box::new(Packet {
                        version: 6,
                        data: Data::Literal(10)
                    }),
                    Box::new(Packet {
                        version: 2,
                        data: Data::Literal(20)
                    }),
                )
            },
            parsers::parse(&day.input[..])
        );
    }

    #[test]
    fn operator_packet_02_test() {
        let input = "EE00D40C823060";
        let day = Day::from_str(input);
        assert_eq!(
            Packet {
                version: 7,
                data: Data::Max(vec![
                    Packet {
                        version: 2,
                        data: Data::Literal(1)
                    },
                    Packet {
                        version: 4,
                        data: Data::Literal(2)
                    },
                    Packet {
                        version: 1,
                        data: Data::Literal(3)
                    },
                ])
            },
            parsers::parse(&day.input[..])
        );
    }
}
