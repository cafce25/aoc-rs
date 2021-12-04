pub struct DayGen;

impl crate::DayGen for DayGen {
    fn input(&self, input: &str) -> Box<dyn crate::Day> {
        let (numbers, boards) = input.split_once("\n\n").unwrap();
        let numbers = numbers.split(',').map(|n| n.parse().unwrap()).collect();
        let boards = boards
            .split("\n\n")
            .filter(|s| !s.is_empty())
            .map(|board_str| Board::new(board_str))
            .collect();
        Box::new(Day::new((numbers, boards)))
    }
}

#[derive(Clone, Debug)]
struct Board {
    numbers: [u64; 25],
    marked: [bool; 25],
}

impl Board {
    fn new(s: &str) -> Self {
        let numbers: Vec<_> = s
            .split('\n')
            .flat_map(|l| l.split(' '))
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        Board {
            numbers: numbers.try_into().unwrap(),
            marked: [false; 25],
        }
    }
    fn mark(&mut self, called: u64) -> Option<u64> {
        self.numbers
            .iter()
            .zip(self.marked.iter_mut())
            .for_each(|(n, m)| {
                if *n == called {
                    *m = true
                }
            });
        self.win()
    }
    fn win(&self) -> Option<u64> {
        let won = self.marked.chunks(5).any(|c| c.iter().all(|m| *m))
            || (0..5).any(|i| self.marked.iter().skip(i).step_by(5).all(|m| *m));
        if won {
            Some(
                self.numbers
                    .into_iter()
                    .zip(self.marked)
                    .filter_map(|(n, m)| if m { None } else { Some(n) })
                    .sum(),
            )
        } else {
            None
        }
    }
}

type Input = (Vec<u64>, Vec<Board>);

struct Day {
    input: Input,
}

impl Day {
    pub fn new(input: Input) -> Self {
        Self { input }
    }
}

impl crate::Day for Day {
    fn part1(&self) -> String {
        let (numbers, mut boards) = (&self.input.0, self.input.1.clone());
        let mut win = None;
        let mut win_num = 0;
        for number in numbers {
            boards.iter_mut().for_each(|b| {
                if let Some(num) = b.mark(*number) {
                    win = Some(num);
                }
            });
            if win.is_some() {
                win_num = *number;
                break;
            }
        }
        match win {
            Some(val) => format!("Won with {} points", val * win_num),
            None => String::from("We didn't win ?!?"),
        }
    }

    fn part2(&self) -> String {
        let (numbers, boards) = (&self.input.0, self.input.1.clone());
        let (boards, num) = numbers.iter().fold((boards, 0), |(mut boards, winner), num| {
            if boards.len() == 1 {
                if boards[0].win().is_none() {
                    boards[0].mark(*num);
                    (boards, *num)
                } else {
                    (boards, winner)
                }
            } else {
                (
                    boards
                        .into_iter()
                        .map(|mut b| {
                            b.mark(*num);
                            b
                        })
                        .filter(|b| !b.win().is_some())
                        .collect(),
                    winner,
                )
            }
        });
        if let Some(val) = boards[0].win() {
            format!("Last winner has {} points", val * num)
        } else {
            String::from("No last winner")
        }
    }
}
