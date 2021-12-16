use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashSet},
    f64::consts::PI,
    ops::{DivAssign, Sub},
};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input<'a>(&self, input: &'a str) -> Box<dyn crate::Day + 'a> {
        Box::new(Day::from_str(input))
    }
}

type Input = HashSet<Point>;

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars().enumerate().filter_map(move |(c, ast)| {
                    (ast == '#').then(|| Point::from((r as i64, c as i64)))
                })
            })
            .collect();
        Self { input }
    }

    fn max_vis_asteroid(&self) -> (Point, usize) {
        self.input
            .iter()
            .copied()
            .map(|point @ Point { y: row, x: col }| {
                let count = self
                    .input
                    .iter()
                    .copied()
                    .filter(|p @ Point { y: r, x: c }| {
                        if row == *r && col == *c {
                            //self
                            return false;
                        }
                        let mut d = *p - point;
                        let gcd = if d.y == 0 {
                            d.x.abs()
                        } else if d.x == 0 {
                            d.y.abs()
                        } else {
                            num::integer::gcd(d.x, d.y)
                        };
                        d /= gcd;
                        for i in 1..gcd {
                            let coords = Point::from((i * d.y + row, i * d.x + col));
                            if self.input.contains(&coords) {
                                return false;
                            }
                        }
                        true
                    })
                    .count();
                (point, count)
            })
            .max_by_key(|(_, v)| *v)
            .unwrap()
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        self.max_vis_asteroid().1.to_string()
    }

    fn part2(&self) -> String {
        let (asteroid, _) = self.max_vis_asteroid();

        let mut by_angle: Vec<Vec<_>> = self
            .input
            .iter()
            .copied()
            .filter(|p| *p != asteroid)
            .fold(BTreeMap::new(), |mut map, coord| {
                let slope = coord - asteroid;
                map.entry(slope)
                    .or_insert(BTreeSet::new())
                    .insert((slope.len(), coord));
                map
            })
            .into_iter()
            .map(|(_, x)| x.into_iter().rev().map(|(_, y)| y).collect())
            .collect();
        let mut i = 0;
        let point = 'search: loop {
            for by_len in &mut by_angle {
                if let Some(point) = by_len.pop() {
                    i += 1;
                    if i == 200 {
                        break 'search point;
                    }
                }
            }
        };

        (point.x * 100 + point.y).to_string()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    y: i64,
    x: i64,
}

impl From<(i64, i64)> for Point {
    fn from((y, x): (i64, i64)) -> Self {
        Self { y, x }
    }
}
impl Point {
    fn angle(&self) -> f64 {
        let angle = (self.x as f64).atan2(-self.y as f64);
        if angle < 0. {
            angle + 2. * PI
        } else {
            angle
        }
    }

    pub(crate) fn len(&self) -> i64 {
        self.x * self.x + self.y * self.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.angle().partial_cmp(&other.angle()).unwrap_or(Ordering::Equal))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl DivAssign<i64> for Point {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day as _;

    #[test]
    fn part1_tiny_test() {
        let input = concat![".#..#\n", ".....\n", "#####\n", "....#\n", "...##"];
        let day = Day::from_str(input);
        assert_eq!(day.part1(), "8");
    }
    #[test]
    fn part2_large_test() {
        let input = concat!(
            ".#..##.###...#######\n",
            "##.############..##.\n",
            ".#.######.########.#\n",
            ".###.#######.####.#.\n",
            "#####.##.#.##.###.##\n",
            "..#####..#.#########\n",
            "####################\n",
            "#.####....###.#.#.##\n",
            "##.#################\n",
            "#####.##.###..####..\n",
            "..######..##.#######\n",
            "####.##.####...##..#\n",
            ".#####..#.######.###\n",
            "##...#.##########...\n",
            "#.##########.#######\n",
            ".####.#.###.###.#.##\n",
            "....##.##.###..#####\n",
            ".#.#.###########.###\n",
            "#.#.#.#####.####.###\n",
            "###.##.####.##.#..##"
        );
        let day = Day::from_str(input);
        assert_eq!(day.part2(), "802");
    }
}
