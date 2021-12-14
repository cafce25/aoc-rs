use std::collections::HashMap;

use super::intcode::{Intcode, Machine};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = Intcode;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Self {
        use Direction::*;
        match (turn, self) {
            (Turn::Left, Up) => Left,
            (Turn::Left, Left) => Down,
            (Turn::Left, Down) => Right,
            (Turn::Left, Right) => Up,
            (Turn::Right, Up) => Right,
            (Turn::Right, Right) => Down,
            (Turn::Right, Down) => Left,
            (Turn::Right, Left) => Up,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
struct RobotAction {
    paint: Color,
    turn: Turn,
}

struct Robot {
    brain: Machine,
}

impl Robot {
    fn input(&mut self, last_color: Color) {
        match last_color {
            Color::Black => self.brain.input(0),
            Color::White => self.brain.input(1),
        }
    }
}

impl From<Machine> for Robot {
    fn from(brain: Machine) -> Self {
        Self { brain }
    }
}

impl Iterator for Robot {
    type Item = RobotAction;

    fn next(&mut self) -> Option<Self::Item> {
        let paint = match self.brain.next()? {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!("Invalid Color"),
        };
        let turn = match self.brain.next()? {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => unreachable!("Invalid Turn"),
        };
        Some(RobotAction { paint, turn })
    }
}

struct Simulator {
    robot: Robot,
    pos: (i32, i32),
    robot_facing: Direction,
    hull: HashMap<(i32, i32), Vec<Color>>,
}

impl Simulator {
    fn painted(&self) -> usize {
        self.hull.len()
    }

    fn run(&mut self) {
        loop {
            let last_color = self
                .hull
                .get(&self.pos)
                .and_then(|c| c.get(c.len() - 1))
                .copied()
                .unwrap_or(Color::Black);
            self.robot.input(last_color);
            match self.robot.next() {
                Some(RobotAction { paint, turn }) => {
                    self.hull
                        .entry(self.pos)
                        .or_insert_with(Vec::new)
                        .push(paint);
                    self.robot_facing = self.robot_facing.turn(turn);
                    let (x, y) = self.pos;
                    self.pos = match self.robot_facing {
                        Direction::Up => (x, y - 1),
                        Direction::Right => (x + 1, y),
                        Direction::Down => (x, y + 1),
                        Direction::Left => (x - 1, y),
                    };
                }
                None => break,
            }
        }
    }

    fn start(&mut self, color: Color) {
        self.hull.insert(self.pos, vec![color]);
    }
}

impl From<&Input> for Simulator {
    fn from(input: &Input) -> Self {
        Self {
            robot: Robot::from(Machine::from(input.clone())),
            pos: (0, 0),
            robot_facing: Direction::Up,
            hull: HashMap::new(),
        }
    }
}

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut sim = Simulator::from(&self.input);
        sim.run();
        sim.painted().to_string()
    }

    fn part2(&self) -> String {
        let mut sim = Simulator::from(&self.input);
        sim.start(Color::White);
        sim.run();
        let final_paint: HashMap<_, _> = sim
            .hull
            .iter()
            .filter_map(|(c, colors)| Some((c, *colors.last()?)))
            .collect();
        let mut paint_string = String::with_capacity(2 * 6 * 42);
        for y in 0..6 {
            paint_string.push('\n');
            for x in 0..42 {
                match final_paint.get(&(x, y)).copied().unwrap_or(Color::Black) {
                    Color::White => paint_string.push('█'),
                    Color::Black => paint_string.push(' '),
                }
            }
        }
        paint_string
    }
}
