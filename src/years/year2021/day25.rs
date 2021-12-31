use std::ops::{Index, IndexMut};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(SeaFloor::from_str(input))
    }
}

#[derive(Clone)]
struct SeaFloor {
    width: usize,
    area: Vec<Tile>,
}

impl SeaFloor {
    pub fn from_str(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let area = input.lines().flat_map(str::chars).map(Tile::from).collect();
        Self { width, area }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.area.len() / self.width()
    }

    fn indices(&self) -> impl Iterator<Item = (usize, usize)> {
        itertools::iproduct!(0..self.width(), 0..self.height())
    }

    fn step(&mut self) -> bool {
        let east = self.step_east();
        let south = self.step_south();
        east || south
    }

    fn step_east(&mut self) -> bool {
        let mut moved = false;
        let before = self.clone();
        for p @ (x, y) in self.indices() {
            let p1 = ((x + 1) % self.width, y);
            if Tile::East == before[p] && Tile::Empty == before[p1] {
                self[p] = Tile::Empty;
                self[p1] = Tile::East;
                moved = true;
            }
        }
        moved
    }

    fn step_south(&mut self) -> bool {
        let mut moved = false;
        let before = self.clone();
        for p @ (x, y) in self.indices() {
            let p1 = (x, (y + 1) % self.height());
            if Tile::South == before[p] && Tile::Empty == before[p1] {
                self[p] = Tile::Empty;
                self[p1] = Tile::South;
                moved = true;
            }
        }
        moved
    }
}

impl Index<(usize, usize)> for SeaFloor {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.area[x + y * self.width()]
    }
}

impl IndexMut<(usize, usize)> for SeaFloor {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let width = self.width();
        &mut self.area[x + y * width]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    East,
    South,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '>' => Tile::East,
            'v' | 'V' => Tile::South,
            _ => Tile::Empty,
        }
    }
}

impl crate::Day for SeaFloor {
    fn part1(&self) -> String {
        let mut floor = self.clone();
        let mut i = 0;
        while floor.step() {
            i += 1
        }

        (i + 1).to_string()
    }

    fn part2(&self) -> String {
        "Nothing to be done!".to_string()
    }
}

impl std::fmt::Debug for SeaFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;
        for y in 0..self.height() {
            for x in 0..self.width() {
                f.write_str(match self[(x, y)] {
                    Tile::Empty => ".",
                    Tile::East => ">",
                    Tile::South => "v",
                })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
