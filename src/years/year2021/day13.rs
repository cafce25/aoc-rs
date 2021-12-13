use std::collections::HashSet;

pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        Box::new(Day::from_str(input))
    }
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    Up(usize),
    Left(usize),
}
type Dot = (usize, usize);
type Input = (HashSet<Dot>, Vec<Fold>);

struct Day {
    input: Input,
}

impl Day {
    pub fn from_str(input: &str) -> Self {
        let (dots, folds) = input.split_once("\n\n").unwrap();
        let dots = dots
            .lines()
            .filter_map(|d| {
                d.split_once(',')
                    .and_then(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?)))
            })
            .collect();

        let folds = folds
            .lines()
            .filter_map(|f| {
                f["fold along ".len()..]
                    .split_once('=')
                    .and_then(|(xy, v)| {
                        Some(match xy {
                            "y" => Fold::Up(v.parse().ok()?),
                            "x" => Fold::Left(v.parse().ok()?),
                            _ => unreachable!(),
                        })
                    })
            })
            .collect();
        Self {
            input: (dots, folds),
        }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let dots = self.input.0.iter().copied().map(|(x, y)| {
            match self.input.1[0] {
                Fold::Up(fy) if y >= fy => (x, 2*fy - y),
                Fold::Left(fx) if x > fx => (2*fx - x, y),
                _ => (x, y),
            }
        }).collect::<HashSet<_>>();
        dots.len().to_string()
    }

    fn part2(&self) -> String {
        let dots = self.input.1.iter().fold(self.input.0.clone(), |dots, fold| {
            dots.into_iter().map(|(x, y)| {
                match fold.clone() {
                    Fold::Up(fy) if y >= fy => (x, 2*fy - y),
                    Fold::Left(fx) if x > fx => (2*fx - x, y),
                    _ => (x, y),
                }
            }).collect::<HashSet<_>>()
        });

        let mut image = String::from("\n\n");
        image.reserve(dots.len()*6);
        for y in 0..6 {
            for x in 0..40 {
                if dots.contains(&(x, y)) {
                    image.push('#');
                } else {
                    image.push(' ');
                }
            }
            image.push('\n')
        }
        image
    }
}
