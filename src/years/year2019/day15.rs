use std::{
    cmp::Ordering,
    collections::{hash_map::Values, BinaryHeap, HashMap},
    fmt,
};

use super::intcode::{Intcode, Machine};
use anyhow::{Error, Result};
pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let day: Day = input.try_into().unwrap();
        Box::new(day)
    }
}

struct Day {
    code: Intcode,
}

impl TryFrom<&str> for Day {
    fn try_from(input: &str) -> Result<Day> {
        Ok(Self {
            code: input.parse()?,
        })
    }

    type Error = Error;
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut simulation: Simulation = self.code.clone().into();
        simulation.explore();
        simulation.distance_to_oxygen().to_string()
    }

    fn part2(&self) -> String {
        let mut simulation: Simulation = self.code.clone().into();
        simulation.explore();
        simulation.fill_with_oxygen().to_string()
    }
}

type Coord = (i64, i64);

struct Plan {
    tiles: HashMap<Coord, Tile>,
    min: Coord,
    max: Coord,
}

impl Plan {
    fn new() -> Self {
        Self {
            tiles: HashMap::from([((0, 0), Tile::Empty)]),
            min: (0, 0),
            max: (0, 0),
        }
    }

    fn path(&self, from: Coord, to: Coord) -> Vec<Coord> {
        let mut visited: HashMap<Coord, Vec<Coord>> = HashMap::new();
        let mut candidates = BinaryHeap::new();
        candidates.push((0, from));
        while !visited.contains_key(&to) {
            if candidates.is_empty() {
                panic!("no more candidates left but still not at target");
            }
            let from = candidates.pop().unwrap().1;
            if visited.contains_key(&from) {
                continue;
            }
            let mut path = self
                .neighbours(from)
                .into_iter()
                .filter_map(|n| visited.get(&n))
                .min_by_key(|p| p.len())
                .map(|x| x.to_vec())
                .unwrap_or_default();
            path.push(from);

            let distance_to_start = path.len() - 1;
            visited.insert(from, path);

            candidates.extend(self.walkable_neighbours(from).map(|coord| {
                (
                    usize::MAX
                        - distance_to_start
                        - ((coord.0 - to.0).abs() + (coord.1 - to.1).abs()) as usize,
                    coord,
                )
            }));
        }

        visited.remove(&to).unwrap()
    }

    fn update_bounds(&mut self, coord: Coord) {
        self.min = (self.min.0.min(coord.0), self.min.1.min(coord.1));
        self.max = (self.max.0.max(coord.0), self.max.1.max(coord.1));
    }
    fn unknown_neighbours<'a>(&'a self, coord: Coord) -> impl Iterator<Item = Coord> + 'a {
        all_neighbours(coord).filter(move |coord| !self.contains_key(coord))
    }

    fn neighbours<'a>(&'a self, coord: Coord) -> impl Iterator<Item = Coord> + 'a {
        all_neighbours(coord).filter(move |coord| self.contains_key(coord))
    }
    fn walkable_neighbours<'a>(&'a self, coord: Coord) -> impl Iterator<Item = Coord> + 'a {
        self.neighbours(coord)
            .filter(move |coord| match self.get(coord) {
                Some(Tile::Empty) | Some(Tile::Oxygen) => true,
                _ => false,
            })
    }

    pub fn get(&self, k: &Coord) -> Option<&Tile> {
        self.tiles.get(k)
    }

    pub fn contains_key(&self, k: &Coord) -> bool {
        self.tiles.contains_key(k)
    }

    pub fn insert(&mut self, k: Coord, v: Tile) -> Option<Tile> {
        self.update_bounds(k);
        self.tiles.insert(k, v)
    }

    fn distance_to_oxygen(&self) -> usize {
        let oxygen = *self.iter().find(|(_, t)| **t == Tile::Oxygen).unwrap().0;
        self.path((0, 0), oxygen).len() - 1
    }

    fn iter(&self) -> std::collections::hash_map::Iter<Coord, Tile> {
        self.tiles.iter()
    }

    pub fn values(&self) -> Values<'_, Coord, Tile> {
        self.tiles.values()
    }
}

fn all_neighbours(coord: Coord) -> impl Iterator<Item = Coord> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(move |(y, x)| (coord.0 + y, coord.1 + x))
}
impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "┌{:─>1$}┐\n",
            "",
            ((self.max.1 - self.min.1).abs() + 1) as usize
        ))?;
        for y in self.min.0..=self.max.0 {
            f.write_str("│")?;
            for x in self.min.1..=self.max.1 {
                f.write_fmt(format_args!(
                    "{}",
                    self.tiles.get(&(y, x)).unwrap_or(&Default::default())
                ))?
            }
            f.write_str("│\n")?;
        }
        f.write_fmt(format_args!(
            "└{:─>1$}┘\n",
            "",
            ((self.max.1 - self.min.1).abs() + 1) as usize
        ))?;
        Ok(())
    }
}
impl fmt::Debug for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Oxygen,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Unknown
    }
}
impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("'")?;
        <Self as fmt::Display>::fmt(self, f)?;
        f.write_str("'")
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Tile::Unknown => "?",
            Tile::Empty => " ",
            Tile::Wall => "█",
            Tile::Oxygen => "O",
        })
    }
}

