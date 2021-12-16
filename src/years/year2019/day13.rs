use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use super::intcode::{Intcode, Machine};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Intcode;

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        Self { input }
    }
}

struct Arcade {
    chip: Machine,
    screen: HashMap<(usize, usize), Tile>,
    points: i64,
}

impl Arcade {
    fn run(&mut self) {
        while let Some(it) = self.next() {
            match it {
                Ok((pos, id)) => {
                    self.screen.insert(pos, id);
                }
                Err(points) => self.points = points,
            }
        }
    }
    fn play(&mut self) -> i64 {
        self.chip.memory[0] = 2;
        let mut paddle_pos = 0;
        while let Some(it) = self.next() {
            match it {
                Ok((pos, id)) => {
                    self.screen.insert(pos, id);
                    if let (Some((paddle, _)), Some((ball, _))) = (
                        self.screen.iter().find(|(_, v)| v.is_paddle()),
                        self.screen.iter().find(|(_, v)| v.is_ball()),
                    ) {
                        match ball.1.cmp(&paddle.1) {
                            Ordering::Less => paddle_pos = -1,
                            Ordering::Equal => paddle_pos = 0,
                            Ordering::Greater => paddle_pos = 1,
                        }
                    }
                    self.chip.set_input(vec![paddle_pos]);
                }
                Err(points) => {
                    self.points = points;
                    if self.screen.values().copied().filter(Tile::is_block).count() == 0 {
                        break;
                    }
                }
            }
        }
        self.points
    }

    #[allow(dead_code)]
    fn print_screen(&self) {
        let (miny, maxy, minx, maxx) = self.screen.keys().fold(
            (usize::MAX, 0, usize::MAX, 0),
            |(miny, maxy, minx, maxx), (y, x)| {
                (miny.min(*y), maxy.max(*y), minx.min(*x), maxx.max(*x))
            },
        );
        for y in miny..maxy {
            for x in minx..maxx {
                print!("{}", self.screen[&(y, x)]);
            }
            println!();
        }
    }
}

impl Iterator for Arcade {
    type Item = Result<((usize, usize), Tile), i64>;

    fn next(&mut self) -> Option<Self::Item> {
        let px = self.chip.next()?;
        let py = usize::try_from(self.chip.next()?).ok()?;
        let id = self.chip.next()?;
        if px < 0 {
            return Some(Err(id));
        }

        let id = Tile::try_from(id).ok()?;
        let px = usize::try_from(px).ok()?;

        Some(Ok(((py, px), id)))
    }
}

impl<I: AsRef<Intcode>> From<I> for Arcade {
    fn from(i: I) -> Self {
        Arcade {
            chip: Machine::from(i.as_ref().clone()),
            screen: HashMap::new(),
            points: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn is_block(&self) -> bool {
        Tile::Block == *self
    }
    fn is_paddle(&self) -> bool {
        Tile::Paddle == *self
    }
    fn is_ball(&self) -> bool {
        Tile::Ball == *self
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Write::write_char(f, match self {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Block => '■',
            Tile::Paddle => '―',
            Tile::Ball => '●',
        })
    }
}

impl TryFrom<i64> for Tile {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => return Err(format!("Invalid tile {}", value)),
        })
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut arcade = Arcade::from(&self.input);
        arcade.run();
        arcade
            .screen
            .into_values()
            .filter(Tile::is_block)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut arcade = Arcade::from(&self.input);
        arcade.play().to_string()
    }
}
