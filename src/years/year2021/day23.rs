use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Clone, Eq, PartialEq, Hash)]
struct House {
    rooms: Rooms,
    corridor: [Space; 11],
}

impl House {
    fn possibilities(&self) -> Vec<Movement> {
        let to_room = self
            .corridor
            .iter()
            .enumerate()
            .filter_map(|(corridor_idx, space)| {
                space.and_then(|amphipod| {
                    let room_idx = amphipod as usize;
                    let room_corridor_idx = room_to_corridor_idx(room_idx);
                    if self.corridor[(corridor_idx + 1).min(room_corridor_idx)
                        ..=(corridor_idx.saturating_sub(1)).max(room_corridor_idx)]
                        .iter()
                        .any(Option::is_some)
                    {
                        return None;
                    }
                    let room = &self.rooms[room_idx];
                    if is_sorted_bottom(room_idx, room) {
                        room.iter().enumerate().rev().find_map(|(i, a)| {
                            a.is_none().then(|| Movement::ToRoom {
                                source: corridor_idx,
                                destination: (room_idx, i),
                            })
                        })
                    } else {
                        None
                    }
                })
            });
        let to_corridor = self
            .rooms
            .iter()
            .enumerate()
            .filter_map(|(room_idx, r)| {
                if is_sorted_bottom(room_idx, r) {
                    return None;
                }
                r.iter()
                    .enumerate()
                    .find_map(|(room_sub_idx, s)| s.map(|_| (room_idx, room_sub_idx)))
            })
            .flat_map(|source @ (room_idx, _)| {
                let i_corridor = room_to_corridor_idx(room_idx);
                let mut movements = Vec::new();
                for corridor_idx in (0..i_corridor).rev() {
                    if self.corridor[corridor_idx].is_some() {
                        break;
                    }
                    if !is_infront_room(corridor_idx) {
                        movements.push(Movement::ToCorridor {
                            source,
                            destination: corridor_idx,
                        })
                    }
                }
                for corridor_idx in i_corridor + 1..self.corridor.len() {
                    if self.corridor[corridor_idx].is_some() {
                        break;
                    }
                    if !is_infront_room(corridor_idx) {
                        movements.push(Movement::ToCorridor {
                            source,
                            destination: corridor_idx,
                        })
                    }
                }
                movements.into_iter()
            });
        to_room.chain(to_corridor).collect()
    }

    fn is_done(&self) -> bool {
        self.corridor.iter().all(Option::is_none)
            && self.rooms.iter().enumerate().all(|(i, room)| {
                room.iter().all(|space| {
                    if let Some(a) = space {
                        *a as usize == i
                    } else {
                        false
                    }
                })
            })
    }
    fn apply(&self, m: Movement) -> House {
        match m {
            Movement::ToCorridor {
                source,
                destination,
            } => {
                let mut new = self.clone();
                new[destination] = new[source].take();
                new
            }
            Movement::ToRoom {
                destination,
                source,
            } => {
                let mut new = self.clone();
                new[destination] = new[source].take();
                new
            }
        }
    }
}

impl Index<usize> for House {
    type Output = Option<Ambipod>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.corridor[index]
    }
}
impl IndexMut<usize> for House {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.corridor[index]
    }
}
impl Index<(usize, usize)> for House {
    type Output = Option<Ambipod>;

    fn index(&self, (room_index, room_sub_index): (usize, usize)) -> &Self::Output {
        &self.rooms[room_index][room_sub_index]
    }
}
impl IndexMut<(usize, usize)> for House {
    fn index_mut(&mut self, (room_index, room_sub_index): (usize, usize)) -> &mut Self::Output {
        &mut self.rooms[room_index][room_sub_index]
    }
}

#[cached::proc_macro::cached]
fn minimal_energy_to_sort(house: House) -> Option<usize> {
    if house.is_done() {
        return Some(0);
    }
    house
        .possibilities()
        .into_iter()
        .filter_map(|m| {
            minimal_energy_to_sort(house.apply(m)).map(|e| {
                (match m {
                    Movement::ToCorridor { source, .. } => house[source],
                    Movement::ToRoom { source, .. } => house[source],
                })
                .unwrap()
                .value()
                    * m.distance()
                    + e
            })
        })
        .min()
}

fn is_infront_room(i: usize) -> bool {
    i % 2 == 0 && i >= 2 && i <= 8
}
fn room_to_corridor_idx(i: usize) -> usize {
    2 + i * 2
}

fn is_sorted_bottom(i: usize, room: &[Space]) -> bool {
    for a in room.iter().flatten() {
        if *a as usize != i {
            return false;
        }
    }
    true
}

impl crate::Day for House {
    fn part1(&self) -> String {
        minimal_energy_to_sort(self.clone()).unwrap().to_string()
    }