impl From<i64> for Tile {
    fn from(v: i64) -> Self {
        match v {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Oxygen,
            _ => Tile::Unknown,
        }
    }
}

struct Simulation {
    plan: Plan,
    robot: Machine,
    pos: Coord,
}

impl Simulation {
    fn new(code: Intcode) -> Self {
        Self {
            plan: Plan::new(),
            robot: code.into(),
            pos: (0, 0),
        }
    }

    fn try_move(&mut self, direction: Direction) -> bool {
        self.robot.input(direction.to_command());
        let target = direction.apply(self.pos);
        match Tile::from(self.robot.next().unwrap()) {
            Tile::Unknown => unreachable!(),
            tile @ Tile::Wall => {
                self.plan.insert(target, tile);
                false
            }
            tile @ Tile::Empty | tile @ Tile::Oxygen => {
                self.pos = target;
                self.plan.insert(target, tile);
                true
            }
        }
    }

    fn explore(&mut self) {
        let mut left_unknown: Vec<Coord> = self.plan.unknown_neighbours((0, 0)).collect();
        while let Some(target) = left_unknown.pop() {
            if self.plan.contains_key(&target) {
                continue;
            }
            let target = target;
            let neighbour = self.plan.walkable_neighbours(target).next().unwrap();
            let path = self.plan.path(self.pos, neighbour);
            let directions = self.path_to_directions(&path);
            let mut i = 1;
            for direction in directions {
                if !self.try_move(direction) {
                    panic!("could not move there {:?}", direction);
                }
                if self.pos != path[i] {
                    panic!(
                        "should be here {:?}, am here instead {:?}",
                        path[i], self.pos
                    );
                }
                i += 1;
            }
            self.try_move(Direction::from([neighbour, target]));
            left_unknown.extend(self.plan.unknown_neighbours(self.pos));
        }
    }

    fn fill_with_oxygen(&mut self) -> usize {
        let mut i = 0;
        let mut oxys: Vec<_> = self
            .plan
            .iter()
            .map(|(k, v)| (*k, *v))
            .filter(|(_, t)| *t == Tile::Oxygen)
            .map(|(c, _)| c)
            .collect();

        while self.plan.values().any(|x| *x == Tile::Empty) {
            let mut next_oxys = Vec::new();
            for oxy in &oxys {
                for prev_empty in self
                    .plan
                    .neighbours(*oxy)
                    .filter(|n| self.plan.get(n) == Some(&Tile::Empty)).collect::<Vec<_>>()
                {
                    next_oxys.push(prev_empty);
                    self.plan.insert(prev_empty, Tile::Oxygen);
                }
            }
            oxys = next_oxys;
            i += 1;
        }
        i
    }

    fn path_to_directions(&self, path: &[Coord]) -> Vec<Direction> {
        path.windows(2).map(Direction::from).collect()
    }

    fn distance_to_oxygen(&self) -> usize {
        self.plan.distance_to_oxygen()
    }
}
impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "┌{:─^1$}┐\n",
            format!("{:?}", self.pos),
            ((self.plan.max.1 - self.plan.min.1).abs() + 1) as usize
        ))?;
        for y in (self.plan.min.0..=self.plan.max.0).rev() {
            f.write_str("│")?;
            for x in self.plan.min.1..=self.plan.max.1 {
                f.write_fmt(format_args!(
                    "{}",
                    if self.pos == (y, x) {
                        "X".to_string()
                    } else if (y, x) == (0, 0) {
                        ".".to_string()
                    } else {
                        self.plan
                            .tiles
                            .get(&(y, x))
                            .unwrap_or(&Default::default())
                            .to_string()
                    }
                ))?
            }
            f.write_str("│\n")?;
        }
        f.write_fmt(format_args!(
            "└{:─>1$}┘\n",
            "",
            ((self.plan.max.1 - self.plan.min.1).abs() + 1) as usize
        ))?;
        Ok(())
    }
}
impl fmt::Debug for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl From<Intcode> for Simulation {
    fn from(ic: Intcode) -> Self {
        Self::new(ic)
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn apply(&self, coord: Coord) -> Coord {
        match self {
            Direction::North => (coord.0 + 1, coord.1),
            Direction::East => (coord.0, coord.1 + 1),
            Direction::South => (coord.0 - 1, coord.1),
            Direction::West => (coord.0, coord.1 - 1),
        }
    }
    fn to_command(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::East => 4,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

impl From<[Coord; 2]> for Direction {
    fn from([a, b]: [Coord; 2]) -> Self {
        match (a.0.cmp(&b.0), a.1.cmp(&b.1)) {
            (Ordering::Less, _) => Direction::North,
            (Ordering::Greater, _) => Direction::South,
            (Ordering::Equal, Ordering::Less) => Direction::East,
            (Ordering::Equal, Ordering::Greater) => Direction::West,
            (Ordering::Equal, Ordering::Equal) => unreachable!(),
        }
    }
}
impl From<&[Coord]> for Direction {
    fn from(pair: &[Coord]) -> Self {
        Self::from([pair[0], pair[1]])
    }
}
