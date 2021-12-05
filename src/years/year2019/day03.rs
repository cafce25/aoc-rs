pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let (wire_a, wire_b) = input.split_once('\n').unwrap();
        dbg!(wire_a, wire_b);
        Box::new(Day::new((
            wire_a
                .split(',')
                .filter_map(|dir| dir.trim().parse().ok())
                .collect::<Vec<Direction>>()
                .into(),
            wire_b
                .split(',')
                .filter_map(|dir| dir.trim().parse().ok())
                .collect::<Vec<Direction>>()
                .into(),
        )))
    }
}

type Input = (Wire, Wire);

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl std::str::FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(d: &str) -> anyhow::Result<Self> {
        Ok(match &d[0..1] {
            "U" => Self::Up(d[1..].parse()?),
            "D" => Self::Down(d[1..].parse()?),
            "L" => Self::Left(d[1..].parse()?),
            "R" => Self::Right(d[1..].parse()?),
            _ => return Err(anyhow::anyhow!("invalid input")),
        })
    }
}

struct Day {
    input: Input,
}

impl Day {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn intersect(&self, other: &Self) -> Option<Point> {
        if self.vertical() && other.horizontal() {
            let point = Point {
                x: self.a.x,
                y: other.a.y,
            };
            (self.contains(point) && other.contains(point)).then(|| point)
        } else if self.horizontal() && other.vertical() {
            let point = Point {
                x: other.a.x,
                y: self.a.y,
            };
            (self.contains(point) && other.contains(point)).then(|| point)
        } else {
            None
        }
    }

    fn vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn distance(&self) -> i64 {
        if self.vertical() {
            (self.a.y - self.b.y).abs()
        } else {
            (self.a.x - self.b.x).abs()
        }
    }
    fn distance_to(&self, p: Point) -> Option<i64> {
        self.contains(p).then(|| {
            Line {
                a: self.a,
                b: p,
            }.distance()
        })
    }
    fn contains(&self, p: Point) -> bool {
        if self.vertical() {
            p.x == self.a.x
                && ((self.a.y..=self.b.y).contains(&p.y) || (self.b.y..=self.a.y).contains(&p.y))
        } else if self.horizontal() {
            p.y == self.a.y
                && ((self.a.x..=self.b.x).contains(&p.x) || (self.b.x..=self.a.x).contains(&p.x))
        } else {
            false
        }
    }
}

impl From<&[Point]> for Line {
    fn from(points: &[Point]) -> Self {
        Self {
            a: points[0],
            b: points[1],
        }
    }
}

struct Wire {
    lines: Vec<Line>,
}

impl Wire {
    fn crossings(&self, other: &Self) -> Vec<Point> {
        self.lines
            .iter()
            .flat_map(|a| other.lines.iter().filter_map(|b| a.intersect(b)))
            .collect()
    }
    fn distance(&self, point: Point) -> i64 {
        let mut dist = 0;
        for line in &self.lines {
            if let Some(d) = line.distance_to(point) {
                return dist + d;
            } else {
                dist += line.distance()
            }
        }
        unreachable!()
    }
}

impl From<Vec<Direction>> for Wire {
    fn from(v: Vec<Direction>) -> Self {
        let mut pos = Point::new();
        let mut wire: Vec<_> = v
            .iter()
            .map(|dir| {
                let old_pos = pos;
                match dir {
                    Direction::Up(n) => pos.x += n,
                    Direction::Down(n) => pos.x -= n,
                    Direction::Left(n) => pos.y += n,
                    Direction::Right(n) => pos.y -= n,
                };
                old_pos
            })
            .collect();
        wire.push(pos);
        wire.into()
    }
}

impl From<Vec<Point>> for Wire {
    fn from(points: Vec<Point>) -> Self {
        let lines = points.windows(2).map(|l| l.into()).collect();
        Self { lines }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        Wire::crossings(&self.input.0, &self.input.1)
            .iter()
            .filter(|p| p != &&Point::new())
            .map(Point::manhattan)
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(&self) -> String {
        Wire::crossings(&self.input.0, &self.input.1)
            .iter()
            .filter(|p| p != &&Point::new())
            .map(|p| self.input.0.distance(*p) + self.input.1.distance(*p))
            .min()
            .unwrap()
            .to_string()
    }
}
