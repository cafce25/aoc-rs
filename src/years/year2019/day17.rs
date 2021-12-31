use super::intcode::{Intcode, Machine};

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

struct Day {
    input: Intcode,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let input = input.parse().unwrap();
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let m = Machine::from(&self.input);
        let mut width = 0;
        let board: Vec<_> = m
            .map(|x| x as u8 as char)
            .collect::<String>()
            .lines()
            .flat_map(|l| {
                if width == 0 {
                    width = l.len();
                } else if width != l.len() {
                    // panic!("uneven length {}, {}", width, l.len());
                }
                width = width.max(l.len());
                l.chars().map(|c| match c {
                    '.' => Tile::Space,
                    '#' => Tile::Scaffold,
                    '^' => Tile::Robot(Facing::North),
                    '>' => Tile::Robot(Facing::East),
                    'v' => Tile::Robot(Facing::South),
                    '<' => Tile::Robot(Facing::West),
                    'X' | 'x' => Tile::RobotTumbling,
                    _ => unreachable!(),
                })
            })
            .collect();
        let mut checksum = 0;
        for x in 1..width - 1 {
            for y in 1..board.len() / width - 1 {
                if board[index(width, x, y)].is_solid()
                    && [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .into_iter()
                        .all(|(dx, dy)| {
                            board[index(
                                width,
                                x.saturating_add_signed(dx),
                                y.saturating_add_signed(dy),
                            )]
                            .is_solid()
                        })
                {
                    checksum += x * y;
                }
            }
        }
        checksum.to_string()
    }

    fn part2(&self) -> String {
        let main = "A,B,A,C,B,C,A,C,B,C";
        let a = "L,8,R,10,L,10";
        let b = "R,10,L,8,L,8,L,10";
        let c = "L,4,L,6,L,8,L,8";

        let mut m = Machine::from(&self.input);
        m.memory[0] = 2;
        m.input_iter(
            format!("{}\n{}\n{}\n{}\n{}\n", main, a, b, c, "n")
                .as_bytes()
                .iter()
                .copied()
                .map(i64::from),
        );
        m.last().unwrap().to_string()
    }
}

fn index(width: usize, x: usize, y: usize) -> usize {
    x + y * width
}

#[derive(PartialEq)]
enum Tile {
    Space,
    Scaffold,
    Robot(Facing),
    RobotTumbling,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Tile::Space => ".",
            Tile::Scaffold => "#",
            Tile::Robot(Facing::North) => "^",
            Tile::Robot(Facing::East) => ">",
            Tile::Robot(Facing::South) => "v",
            Tile::Robot(Facing::West) => "<",
            Tile::RobotTumbling => "X",
        })
    }
}

#[derive(PartialEq)]
enum Facing {
    North,
    East,
    South,
    West,
}
impl Tile {
    fn is_solid(&self) -> bool {
        if let Tile::Robot(_) = self {
            return true;
        }
        self == &Tile::Scaffold
    }
}
