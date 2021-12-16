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
            if let Data::SubPackets(mut packets) = pack.data {
                to_visit.append(&mut packets)
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
    type_id: TypeId,
    data: Data,
}
impl Packet {
    fn value(&self) -> u64 {
        match (&self.type_id, &self.data) {
            (_, Data::Literal(v)) => *v,
            (TypeId::Sum, Data::SubPackets(others)) => others.iter().map(|p| p.value()).sum(),
            (TypeId::Product, Data::SubPackets(others)) => others.iter().map(|p| p.value()).product(),
            (TypeId::Min, Data::SubPackets(others)) => others.iter().map(|p| p.value()).min().unwrap_or(u64::MAX),
            (TypeId::Max, Data::SubPackets(others)) => others.iter().map(|p| p.value()).max().unwrap_or(0),
            (TypeId::Literal, Data::SubPackets(..)) => unreachable!(),
            (TypeId::Greater, Data::SubPackets(others)) => if others[0].value() > others[1].value() { 1 } else { 0},
            (TypeId::Less, Data::SubPackets(others)) => if others[0].value() < others[1].value() { 1 } else { 0},
            (TypeId::Equal, Data::SubPackets(others)) => if others[0].value() == others[1].value() { 1 } else { 0},
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TypeId {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    Greater,
    Less,
    Equal,
}

impl From<u8> for TypeId {
    fn from(raw_type_id: u8) -> Self {
        match raw_type_id {
            0 => TypeId::Sum,
            1 => TypeId::Product,
            2 => TypeId::Min,
            3 => TypeId::Max,
            4 => TypeId::Literal,
            5 => TypeId::Greater,
            6 => TypeId::Less,
            7 => TypeId::Equal,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Data {
    Literal(u64),
    SubPackets(Vec<Packet>),
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

impl From<u8> for LengthType {
    fn from(raw: u8) -> Self {
        match raw {
            0 => Self::Size,
            1 => Self::Len,
            _ => panic!("invalid length type"),
        }
    }
}

mod parsers {
    use super::{Data, LengthType, Packet, TypeId};
    use nom::{
        bits::{
            bits,
            complete::{tag, take},
        },
        multi::{count, many0},
        sequence::{preceded, tuple},
        IResult,
    };

    fn version(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
        take(3usize)(input)
    }

    fn type_id(input: (&[u8], usize)) -> IResult<(&[u8], usize), TypeId> {
        let (rest, type_id): (_, u8) = take(3usize)(input)?;
        Ok((rest, TypeId::from(type_id)))
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
        let (rest, length_type): (_, u8) = take(1usize)(input)?;
        Ok((rest, LengthType::from(length_type)))
    }

    fn other_data(
        length_type: LengthType,
        length: usize,
    ) -> impl Fn((&[u8], usize)) -> IResult<(&[u8], usize), Data> {
        move |input: (&[u8], usize)| -> IResult<(&[u8], usize), Data> {
            if let LengthType::Size = length_type {
                let (rest, (mut inp, l_inp)): (_, (Vec<u8>, u8)) =
                    tuple((count(take(8usize), length / 8), take(length % 8)))(input)?;
                if length % 8 != 0 {
                    inp.push(l_inp << (8 - length % 8));
                }
                let ((_, _), packets) = many0(packet)((&inp[..], 0)).unwrap();
                Ok((rest, Data::SubPackets(packets)))
            } else {
                let (rest, data) = count(packet, length)(input)?;
                Ok((rest, Data::SubPackets(data)))
            }
        }
    }

    fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        let (input, (version, type_id)) = tuple((version, type_id))(input)?;
        let (rest, data) = match type_id {
            TypeId::Literal => literal(input)?,
            _ => {
                let (input, length_type) = length_type(input)?;
                let (input, length) = take(length_type.bits())(input)?;
                other_data(length_type, length)(input)?
            }
        };
        Ok((rest, Packet { type_id, version, data }))
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
                type_id: TypeId::Literal,
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
                type_id: TypeId::Less,
                data: Data::SubPackets(vec![
                    Packet {
                        version: 6,
                        type_id: TypeId::Literal,
                        data: Data::Literal(10)
                    },
                    Packet {
                        version: 2,
                        type_id: TypeId::Literal,
                        data: Data::Literal(20)
                    },
                ])
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
                type_id: TypeId::Max,
                data: Data::SubPackets(vec![
                    Packet {
                        version: 2,
                        type_id: TypeId::Literal,
                        data: Data::Literal(1)
                    },
                    Packet {
                        version: 4,
                        type_id: TypeId::Literal,
                        data: Data::Literal(2)
                    },
                    Packet {
                        version: 1,
                        type_id: TypeId::Literal,
                        data: Data::Literal(3)
                    },
                ])
            },
            parsers::parse(&day.input[..])
        );
    }
}
