type Input = Vec<Direction>;

#[derive(Debug)]
enum Direction {
    Up(i64),
    Down(i64),
    Forward(i64),
}

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let input: Vec<_> = input
            .split('\n')
            .filter_map(|n| {
                let n = n.split(' ').collect::<Vec<_>>();
                Some(match n[0].chars().next()? {
                    'u' => Direction::Up(n[1].parse::<i64>().ok()?),
                    'd' => Direction::Down(n[1].parse::<i64>().ok()?),
                    'f' => Direction::Forward(n[1].parse::<i64>().ok()?),
                    _ => None?,
                })
            })
            .collect();
        Box::new(Day::new(input))
    }
}

struct Day {
    input: Input,
}

impl Day {
    fn new(input: Input) -> Self {
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut horizontal: i64 = 0;
        let mut vertical: i64 = 0;
        self.input.iter().for_each(|dir| match dir {
            Direction::Up(n) => vertical -= n,
            Direction::Down(n) => vertical += n,
            Direction::Forward(n) => horizontal += n,
        });
        format!("{} x {} = {}", horizontal, vertical, horizontal * vertical)
    }

    fn part2(&self) -> String {
        let mut horizontal = 0;
        let mut vertical = 0;
        let mut aim = 0;
        self.input.iter().for_each(|dir| match dir {
            Direction::Up(n) => aim -= n,
            Direction::Down(n) => aim += n,
            Direction::Forward(n) => {
                horizontal += n;
                vertical += n * aim;
            }
        });
        format!("{} x {} = {}", horizontal, vertical, horizontal * vertical)
    }
}
