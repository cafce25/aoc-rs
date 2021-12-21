use cached::proc_macro::cached;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

type Input = (u64, u64);

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(_input: &str) -> Self {
        Self { input: (7, 10) }
    }
}

#[cached]
fn run(
    active_pos: u64,
    next_pos: u64,
    active_points: u64,
    next_points: u64,
    rolled: [u8; 3],
) -> (u64, u64) {
    let mut rolled = rolled;
    if active_points >= 21 {
        return (1, 0);
    }
    if next_points >= 21 {
        return (0, 1);
    }
    if rolled.iter().sum::<u8>() < 3 {
        let mut res = (0, 0);
        for i in 0..=2 {
            rolled[i] += 1;
            let a = run(
                active_pos,
                next_pos,
                active_points,
                next_points,
                rolled,
            );
            rolled[i] -= 1;
            res.0 += a.0;
            res.1 += a.1;
        }
        return res;
    }
    let rolled = (0u64..3).map(|i| (i+1) * rolled[i as usize] as u64).sum::<u64>();
    let active_pos = (active_pos + rolled) % 10;
    let res = run(
        next_pos,
        active_pos,
        next_points,
        active_points + active_pos + 1,
        [0;3]
    );
    let res = (res.1, res.0);
    res
}
impl crate::Day for Day {
    fn part1(&self) -> String {
        let mut die = 1;
        let mut n_rolled = 0;
        let mut p1 = self.input.0 - 1;
        let mut p2 = self.input.1 - 1;
        let mut points1 = 0;
        let mut points2 = 0;
        let mut p1turn = true;
        while points1 < 1000 && points2 < 1000 {
            n_rolled += 3;
            let rolled = match die {
                98 => {
                    die = 1;
                    98 + 99 + 100
                }
                99 => {
                    die = 2;
                    99 + 100 + 1
                }
                100 => {
                    die = 3;
                    100 + 1 + 2
                }
                _ => {
                    die += 3;
                    3 * (die - 2)
                }
            };
            if p1turn {
                p1 += rolled;
                p1 %= 10;
                points1 += p1 + 1;
            } else {
                p2 += rolled;
                p2 %= 10;
                points2 += p2 + 1;
            }
            p1turn = !p1turn;
        }

        if points1 > points2 {
            points2 * n_rolled
        } else {
            points1 * n_rolled
        }
        .to_string()
    }

    fn part2(&self) -> String {
        let p1 = self.input.0 - 1;
        let p2 = self.input.1 - 1;
        let (p1wins, p2wins) = run(p1, p2, 0, 0, [0;3]);
        p1wins.max(p2wins).to_string()
    }
}