    fn part2(&self) -> String {
        let mut expanded = self.clone();
        expanded.rooms[0].insert(1, Some(Ambipod::Desert));
        expanded.rooms[0].insert(1, Some(Ambipod::Desert));
        expanded.rooms[1].insert(1, Some(Ambipod::Bronze));
        expanded.rooms[1].insert(1, Some(Ambipod::Copper));
        expanded.rooms[2].insert(1, Some(Ambipod::Amber));
        expanded.rooms[2].insert(1, Some(Ambipod::Bronze));
        expanded.rooms[3].insert(1, Some(Ambipod::Copper));
        expanded.rooms[3].insert(1, Some(Ambipod::Amber));
        minimal_energy_to_sort(expanded).unwrap().to_string()
    }
}

#[derive(Copy, Clone, Debug)]
enum Movement {
    ToCorridor {
        source: (usize, usize),
        destination: usize,
    },
    ToRoom {
        destination: (usize, usize),
        source: usize,
    },
}

impl Movement {
    fn distance(&self) -> usize {
        match *self {
            Movement::ToCorridor {
                source: (room_idx, room_sub_idx),
                destination: corridor_idx,
            }
            | Movement::ToRoom {
                source: corridor_idx,
                destination: (room_idx, room_sub_idx),
            } => {
                (corridor_idx as i64 - room_to_corridor_idx(room_idx) as i64).abs() as usize
                    + room_sub_idx
                    + 1
            }
        }
    }
}

type Rooms = [Vec<Space>; 4];
type Space = Option<Ambipod>;

use Ambipod::{Amber, Bronze, Copper, Desert};
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Ambipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

impl Ambipod {
    fn value(&self) -> usize {
        10usize.pow(usize::from(self) as u32)
    }
}

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let house: House = input.parse().expect("parse input");
        Box::new(house)
    }
}

// conversions
impl FromStr for House {
    fn from_str(input: &str) -> anyhow::Result<Self> {
        let mut rooms = array_init::array_init(|_| vec![None; 2]);
        let corridor = [None; 11];
        let mut room = 0;
        let mut space = 0;
        input.lines().for_each(|l| {
            l.chars().for_each(|c| {
                if let Ok(a) = Ambipod::try_from(c) {
                    rooms[room][space] = a.into();
                    room += 1;
                    if room >= rooms.len() {
                        room = 0;
                        space += 1;
                    }
                }
            })
        });

        Ok(Self { corridor, rooms })
    }

    type Err = anyhow::Error;
}

impl From<&Ambipod> for Space {
    fn from(a: &Ambipod) -> Self {
        From::from(*a)
    }
}

impl From<Ambipod> for usize {
    fn from(a: Ambipod) -> Self {
        a as usize
    }
}

impl From<&Ambipod> for usize {
    fn from(a: &Ambipod) -> Self {
        From::from(*a)
    }
}

impl TryFrom<usize> for Ambipod {
    type Error = anyhow::Error;
    fn try_from(u: usize) -> anyhow::Result<Self> {
        Ok(match u {
            0 => Amber,
            1 => Bronze,
            2 => Copper,
            3 => Desert,
            _ => return Err(anyhow::anyhow!("invalid value {}", u)),
        })
    }
}

impl TryFrom<&usize> for Ambipod {
    type Error = anyhow::Error;

    fn try_from(value: &usize) -> Result<Self, Self::Error> {
        TryFrom::try_from(*value)
    }
}

impl From<Ambipod> for char {
    fn from(a: Ambipod) -> Self {
        match a {
            Amber => 'A',
            Bronze => 'B',
            Copper => 'C',
            Desert => 'D',
        }
    }
}

impl From<&Ambipod> for char {
    fn from(a: &Ambipod) -> Self {
        From::from(*a)
    }
}

impl TryFrom<char> for Ambipod {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'A' => Amber,
            'B' => Bronze,
            'C' => Copper,
            'D' => Desert,
            _ => return Err(anyhow::anyhow!("{} is not a known ambipod type", c)),
        })
    }
}

impl TryFrom<&char> for Ambipod {
    type Error = anyhow::Error;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        TryFrom::try_from(*c)
    }
}

// debug / display
impl std::fmt::Debug for Ambipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        f.write_fmt(format_args!("{}", c))
    }
}

impl std::fmt::Display for Ambipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl std::fmt::Display for House {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.corridor.len() + 2;
        f.write_str("\n")?;
        f.write_fmt(format_args!("{0:█^1$}\n", "", len))?;
        f.write_fmt(format_args!(
            "{0:█^1$}\n",
            self.corridor
                .iter()
                .copied()
                .map(|s| s.map_or(' ', char::from))
                .collect::<String>(),
            len
        ))?;
        for i in 0..self.rooms[0].len() {
            let rooms = self
                .rooms
                .iter()
                .cloned()
                .map(|r| r[i].map_or(' ', char::from))
                .intersperse('█')
                .collect::<String>();
            f.write_fmt(format_args!("{0:█^1$}\n", rooms, len))?;
        }
        f.write_fmt(format_args!("{0:█^1$}\n", "", len))
    }
}

impl std::fmt::Debug for House {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
