use std::collections::HashMap;

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

fn run(
    memo: &mut HashMap<(u64, u64, u64, u64, bool, [u64; 3]), (u64, u64)>,
    p1: u64,
    p2: u64,
    points1: u64,
    points2: u64,
    p1turn: bool,
    rolled: &mut Vec<u64>,
) -> (u64, u64) {
    let mut rolled_arr = [0; 3];
    rolled_arr[0] = rolled.iter().filter(|x| **x == 1).count() as u64;
    rolled_arr[1] = rolled.iter().filter(|x| **x == 2).count() as u64;
    rolled_arr[2] = rolled.iter().filter(|x| **x == 3).count() as u64;
    let key = (p1, p2, points1, points2, p1turn, rolled_arr);
    if let Some(res) = memo.get(&key) {
        return *res;
    }
    if points1 >= 21 {
        memo.insert(key, (1, 0));
        return (1, 0);
    }
    if points2 >= 21 {
        memo.insert(key, (0, 1));
        return (0, 1);
    }
    if rolled.len() < 3 {
        rolled.push(1);
        let a = run(memo, p1, p2, points1, points2, p1turn, rolled);
        rolled.pop();
        rolled.push(2);
        let b = run(memo, p1, p2, points1, points2, p1turn, rolled);
        rolled.pop();
        rolled.push(3);
        let c = run(memo, p1, p2, points1, points2, p1turn, rolled);
        rolled.pop();
        let res = (a.0 + b.0 + c.0, a.1 + b.1 + c.1);
        memo.insert(key, res);
        return res;
    }
    let rolled = rolled.iter().sum::<u64>();
    let res = if p1turn {
        let p1 = (p1 + rolled) % 10;
        run(
            memo,
            p1,
            p2,
            points1 + p1 + 1,
            points2,
            !p1turn,
            &mut Vec::new(),
        )
    } else {
        let p2 = (p2 + rolled) % 10;
        run(
            memo,
            p1,
            p2,
            points1,
            points2 + p2 + 1,
            !p1turn,
            &mut Vec::new(),
        )
    };
    memo.insert(key, res);
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
        let mut memo = HashMap::new();
        let (p1wins, p2wins) = run(&mut memo, p1, p2, 0, 0, true, &mut Vec::new());
        p1wins.max(p2wins).to_string()
    }
}
